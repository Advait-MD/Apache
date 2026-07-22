pub struct Hud {
    value: String,
}

impl Hud {
    pub fn new() -> Self {
        Self {
            value: "10".to_string(),
        }
    }

    pub fn text(&self) -> &str {
        &self.value
    }
}
