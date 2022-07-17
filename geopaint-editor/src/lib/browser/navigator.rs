pub fn is_mobile() -> bool {
    let window = web_sys::window().expect("Missing Window");
    let navigator = window.navigator();
    match navigator.user_agent() {
        Ok(ua) => ua.contains("Mobi"),
        Err(_) => false,
    }
}
