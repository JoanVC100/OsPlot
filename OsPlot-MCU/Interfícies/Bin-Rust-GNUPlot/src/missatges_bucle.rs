pub enum CapMsgBucleSerial {
    ParaLlegir,
}

pub struct MsgBucleSerial {
    pub capçalera: CapMsgBucleSerial,
    pub valor: u32
}
impl Default for MsgBucleSerial {
    fn default() -> Self {
        MsgBucleSerial { capçalera: CapMsgBucleSerial::ParaLlegir, valor: 0 }
    }
}