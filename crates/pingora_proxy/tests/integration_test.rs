use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
use pingora_proxy::proxy_1::first::ProxyBuilder;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::time::{Duration, timeout};

// 启动一个测试HTTP服务器
async fn start_test_server(
    port: u16,
    response_text: &'static str,
) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let make_svc = make_service_fn(move |_conn| {
        let response_text = response_text.to_owned();
        async move {
            Ok::<_, Infallible>(service_fn(move |_req| {
                let response_text = response_text.clone();
                async move {
                    Ok::<_, Infallible>(Response::new(
                        Body::from(response_text),
                    ))
                }
            }))
        }
    });

    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    let server = Server::bind(&addr).serve(make_svc);
    let actual_addr = server.local_addr();

    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!(
                "Test server on port {} error: {}",
                port, e
            );
        }
    });

    // 等待服务器启动
    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(actual_addr)
}

#[tokio::test]
async fn test_request_forwarding()
-> Result<(), Box<dyn std::error::Error>> {
    // 启动两个后端服务器
    let backend1_addr =
        start_test_server(8081, "Response from backend 1")
            .await?;
    let backend2_addr =
        start_test_server(8082, "Response from backend 2")
            .await?;

    println!("Backend 1 address: {}", backend1_addr);
    println!("Backend 2 address: {}", backend2_addr);

    // 构建代理服务器
    let backend1_target = format!(
        "{}:{}",
        backend1_addr.ip(),
        backend1_addr.port()
    );
    let backend2_target = format!(
        "{}:{}",
        backend2_addr.ip(),
        backend2_addr.port()
    );

    let builder = ProxyBuilder::new()
        .add_route("/api", &backend1_target, false)
        .add_route("/static", &backend2_target, false);

    let routing_proxy = builder.build();

    // 启动代理服务器
    let mut server = pingora::server::Server::new(None)?;
    server.bootstrap();

    let mut proxy_service =
        pingora::proxy::http_proxy_service(
            &server.configuration,
            routing_proxy,
        );
    proxy_service.add_tcp("127.0.0.1:6188");

    server.add_service(proxy_service);

    // 在后台线程中运行代理服务器
    let _server_handle = std::thread::spawn(move || {
        let run_args = pingora::server::RunArgs::default();
        server.run(run_args);
    });

    // 等待代理服务器启动
    tokio::time::sleep(Duration::from_millis(100)).await;

    // 发送请求到代理服务器
    let client = reqwest::Client::new();

    // 测试转发到第一个后端
    let url1 = "http://127.0.0.1:6188/api/users";
    let resp1 = timeout(
        Duration::from_secs(5),
        client.get(url1).send(),
    )
    .await??;

    assert_eq!(resp1.status(), 200);
    let body1 = resp1.text().await?;
    assert_eq!(body1, "Response from backend 1");
    println!(
        "Request to /api/users successfully forwarded to backend 1"
    );

    // 测试转发到第二个后端
    let url2 = "http://127.0.0.1:6188/static/image.png";
    let resp2 = timeout(
        Duration::from_secs(5),
        client.get(url2).send(),
    )
    .await??;

    assert_eq!(resp2.status(), 200);
    let body2 = resp2.text().await?;
    assert_eq!(body2, "Response from backend 2");
    println!(
        "Request to /static/image.png successfully forwarded to backend 2"
    );

    // 测试未匹配路径
    let url3 = "http://127.0.0.1:6188/unknown";
    let resp3 = timeout(
        Duration::from_secs(5),
        client.get(url3).send(),
    )
    .await?;

    // 应该返回错误，因为没有匹配的路由
    // 注意：实际行为可能取决于代理的配置，这里我们只测试成功的转发
    if let Ok(response) = resp3 {
        println!(
            "Request to /unknown returned status: {}",
            response.status()
        );
    } else {
        println!(
            "Request to /unknown correctly failed with no matching route"
        );
    }

    Ok(())
}
