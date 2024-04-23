use axum::{debug_handler, extract::Path, http::StatusCode };

#[debug_handler]
pub async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}", sentence_case(&name))
}

#[debug_handler]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

fn sentence_case(input: &str) -> String {
    let mut characters = input.chars();

    let output = characters
        .next()
        .map(|first_character| {
            first_character
                .to_uppercase()
                .chain(characters.flat_map(|character| character.to_lowercase()))
                .collect()
        })
        .unwrap_or_default();

    output
}
