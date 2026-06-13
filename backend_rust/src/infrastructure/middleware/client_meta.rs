use axum::{extract::FromRequestParts, http::request::Parts};
use std::convert::Infallible;

/// Metadata permintaan untuk audit trail: IP, user agent, koordinat (opsional).
#[derive(Debug, Clone, Default)]
pub struct ClientMeta {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
}

impl<S> FromRequestParts<S> for ClientMeta
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let h = &parts.headers;
        let get = |k: &str| h.get(k).and_then(|v| v.to_str().ok()).map(|s| s.to_string());

        let ip_address = get("x-forwarded-for")
            .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| get("x-real-ip"))
            .or_else(|| {
                parts
                    .extensions
                    .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                    .map(|ci| ci.0.ip().to_string())
            });

        let user_agent = get("user-agent");
        let lat = get("x-geo-lat").and_then(|s| s.parse::<f64>().ok());
        let long = get("x-geo-long").and_then(|s| s.parse::<f64>().ok());

        Ok(ClientMeta { ip_address, user_agent, lat, long })
    }
}
