use smart_home_2::{House, SmartHouse};

fn main() {
    let mut house: House = SmartHouse::new(String::from("house 1"));
    println!("House name: {}", house.get_name());

    house.push_room(String::from("First room")).unwrap();
    house.push_room(String::from("Second room")).unwrap();

    for room in house.overview_rooms().iter() {
        println!("Room name: {}", room.get_name());
    }

    let first_room = match house.get_rooms_list().get_mut(0) {
        Some(r) => r,
        None => panic!("room not found"),
    };
    first_room.push_device(String::from("2ZR5Y-S29XX-OEXDL-VIULN-5XVR4")).unwrap();

    for device in first_room.list_devices() {
        println!(
            "Room name: {} || Device name: {}",
            first_room.get_name(),
            device.get_name()
        );
    }

    let second_room = match house.get_room(1) {
        Some(r) => r,
        None => panic!("room not found"),
    };

    let serial_numbers: [&str; 3] = [
        "2IXEI-FNP0E-FMTPA-KXBPL-DZ665",
        "8YTYJ-SS357-1JTOD-12G03-DKZ8Z",
        "Q8ON7-U6WZH-Z73FD-T8PKH-P1BQH",
    ];

    for id in serial_numbers.iter() {
        second_room.push_device(format!("device {}", id)).unwrap();
    }

    for device in second_room.list_devices().iter() {
        println!(
            "Room name: {}; Device name: {}",
            second_room.get_name(),
            device.get_name()
        );
    }

    second_room.push_device(String::from("final name")).unwrap();
    match second_room.push_device(String::from("final name")) {
        Ok(_) => panic!("negative scenario"),
        Err(err) => println!("received error: {}", err),
    }

    house.push_room(String::from("Third room")).unwrap();

    println!("\n{}", house.devices_report());
}