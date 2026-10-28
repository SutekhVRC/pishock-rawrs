mod serial_tests {

    use crate::handle::serial::commands::{PiShockCommand, PiShockOperationParams};

    #[cfg(feature = "dynamic-tests")]
    use crate::handle::serial::{PiShockSerialCommandResponse, PiShockSerialHandle};
    #[test]
    #[cfg(feature = "dynamic-tests")]
    fn connect_serial() {
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

        let mut assert_str =
            r#"{"cmd":"addnetwork","value":{"ssid":"rawr","password":"mjauu"}}"#.to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
    #[test]
    fn test_remove_network_serialize() {
        let json_serd = PiShockCommand::RemoveNetwork {
            ssid: "RAWR".to_string(),
        }
        .get_cmd_string();

        let mut assert_str = r#"{"cmd":"removenetwork","value":"RAWR"}"#.to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
    #[test]
    fn test_temp_network_connect_serialize() {
        let json_serd = PiShockCommand::TempConnectNetwork {
            ssid: "RAWR".to_string(),
            password: "mrauuu".to_string(),
        }
        .get_cmd_string();

        let mut assert_str =
            r#"{"cmd":"connect","value":{"ssid":"RAWR","password":"mrauuu"}}"#.to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
    #[test]
    fn test_info_serialize() {
        let json_serd = PiShockCommand::Info.get_cmd_string();

        let mut assert_str = r#"{"cmd":"info"}"#.to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
    #[test]
    fn test_restart_serialize() {
        let json_serd = PiShockCommand::Restart.get_cmd_string();

        let mut assert_str = r#"{"cmd":"restart"}"#.to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
    #[test]
    fn test_operate_vibrate_serialize() {
        let op_params = PiShockOperationParams::vibrate(1337, 5000, 90);
        let json_serd = PiShockCommand::Operate { op: op_params }.get_cmd_string();

        let mut assert_str = r#"{"cmd":"operate","value":{"id":1337,"op":"vibrate","duration":5000,"intensity":90}}"#.to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
    #[test]
    fn test_operate_beep_serialize() {
        let op_params = PiShockOperationParams::beep(1337, 5000);
        let json_serd = PiShockCommand::Operate { op: op_params }.get_cmd_string();

        let mut assert_str =
            r#"{"cmd":"operate","value":{"id":1337,"op":"beep","duration":5000}}"#.to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
    #[test]
    fn test_operate_shock_serialize() {
        let op_params = PiShockOperationParams::shock(1337, 5000, 90);
        let json_serd = PiShockCommand::Operate { op: op_params }.get_cmd_string();

        let mut assert_str =
            r#"{"cmd":"operate","value":{"id":1337,"op":"shock","duration":5000,"intensity":90}}"#
                .to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
    #[test]
    fn test_operate_end_serialize() {
        let op_params = PiShockOperationParams::end(1337);
        let json_serd = PiShockCommand::Operate { op: op_params }.get_cmd_string();

        let mut assert_str =
            r#"{"cmd":"operate","value":{"id":1337,"op":"end","duration":0}}"#.to_string();
        assert_str.push_str("\r\n");
        assert_eq!(json_serd, assert_str);
    }
}
