use axum::{body::Body, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

pub mod file;
pub mod session;

trait BodyBuilder {
    fn build_body(&self) -> Body;
}

trait ConvertToString {
    fn convert_to_string(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    pub message: String,
}

impl ResponseBody {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl ConvertToString for ResponseBody {
    fn convert_to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl BodyBuilder for ResponseBody {
    fn build_body(&self) -> Body {
        Body::from(self.convert_to_string())
    }
}

// App Error
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}
