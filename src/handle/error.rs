#[derive(Debug)]
pub struct SerialHandleErr(pub &'static str);
#[derive(Debug)]
pub struct SerialSendErr(pub &'static str);
#[derive(Debug)]
pub struct SerialParseErr(pub &'static str);
