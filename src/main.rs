use axum::{routing::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> Json<Pong> {
    Json(Pong {
        message: "pong".to_string(),
    })
}

#[derive(Serialize, serde::Deserialize, Debug, PartialEq)]
struct Pong {
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn ping() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let pong: Pong = serde_json::from_slice(&body).unwrap();
        assert_eq!(pong, Pong { message: "pong".to_string() });
    }
}
