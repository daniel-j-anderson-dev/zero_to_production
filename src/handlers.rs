use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn greet(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    return format!("Hello {}", sentence_case(name));
}

pub async fn health_check(request: HttpRequest) -> impl Responder {
    return HttpResponse::Ok();
}

fn sentence_case(input: &str) -> String {
    let output = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase()
            } else {
                c.to_lowercase()
            }
        })
        .collect();
    return output;
}
