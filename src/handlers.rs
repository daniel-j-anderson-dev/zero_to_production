use actix_web::{HttpRequest, Responder};

pub async fn root(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    return format!("Hello {}", name);
}
