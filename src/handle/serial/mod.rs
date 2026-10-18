use crate::handle::serial::commands::PiShockCommand;
use std::{fmt, path::PathBuf};

use serial2::SerialPort;

pub mod commands;
pub mod model;

use crate::{
    connect::{
        ConnectTarget::{self},
        PiShockConnect,
        error::ConnectErr,
    },
    handle::error::{SerialHandleErr, SerialParseErr, SerialSendErr},
};

use serde_json::Value;

pub enum PiShockSerialCommandResponse {
    None,
    Data(String),
}

impl fmt::Display for PiShockSerialCommandResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Self::Data(res) = self {
            let j: Value = serde_json::from_str(res.as_str()).unwrap();
            write!(f, "{}", serde_json::to_string_pretty(&j).unwrap())
        } else {
            write!(f, "None")
        }
    }
}

#[derive(Debug)]
pub struct PiShockSerialHandle {
    _device_node: String,
    _device_baud: u32,
    device_connection: SerialPort,
}

impl PiShockSerialHandle {
    pub fn new(serial_vdev: &str) -> Result<Self, SerialHandleErr> {
        if let Ok(serial_ports) = SerialPort::available_ports() {
            let Ok(()) = Self::check_port(serial_ports, serial_vdev) else {
                return Err(SerialHandleErr("Serial Port Check Failed!"));
            };
        } else {
            return Err(SerialHandleErr("Failed to get available ports!"));
        }

        let _device_node = serial_vdev.to_string();
        let _device_baud = 115200;
        if let Ok(device_connection) = SerialPort::open(serial_vdev, _device_baud) {
            let mut handle = Self {
                _device_node,
                _device_baud,
                device_connection,
            };

            if !handle.prove_connection() {
                return Err(SerialHandleErr("Connection proof failure!"));
            } else {
                return Ok(handle);
            }
        };

        return Err(SerialHandleErr("Failed to connect to serial device!"));
    }

    fn prove_connection(&mut self) -> bool {
        let mut retries = 1;
        while retries >= 0 {
            match self.send_command(PiShockCommand::Info) {
                Ok(res) => {
                    if let PiShockSerialCommandResponse::Data(res) = res {
                        if let Ok(_) = serde_json::from_str::<serde_json::Value>(res.as_str()) {
                            return true;
                        }
                    }
                }
                Err(_) => (),
            }

            retries -= 1;
        }
        return false;
    }

    fn check_port(port_list: Vec<PathBuf>, find_port: &str) -> Result<(), SerialHandleErr> {
        if port_list.is_empty() {
            return Err(SerialHandleErr("Ports list was empty!"));
        }
        match port_list
            .iter()
            .find(|p| p.as_path().to_str().unwrap() == find_port)
        {
            Some(fp) => fp,
            None => return Err(SerialHandleErr("Specified port was not found!")),
        };

        return Ok(());
    }

    /*
    fn find_pat(data: &[u8], pat: &[u8]) -> Option<usize> {
        data.windows(pat.len()).position(|w| w == pat)
    }
    */

    fn recv_serial_buffer(&self) -> Vec<u8> {
        let mut recvbuf = Vec::new();

        loop {
            let mut buffer = [0; 1024];
            match self.device_connection.read(&mut buffer) {
                Ok(bc) => {
                    if bc == 0 {
                        break;
                    }
                }
                Err(_) => break,
            }

            let mut idk = buffer.to_vec();
            recvbuf.append(&mut idk);
        }
        recvbuf
    }

    fn parse_cmd_response(&self, response_data: Vec<u8>) -> Result<String, SerialParseErr> {
        let mut parse_str = match String::from_utf8(response_data.clone()) {
            Ok(res) => res,
            Err(_) => {
                return Err(SerialParseErr(
                    "Failed to parse serial response into String!",
                ));
            }
        };
        parse_str = parse_str.replace('\0', "");

        let Some(res_offset) = parse_str.find("TERMINALINFO: ") else {
            return Err(SerialParseErr("No response delimeter found!"));
        };

        let mut response_str = parse_str.split_at(res_offset).1.to_string();
        if response_str.starts_with("TERMINALINFO: ") {
            response_str = response_str
                .strip_prefix("TERMINALINFO: ")
                .unwrap()
                .to_string();

            let Some(newline_offset) = response_str.find("\r\n") else {
                return Err(SerialParseErr("CRLF response delimeter missing!"));
            };
            response_str = response_str.split_at(newline_offset).0.to_string();
            Ok(response_str)
        } else {
            return Err(SerialParseErr("Incorrect prefix of response data!"));
        }
    }

    pub fn send_command(
        &mut self,
        ps_cmd: PiShockCommand,
    ) -> Result<PiShockSerialCommandResponse, SerialSendErr> {
        let cmd_data = match ps_cmd {
            PiShockCommand::Info => ps_cmd.get_cmd_string(),
            PiShockCommand::AddNetwork { .. } => ps_cmd.get_cmd_string(),
            PiShockCommand::Operate { .. } => ps_cmd.get_cmd_string(),
            PiShockCommand::RemoveNetwork { .. } => ps_cmd.get_cmd_string(),
            PiShockCommand::Restart => ps_cmd.get_cmd_string(),
            PiShockCommand::TempConnectNetwork { .. } => ps_cmd.get_cmd_string(),
        };

        self.device_connection
            .write_all(cmd_data.as_bytes())
            .unwrap();

        self.device_connection.flush().unwrap();

        if !ps_cmd.expect_response() {
            return Ok(PiShockSerialCommandResponse::None);
        }

        let response = self.recv_serial_buffer();
        if response.is_empty() {
            return Err(SerialSendErr("Response was empty!"));
        }
        let response = match self.parse_cmd_response(response) {
            Ok(res) => res,
            Err(_) => {
                return Err(SerialSendErr("Cmd parsing failure!"));
            }
        };
        return Ok(PiShockSerialCommandResponse::Data(response));
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
