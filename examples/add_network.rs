use pishock_rawrs::handle::serial::{
    PiShockSerialCommandResponse, PiShockSerialHandle, commands::PiShockCommand,
};
use serde_json::Value;

fn add_network() {
    let mut handle = PiShockSerialHandle::new("/dev/ttyUSB0").unwrap();

    let res_data = handle
        .send_command(PiShockCommand::AddNetwork {
            ssid: "Fakessid".to_string(),
            password: "Fakepwd".to_string(),
        })
        .unwrap();

    if let PiShockSerialCommandResponse::Data(res) = res_data {
        let j: Value = serde_json::from_str(res.as_str()).unwrap();
        println!("{}", serde_json::to_string_pretty(&j).unwrap());
    }
}

fn main() {
    add_network();
}
