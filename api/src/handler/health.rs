use axum::{ extract::State, http::StatusCode };
use registry::AppRegistry;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(registory): State<AppRegistry>) -> StatusCode {
    if registory.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
