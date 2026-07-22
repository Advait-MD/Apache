use crate::metrics::battery;

pub struct Hud {
    value: u8,
}

impl Hud {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn update(&mut self) {
        if let Some(level) = battery::battery_percentage() {
            self.value = level;
        }
    }

    pub fn text(&self) -> String {
        self.value.to_string()
    }
}
