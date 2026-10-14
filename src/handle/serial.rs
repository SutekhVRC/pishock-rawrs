use std::{fmt, path::PathBuf};

use crate::{
    connect::{
        ConnectTarget::{self, Serial},
        PiShockConnect,
        error::ConnectErr,
    },
    handle::error::HandleErr,
};

#[derive(Debug)]
pub struct PiShockSerialHandle {
    device_node: String,
}

impl PiShockSerialHandle {
    fn new(serial_vdev: String) -> Result<Self, HandleErr> {
        Ok(Self {
            device_node: serial_vdev,
        })
    }
}

impl fmt::Display for PiShockSerialHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PiShockConnect for PiShockSerialHandle {
    type Output = Result<Self, ConnectErr>;
    fn connect(&self, target: ConnectTarget) -> Output {
        if let ConnectTarget::Serial(vdev) = target {
            Ok(Self::new(vdev).unwrap())
        } else {
            todo!()
        }
    }
}
