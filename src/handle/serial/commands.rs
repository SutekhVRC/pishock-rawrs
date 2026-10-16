use crate::handle::serial::model::{
    AddNetworkValueModel, PiShockCommandModel, PiShockOperateCommandModel, RemoveNetworkValueModel,
    TempConnectNetworkValueModel,
};

pub enum PiShockOperationParams {
    Beep {
        id: u32,
        duration: u32,
    },
    Vibrate {
        id: u32,
        duration: u32,
        intensity: u8,
    },
    Shock {
        id: u32,
        duration: u32,
        intensity: u8,
    },
    End {
        id: u32,
    },
}

impl PiShockOperationParams {
    pub fn beep(id: u32, duration_ms: u32) -> Self {
        Self::Beep {
            id,
            duration: duration_ms,
        }
    }
    pub fn vibrate(id: u32, duration_ms: u32, intensity: u8) -> Self {
        Self::Vibrate {
            id,
            duration: duration_ms,
            intensity,
        }
    }

    pub fn shock(id: u32, duration_ms: u32, intensity: u8) -> Self {
        Self::Shock {
            id,
            duration: duration_ms,
            intensity,
        }
    }

    pub fn end(id: u32) -> Self {
        Self::End { id }
    }
}

pub enum PiShockCommand {
    AddNetwork { ssid: String, password: String },
    RemoveNetwork { ssid: String },
    TempConnectNetwork { ssid: String, password: String },
    Restart,
    Info,
    Operate { op: PiShockOperationParams },
}

impl PiShockCommand {
    pub fn add_network(ssid: &str, password: &str) -> Self {
        Self::AddNetwork {
            ssid: ssid.to_string(),
            password: password.to_string(),
        }
    }

    pub fn remove_network(ssid: &str) -> Self {
        Self::RemoveNetwork {
            ssid: ssid.to_string(),
        }
    }

    pub fn temp_connect_network(ssid: &str, password: &str) -> Self {
        Self::TempConnectNetwork {
            ssid: ssid.to_string(),
            password: password.to_string(),
        }
    }

    pub fn restart() -> Self {
        Self::Restart
    }

    pub fn info() -> Self {
        Self::Info
    }

    pub fn operate(operation: PiShockOperationParams) -> Self {
        Self::Operate { op: operation }
    }

    pub fn get_cmd_string(&self) -> String {
        match self {
            Self::AddNetwork { .. } => self.add_network_serialize(),
            Self::RemoveNetwork { .. } => self.remove_network_serialize(),
            Self::Info => {
                let mut str_data = serde_json::to_string(&PiShockCommandModel::Info {
                    cmd: "info".to_string(),
                })
                .unwrap();
                str_data.push_str("\r\n");
                str_data
            }
            Self::TempConnectNetwork { .. } => self.temp_network_connect_serialize(),
            Self::Restart => {
                let mut str_data = serde_json::to_string(&PiShockCommandModel::Restart {
                    cmd: "restart".to_string(),
                })
                .unwrap();
                str_data.push_str("\r\n");
                str_data
            }
            Self::Operate { .. } => self.operate_serialize(),
        }
    }
    fn add_network_serialize(&self) -> String {
        let cmd_value = AddNetworkValueModel::try_from(self).unwrap();
        let cmd_model = PiShockCommandModel::AddNetwork {
            cmd: "addnetwork".to_string(),
            value: cmd_value,
        };

        let mut str_data = serde_json::to_string(&cmd_model).unwrap();
        str_data.push_str("\r\n");
        str_data
    }

    fn remove_network_serialize(&self) -> String {
        let cmd_value = RemoveNetworkValueModel::try_from(self).unwrap();
        let cmd_model = PiShockCommandModel::RemoveNetwork {
            cmd: "removenetwork".to_string(),
            value: cmd_value,
        };

        let mut str_data = serde_json::to_string(&cmd_model).unwrap();
        str_data.push_str("\r\n");
        str_data
    }

    fn temp_network_connect_serialize(&self) -> String {
        let cmd_value = TempConnectNetworkValueModel::try_from(self).unwrap();
        let cmd_model = PiShockCommandModel::TempConnectNetwork {
            cmd: "connect".to_string(),
            value: cmd_value,
        };

        let mut str_data = serde_json::to_string(&cmd_model).unwrap();
        str_data.push_str("\r\n");
        str_data
    }

    fn operate_serialize(&self) -> String {
        let cmd_value = PiShockOperateCommandModel::try_from(self).unwrap();
        let cmd_model = PiShockCommandModel::Operate {
            cmd: "operate".to_string(),
            value: cmd_value,
        };

        let mut str_data = serde_json::to_string(&cmd_model).unwrap();
        str_data.push_str("\r\n");
        str_data
    }

    pub fn expect_response(&self) -> bool {
        match self {
            Self::Info => true,
            Self::AddNetwork { .. } => true,
            Self::RemoveNetwork { .. } => true,
            _ => false,
        }
    }
}
