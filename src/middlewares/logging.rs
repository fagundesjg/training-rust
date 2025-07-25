use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use tokio::time::Instant;

pub async fn logging(req: Request<Body>, next: Next) -> Response<Body> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;
    let duration = start.elapsed();
    let status = response.status();

    tracing::info!("{method} {uri} -> {status} ({:?})", duration);

    response
}
