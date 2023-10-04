pub trait SmartHouse {
    fn new(name: String) -> Self;
    fn get_name(&self) -> &str;
    fn push_room(&mut self, name: String) -> Result<(), String>;
    fn overview_rooms(&self) -> &[Room];
    fn get_rooms_list(&mut self) -> &mut [Room];
    fn overview_room(&self, index: usize) -> Option<&Room>;
    fn get_room(&mut self, index: usize) -> Option<&mut Room>;
    fn devices_report(&self) -> String;
}

pub struct House {
    name: String,
    rooms: Vec<Room>,
}

impl SmartHouse for House {
    fn new(name: String) -> Self {
        House {
            name,
            rooms: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn push_room(&mut self, name: String) -> Result<(), String> {
        for r in self.rooms.iter() {
            if r.name == name {
                return Err(String::from("such a room name already exists"));
            }
        }

        let room = Room {
            name,
            devices: Vec::new(),
        };
        self.rooms.push(room);

        Ok(())
    }

    fn overview_rooms(&self) -> &[Room] {
        &self.rooms
    }

    fn get_rooms_list(&mut self) -> &mut [Room] {
        &mut self.rooms
    }

    fn overview_room(&self, index: usize) -> Option<&Room> {
        self.rooms.get(index)
    }

    fn get_room(&mut self, index: usize) -> Option<&mut Room> {
        self.rooms.get_mut(index)
    }

    fn devices_report(&self) -> String {
        let mut res = format!("House name: {}", self.get_name());

        for room in self.overview_rooms().iter() {
            res.push('\n');
            res.push_str(format!("Room name: {}", room.get_name()).as_str());

            if room.list_devices().is_empty() {
                res.push_str("\ndevices not found");
                continue;
            }

            for device in room.list_devices().iter() {
                res.push('\n');
                res.push_str(format!("Device name: {}", device.get_name()).as_str());
            }
        }

        res
    }
}

pub struct Room {
    name: String,
    devices: Vec<Device>,
}

impl Room {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn push_device(&mut self, name: String) -> Result<(), String> {
        for device in self.devices.iter() {
            if device.name == name {
                return Err(String::from("such a device name already exists"));
            }
        }

        let dev = Device {
            name
        };

        self.devices.push(dev);

        Ok(())
    }

    pub fn list_devices(&self) -> &[Device] {
        &self.devices
    }

    pub fn get_device(&self, index: usize) -> Option<&Device> {
        self.devices.get(index)
    }
}

pub struct Device {
    name: String,
}

impl Device {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}