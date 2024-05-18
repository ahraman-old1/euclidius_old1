use axum::{extract::Path, response::Response};

pub async fn show(Path(title): Path<String>) -> Response {
    Response::new(format!("Looking for page {title}").into())
}
