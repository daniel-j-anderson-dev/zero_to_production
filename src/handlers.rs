use actix_web::{HttpRequest, Responder};

pub async fn greet(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    return format!("Hello {}", name);
}
