use axum::{extract::Path, http::{Response, StatusCode, header::CONTENT_TYPE}};

pub async fn assets(Path((content_type, file_name)): Path<(String, String)>) -> Result<Response<String>, StatusCode> {
    let content_type = match content_type.as_str() {
        "css" => "css",
        "js" => "js",
        _ => return Err(StatusCode::NOT_FOUND)
    };

    let file = std::fs::read("./src/assets/".to_string() + content_type + "/" + &file_name);

    if file.is_err() {
        return Err(StatusCode::NOT_FOUND);
    }

    let content_type = match content_type {
        "css" => "text/css",
        "js" => "application/javascript",
        _ => unreachable!()
    };

    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, content_type)
            .body(String::from_utf8(file.unwrap()).unwrap())
            .unwrap()
    )
}