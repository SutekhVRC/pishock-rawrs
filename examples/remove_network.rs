use pishock_rawrs::handle::serial::{
    PiShockSerialCommandResponse, PiShockSerialHandle, commands::PiShockCommand,
};
use serde_json::Value;

fn remove_network() {
    let mut handle = PiShockSerialHandle::new("/dev/ttyUSB0").unwrap();

    let res_data = handle
        .send_command(PiShockCommand::RemoveNetwork {
            ssid: "PiShock".to_string(),
        })
        .unwrap();

    if let PiShockSerialCommandResponse::Data(res) = res_data {
        let j: Value = serde_json::from_str(res.as_str()).unwrap();
        println!("{}", serde_json::to_string_pretty(&j).unwrap());
    }
}

fn main() {
    remove_network();
}
