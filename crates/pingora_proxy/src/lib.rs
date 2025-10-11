use pingora::proxy::{ProxyHttp, Session};
use pingora::Result;
use pingora::server::Server;
use pingora::upstreams::peer::{HttpPeer};
use pingora::Error;
use std::sync::Arc;

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
    fn find_route(&self, path: &str) -> Option<&RouteRule> {
        self.routes.iter().find(|route| path.starts_with(&route.path_prefix))
    }
}

#[async_trait::async_trait]
impl ProxyHttp for RoutingProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let path = session.req_header().uri.path();
        
        // Find matching route
        let route = self.find_route(path).ok_or_else(|| {
            Error::because("No matching route found", "Upstream peer selection failed")
        })?;
        
        println!("Routing {} to {}", path, &route.backend);
        
        let peer = Box::new(HttpPeer::new(&route.backend, route.tls, "".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut pingora::http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        // Add custom headers or modify request as needed
        upstream_request.insert_header("Proxy-Agent", "Pingora-Routing-Proxy")?;
        Ok(())
    }
}

/// Proxy server builder
pub struct ProxyBuilder {
    routes: Vec<RouteRule>,
}

impl ProxyBuilder {
    /// Create a new proxy builder
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Add a route rule
    pub fn add_route(mut self, path_prefix: &str, backend: &str, tls: bool) -> Self {
        self.routes.push(RouteRule {
            path_prefix: path_prefix.to_string(),
            backend: backend.to_string(),
            tls,
        });
        self
    }

    /// Build and start the proxy server
    pub fn build_and_start(self) -> Server {
        let mut server = Server::new(None).unwrap();
        server.bootstrap();

        let routing_proxy = Arc::new(RoutingProxy::new(self.routes));
        let mut proxy_service = pingora::proxy::http_proxy_service(&server.configuration, routing_proxy);
        proxy_service.add_tcp("0.0.0.0:6188");

        server.add_service(proxy_service);
        server
    }
}