pub mod error;

pub trait PiShockConnect {
    type Output;
    fn connect(&self, target: ConnectTarget) -> Self::Output;
}

pub enum ConnectTarget {
    Http(String),
    Serial(String),
}
