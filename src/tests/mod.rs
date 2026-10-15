mod serial_tests {
    use crate::handle::serial::{
        PiShockCommand, PiShockSerialCommandResponse, PiShockSerialHandle,
    };

    #[test]
    fn connect_serial() {
        // This test was failing communication.. I think because of Linux's default behavior of loading the cdc_acm driver..
        let mut handle = PiShockSerialHandle::new("/dev/ttyUSB0").unwrap();
        let res_data = handle.send_command(PiShockCommand::Info).unwrap();

        if let PiShockSerialCommandResponse::Data(res) = res_data {
            println!("RESPONSE ======\n{}\n======", res);
        }
    }

    #[test]
    fn test_add_network_serialize() {
        let json_serd = PiShockCommand::AddNetwork {
            ssid: "rawr".to_string(),
            password: "mjauu".to_string(),
        }
        .get_cmd_string();

        println!("BONK ===\n{}", json_serd);
    }
}
