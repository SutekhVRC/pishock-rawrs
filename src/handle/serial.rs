use std::{fmt, time::Duration};

use json::object;
use serialport::{SerialPort, SerialPortInfo, available_ports};

use crate::{
    connect::{
        ConnectTarget::{self},
        PiShockConnect,
        error::ConnectErr,
    },
    handle::error::{SerialHandleErr, SerialSendErr},
};

pub enum PiShockSerialCommandResponse {
    None,
    Data(String),
}

pub enum PiShockOperateCommand {
    Beep {
        id: u32,
        duration: u32,
    },
    Shock {
        id: u32,
        duration: u32,
        intensity: u8,
    },
    Vibrate {
        id: u32,
        duration: u32,
        intensity: u8,
    },
    End {
        id: u32,
        // According to documentation should always be 0
        //duration: u32,
    },
}

pub enum PiShockCommand {
    AddNetwork { ssid: String, password: String },
    RemoveNetwork { ssid: String },
    TempConnectNetwork { ssid: String, password: String },
    Restart,
    Info,
    Operate(PiShockOperateCommand),
}

impl PiShockCommand {
    pub fn get_cmd_string(&self) -> String {
        match self {
            Self::AddNetwork { ssid, password } => self.add_network_serialize(ssid, password),
            Self::Info => "{\"cmd\": \"info\"}\r\n".to_string(),
            _ => todo!(),
        }
    }
    fn add_network_serialize(&self, ssid: &String, password: &String) -> String {
        let mut data = object! {
            "cmd": "addnetwork",
            "value": {
                "ssid": "",
                "password": "",
            }
        };
        data["value"]["ssid"] = json::parse(&Self::json_escape_str(ssid)).unwrap();
        data["value"]["password"] = json::parse(&Self::json_escape_str(password)).unwrap();
        let mut str_data = json::stringify(data);
        str_data.push_str("\r\n");
        str_data
    }

    fn expect_response(&self) -> bool {
        match self {
            Self::Info => true,
            _ => todo!(),
        }
    }

    fn json_escape_str(in_str: &str) -> String {
        format!("\"{}\"", in_str)
    }
}

#[derive(Debug)]
pub struct PiShockSerialHandle {
    device_node: String,
    device_baud: u32,
    device_connection: Box<dyn SerialPort>,
}

impl PiShockSerialHandle {
    pub fn new(serial_vdev: &str) -> Result<Self, SerialHandleErr> {
        if let Ok(serial_ports) = serialport::available_ports() {
            let Ok(()) = Self::check_port(serial_ports, serial_vdev) else {
                return Err(SerialHandleErr("Serial Port Check Failed!"));
            };
        } else {
            return Err(SerialHandleErr("Failed to get available ports!"));
        }

        let device_node = serial_vdev.to_string();
        let device_baud = 115200;
        if let Ok(device_connection) = serialport::new(serial_vdev, device_baud)
            .timeout(Duration::from_secs(10))
            .open()
        {
            println!(
                "Connection Information:\nVdev: {}\nData bits: {}\nStop bits: {}\nBaud rate: {}\nFlow control: {}\nParity: {}",
                device_connection.name().unwrap(),
                device_connection.data_bits().unwrap(),
                device_connection.stop_bits().unwrap(),
                device_connection.baud_rate().unwrap(),
                device_connection.flow_control().unwrap(),
                device_connection.parity().unwrap(),
            );

            let handle = Self {
                device_node,
                device_baud,
                device_connection,
            };
            return Ok(handle);
        };

        return Err(SerialHandleErr("Failed to connect to serial device!"));
    }

    fn check_port(port_list: Vec<SerialPortInfo>, find_port: &str) -> Result<(), SerialHandleErr> {
        if port_list.is_empty() {
            return Err(SerialHandleErr("Ports list was empty!"));
        }

        if let None = port_list.iter().find(|p| p.port_name == find_port) {
            return Err(SerialHandleErr("Specified port was not found!"));
        }

        port_list
            .iter()
            .for_each(|p| println!("Port: [{}]", p.port_name));
        return Ok(());
    }

    fn find_pat(data: &[u8], pat: &[u8]) -> Option<usize> {
        data.windows(pat.len()).position(|w| w == pat)
    }

    pub fn send_command(
        &mut self,
        ps_cmd: PiShockCommand,
    ) -> Result<PiShockSerialCommandResponse, SerialSendErr> {
        let cmd_data = match ps_cmd {
            PiShockCommand::Info => ps_cmd.get_cmd_string(),
            _ => todo!(),
        };
        println!("Sending data: [{}]", cmd_data);
        self.device_connection
            .write_all(cmd_data.as_bytes())
            .unwrap();
        println!("Data written..");
        self.device_connection.flush().unwrap();
        println!("Data flushed..");

        if !ps_cmd.expect_response() {
            return Ok(PiShockSerialCommandResponse::None);
        }

        let mut response = Vec::new();
        println!("Reading..");
        self.device_connection.read_to_end(&mut response).unwrap();
        println!("READ!!");

        let res_offset = Self::find_pat(&response, b"TERMINALINFO: ").unwrap();
        response = response.split_at(res_offset).1.to_vec();

        if response.starts_with("TERMINALINFO: ".as_bytes()) {
            response = response
                .strip_prefix("TERMINALINFO:".as_bytes())
                .unwrap()
                .to_vec();
            let newline_offset = Self::find_pat(&response, b"\n").unwrap();
            response = response.split_at(res_offset).0.to_vec();
        } else {
            return Err(SerialSendErr("Incorrect prefix of response data!"));
        }

        return Ok(PiShockSerialCommandResponse::Data(
            String::from_utf8(response).unwrap(),
        ));
    }
}

impl fmt::Display for PiShockSerialHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PiShockConnect for PiShockSerialHandle {
    type Output = Result<Self, ConnectErr>;
    fn connect(&self, target: ConnectTarget) -> Result<Self, ConnectErr> {
        if let ConnectTarget::Serial(vdev) = target {
            let handler = Self::new(&vdev).unwrap();
            Ok(handler)
        } else {
            todo!()
        }
    }
}
