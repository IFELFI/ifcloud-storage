use std::collections::HashMap;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use tower_sessions::Session;

use crate::services::{
    session_manager::{SessionManagerService, ValidMethod},
    SessionManager,
};

const PATH_FORM: &str = "/:route_name/:file_key/:file_name";

pub async fn auth_session(request: Request, next: Next) -> Result<Response, StatusCode> {
    let session = request.extensions().get::<Session>().unwrap().clone();
    let path = request.uri().path().to_string();

    let path_params = parse_path(&path, PATH_FORM).await;
    // convert to &str
    let file_key = path_params.get("file_key").unwrap().to_string();

    if let Ok(true) = SessionManager
        .is_available_key(&session, &file_key, ValidMethod::Read)
        .await
    {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn parse_path<'a>(path: &'a str, path_form: &'a str) -> HashMap<&'a str, &'a str> {
    let path_form = path_form
        .trim_start_matches("/") // Remove leading slash
        .split("/") // Split by slash
        .collect::<Vec<&str>>(); // Collect to Vec<&str>
    let path = path
        .trim_start_matches("/")
        .split("/")
        .collect::<Vec<&str>>();

    let mut path_params = HashMap::new();
    for (i, form) in path_form.iter().enumerate() {
        if form.starts_with(":") && i < path.len() {
            path_params.insert(form.trim_start_matches(":"), path[i]);
        }
    }

    path_params
}
