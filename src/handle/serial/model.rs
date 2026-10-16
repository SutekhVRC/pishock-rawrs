use serde::{Deserialize, Serialize};

use crate::handle::serial::commands::{PiShockCommand, PiShockOperationParams};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PiShockOperateCommandModel {
    Beep {
        id: u32,
        op: String,
        duration: u32,
    },
    Shock {
        id: u32,
        op: String,
        duration: u32,
        intensity: u8,
    },
    Vibrate {
        id: u32,
        op: String,
        duration: u32,
        intensity: u8,
    },
    End {
        id: u32,
        op: String,
        // According to documentation should always be 0
        duration: u32,
    },
}

impl TryFrom<&PiShockCommand> for PiShockOperateCommandModel {
    type Error = &'static str;
    fn try_from(value: &PiShockCommand) -> Result<Self, Self::Error> {
        match value {
            PiShockCommand::Operate { op } => match *op {
                PiShockOperationParams::Beep { id, duration } => Ok(Self::Beep {
                    id,
                    op: "beep".to_string(),
                    duration,
                }),
                PiShockOperationParams::Vibrate {
                    id,
                    duration,
                    intensity,
                } => Ok(Self::Vibrate {
                    id,
                    op: "vibrate".to_string(),
                    duration,
                    intensity,
                }),
                PiShockOperationParams::Shock {
                    id,
                    duration,
                    intensity,
                } => Ok(Self::Shock {
                    id,
                    op: "shock".to_string(),
                    duration,
                    intensity,
                }),
                PiShockOperationParams::End { id } => Ok(Self::End {
                    id,
                    op: "end".to_string(),
                    duration: 0,
                }),
            },
            _ => {
                Err("Unexpected command in PiShockCommand->PiShockOperateCommandModel conversion!")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddNetworkValueModel {
    ssid: String,
    password: String,
}

impl TryFrom<&PiShockCommand> for AddNetworkValueModel {
    type Error = &'static str;
    fn try_from(value: &PiShockCommand) -> Result<Self, Self::Error> {
        match value {
            PiShockCommand::AddNetwork { ssid, password } => Ok(Self {
                ssid: ssid.to_string(),
                password: password.to_string(),
            }),
            _ => Err("Unexpected command in PiShockCommand->AddNetworkValueModel conversion!"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveNetworkValueModel(String);
impl TryFrom<&PiShockCommand> for RemoveNetworkValueModel {
    type Error = &'static str;
    fn try_from(value: &PiShockCommand) -> Result<Self, Self::Error> {
        match value {
            PiShockCommand::RemoveNetwork { ssid } => Ok(Self(ssid.to_string())),
            _ => Err("Unexpected command in PiShockCommand->RemoveNetworkValueModel conversion!"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempConnectNetworkValueModel {
    ssid: String,
    password: String,
}
impl TryFrom<&PiShockCommand> for TempConnectNetworkValueModel {
    type Error = &'static str;
    fn try_from(value: &PiShockCommand) -> Result<Self, Self::Error> {
        match value {
            PiShockCommand::TempConnectNetwork { ssid, password } => Ok(Self {
                ssid: ssid.to_string(),
                password: password.to_string(),
            }),
            _ => Err(
                "Unexpected command in PiShockCommand->TempConnectNetworkValueModel conversion!",
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PiShockCommandModel {
    AddNetwork {
        cmd: String,
        value: AddNetworkValueModel,
    },
    RemoveNetwork {
        cmd: String,
        value: RemoveNetworkValueModel,
    },
    TempConnectNetwork {
        cmd: String,
        value: TempConnectNetworkValueModel,
    },
    Restart {
        cmd: String,
    },
    Info {
        cmd: String,
    },
    Operate {
        cmd: String,
        value: PiShockOperateCommandModel,
    },
}
