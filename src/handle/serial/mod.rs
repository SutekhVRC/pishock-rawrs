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

            if handle.prove_connection() {
                return Ok(handle);
            }
            return Err(SerialHandleErr("Connection proof failure!"));
        };

        return Err(SerialHandleErr("Failed to connect to serial device!"));
    }

    fn prove_connection(&mut self) -> bool {
        let mut retries = 1;
        while retries >= 0 {
            if let Ok(res_data) = self.send_command(PiShockCommand::Info) {
                if let PiShockSerialCommandResponse::Data(res) = res_data {
                    if let Ok(_) = serde_json::from_str::<serde_json::Value>(res.as_str()) {
                        return true;
                    };
                }
            };
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

    fn find_pat(data: &[u8], pat: &[u8]) -> Option<usize> {
        data.windows(pat.len()).position(|w| w == pat)
    }

    fn recv_serial_buffer(&self) -> Vec<u8> {
        self.device_connection.discard_buffers().unwrap();
        self.device_connection.flush().unwrap();
        let mut recvbuf = Vec::new();
        loop {
            let mut buffer = [0; 256];
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

    fn parse_cmd_response(&self, mut response_data: Vec<u8>) -> Result<String, SerialParseErr> {
        let Some(res_offset) = Self::find_pat(&response_data, b"TERMINALINFO: ") else {
            return Err(SerialParseErr("No response delimeter found!"));
        };
        response_data = response_data.split_at(res_offset).1.to_vec();

        if response_data.starts_with("TERMINALINFO: ".as_bytes()) {
            response_data = response_data
                .strip_prefix("TERMINALINFO: ".as_bytes())
                .unwrap()
                .to_vec();
            let newline_offset = Self::find_pat(&response_data, b"\n").unwrap();
            response_data = response_data.split_at(newline_offset).0.to_vec();
            match String::from_utf8(response_data) {
                Ok(res) => Ok(res),
                Err(_) => Err(SerialParseErr("Failed to parse JSON payload into String!")),
            }
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
        let Ok(response) = self.parse_cmd_response(response) else {
            return Err(SerialSendErr("Failed to parse cmd response!"));
        };

        let response = response.replace('\0', "");

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
