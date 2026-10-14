use super::connect::error::ConnectErr;

pub enum PiShockRawrsErr {
    Connect(ConnectErr),
}
