pub struct SmartSocket {
    description: String,
}

const DEVICE_NAME: &str = "Smart Socket";

impl SmartSocket {
    pub fn new(serial_number: &str) -> Self {
        SmartSocket {
            description: format!(
                "Device: \
                name: {} \n \
                id: {}",
                DEVICE_NAME, serial_number,
            ),
        }
    }

    pub fn _turn_on() {
        todo!()
    }

    pub fn _turn_off() {
        todo!()
    }

    pub fn _get_power_info() {
        todo!()
    }

    pub fn about(&self) -> &str {
        self.description.as_str()
    }
}
