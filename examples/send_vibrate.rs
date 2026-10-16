use pishock_rawrs::handle::serial::{
    PiShockSerialCommandResponse, PiShockSerialHandle,
    commands::{PiShockCommand, PiShockOperationParams},
};
use serde_json::Value;

fn send_vibrate() {
    let mut handle = PiShockSerialHandle::new("/dev/ttyUSB0").unwrap();
    let op_params = PiShockOperationParams::Vibrate {
        id: 1337,
        duration: 1000,
        intensity: 50,
    };
    let res_data = handle
        .send_command(PiShockCommand::Operate { op: op_params })
        .unwrap();

    if let PiShockSerialCommandResponse::Data(res) = res_data {
        let j: Value = serde_json::from_str(res.as_str()).unwrap();
        println!("{}", serde_json::to_string_pretty(&j).unwrap());
    }
}

fn main() {
    send_vibrate();
}
