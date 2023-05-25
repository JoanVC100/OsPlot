pub enum AltresMsgBucleSerial {
    ParaLlegir,
}

pub enum MsgBucleSerial {
    Altres(AltresMsgBucleSerial),
    FactorOversampling(u8),
    NMostres(u16)
}
impl Default for MsgBucleSerial {
    fn default() -> Self {
        return MsgBucleSerial::Altres(AltresMsgBucleSerial::ParaLlegir);
    }
}