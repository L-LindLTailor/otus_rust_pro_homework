mod devices;

use devices::smart_socket::SmartSocket;
use devices::thermometer::Thermometer;

fn main() {
    let smart_socket = SmartSocket::new("NKCQK-DNUII-90IY7-H4G69-RJHDP");
    let thermometer = Thermometer::new();

    println!("Device Information: {}", smart_socket.about());
    println!("Current temperature: {}", thermometer.get_temperature());
}
