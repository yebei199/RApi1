pub mod first {
    use pingora::Error;
    use pingora::Result;
    use pingora::proxy::{ProxyHttp, Session};
    use pingora::server::Server;
    use pingora::upstreams::peer::HttpPeer;

    /// Router rule definition
    #[derive(Debug, Clone)]
    pub struct RouteRule {
        /// Path prefix to match
        pub path_prefix: String,
        /// Backend to forward to
        pub backend: String,
        /// Whether to use HTTPS for backend connection
        pub tls: bool,
    }

    /// Proxy service with routing capability
    pub struct RoutingProxy {
        /// List of route rules
        pub routes: Vec<RouteRule>,
    }

    impl RoutingProxy {
        /// Create a new routing proxy with given routes
        pub fn new(routes: Vec<RouteRule>) -> Self {
            Self { routes }
        }

        /// Find a matching route for the given path
        pub fn find_route(
            &self,
            path: &str,
        ) -> Option<&RouteRule> {
            self.routes.iter().find(|route| {
                path.starts_with(&route.path_prefix)
            })
        }
    }

    #[async_trait::async_trait]
    impl ProxyHttp for RoutingProxy {
        type CTX = ();
        fn new_ctx(&self) -> Self::CTX {}

        async fn upstream_peer(
            &self,
            session: &mut Session,
            _ctx: &mut (),
        ) -> Result<Box<HttpPeer>> {
            let path = session.req_header().uri.path();

            // Find matching route
            let route =
                self.find_route(path).ok_or_else(|| {
                    Error::because(
                        pingora::ErrorType::InternalError,
                        "No matching route found",
                        "Upstream peer selection failed",
                    )
                })?;

            println!(
                "Routing {} to {}",
                path, &route.backend
            );

            let peer = Box::new(HttpPeer::new(
                &route.backend,
                route.tls,
                "".to_string(),
            ));
            Ok(peer)
        }

        async fn upstream_request_filter(
            &self,
            _session: &mut Session,
            upstream_request: &mut pingora::http::RequestHeader,
            _ctx: &mut Self::CTX,
        ) -> Result<()> {
            // Add custom headers or modify request as needed
            upstream_request.insert_header(
                "Proxy-Agent",
                "Pingora-Routing-Proxy",
            )?;
            Ok(())
        }
    }

    /// Proxy server builder
    pub struct ProxyBuilder {
        routes: Vec<RouteRule>,
    }

    impl Default for ProxyBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ProxyBuilder {
        /// Create a new proxy builder
        pub fn new() -> Self {
            Self { routes: Vec::new() }
        }

        /// Add a route rule
        pub fn add_route(
            mut self,
            path_prefix: &str,
            backend: &str,
            tls: bool,
        ) -> Self {
            self.routes.push(RouteRule {
                path_prefix: path_prefix.to_string(),
                backend: backend.to_string(),
                tls,
            });
            self
        }

        /// Get routes (for testing)
        #[cfg(test)]
        pub fn routes(&self) -> &[RouteRule] {
            &self.routes
        }

        /// Build the routing proxy
        pub fn build(self) -> RoutingProxy {
            RoutingProxy::new(self.routes)
        }

        /// Build and start the proxy server
        pub fn build_and_start(self) -> Server {
            let mut server = Server::new(None).unwrap();
            server.bootstrap();

            let routing_proxy = self.build();
            let mut proxy_service =
                pingora::proxy::http_proxy_service(
                    &server.configuration,
                    routing_proxy,
                );
            proxy_service.add_tcp("0.0.0.0:6188");

            server.add_service(proxy_service);
            server
        }
    }
}

#[cfg(test)]
mod tests {
    use super::first::*;

    #[test]
    fn test_route_matching() {
        let routes = vec![
            RouteRule {
                path_prefix: "/api/v1".to_string(),
                backend: "127.0.0.1:8080".to_string(),
                tls: false,
            },
            RouteRule {
                path_prefix: "/static".to_string(),
                backend: "127.0.0.1:8081".to_string(),
                tls: false,
            },
        ];

        let proxy = RoutingProxy::new(routes);

        // Test exact match
        let route = proxy.find_route("/api/v1");
        assert!(route.is_some());
        assert_eq!(
            route.unwrap().backend,
            "127.0.0.1:8080"
        );

        // Test prefix match
        let route = proxy.find_route("/api/v1/users");
        assert!(route.is_some());
        assert_eq!(
            route.unwrap().backend,
            "127.0.0.1:8080"
        );

        // Test another prefix match
        let route =
            proxy.find_route("/static/images/logo.png");
        assert!(route.is_some());
        assert_eq!(
            route.unwrap().backend,
            "127.0.0.1:8081"
        );

        // Test no match
        let route = proxy.find_route("/unknown");
        assert!(route.is_none());
    }

    #[test]
    fn test_proxy_builder() {
        let builder = ProxyBuilder::new()
            .add_route("/api", "127.0.0.1:8080", false)
            .add_route("/static", "127.0.0.1:8081", true);

        let routes = builder.routes();
        assert_eq!(routes.len(), 2);
        assert_eq!(routes[0].path_prefix, "/api");
        assert_eq!(routes[0].backend, "127.0.0.1:8080");
        assert_eq!(routes[0].tls, false);
        assert_eq!(routes[1].path_prefix, "/static");
        assert_eq!(routes[1].backend, "127.0.0.1:8081");
        assert_eq!(routes[1].tls, true);
    }

    #[test]
    #[ignore]
    fn integration_test_with_real_backend() {
        // 这个测试展示了如何测试实际的请求转发
        // 但由于需要外部服务，所以默认忽略

        // 示例代码:
        /*
        let builder = ProxyBuilder::new()
            .add_route("/httpbin", "httpbin.org:80", false)
            .add_route("/example", "example.com:80", false);

        let server_handle = std::thread::spawn(|| {
            let server = builder.build_and_start();
            server.run();  // 这会阻塞当前线程
        });

        // 等待服务器启动
        std::thread::sleep(std::time::Duration::from_secs(1));

        // 发送请求到代理
        let client = reqwest::blocking::Client::new();
        let resp = client.get("http://localhost:6188/httpbin/get")
            .send()
            .unwrap();

        assert!(resp.status().is_success());
        */
    }
}
