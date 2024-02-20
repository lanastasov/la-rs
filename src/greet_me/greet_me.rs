pub fn greet(name: &str) -> String {
    format!(
        "Hello {}{}!",
        name[..1].to_uppercase(),
        name[1..].to_lowercase()
    )
}
