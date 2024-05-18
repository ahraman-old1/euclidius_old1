use euclidius::controllers::page;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = build_router();
    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap()
}

pub fn build_router() -> Router {
    Router::new()
        .route("/wiki/", get(root))
        .route("/wiki/:title", get(page::show))
}

pub async fn root() -> String {
    "Hello wiki!".into()
}
