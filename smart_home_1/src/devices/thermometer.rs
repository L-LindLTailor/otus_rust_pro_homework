pub struct Thermometer {
    temperature: f32,
}

impl Thermometer {
    pub fn new() -> Self {
        Thermometer {
            temperature: Self::set_temperature(),
        }
    }

    fn set_temperature() -> f32 {
        todo!()
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

impl Default for Thermometer {
    fn default() -> Self {
        Self::new()
    }
}
