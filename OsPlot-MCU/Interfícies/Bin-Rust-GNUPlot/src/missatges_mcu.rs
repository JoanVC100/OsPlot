pub enum MsgCapçaleraPC {
    PCIniciaTrigger = 0,
    PCCanviarFS,
    PCRetornaPossiblesFS,
    PCCanviarNMostres,
    PCRetornaNMostres
}

pub enum MsgCapçaleraMCU {
    MCUOk = 129,
    MCUError,
    MCURetornaFSS,
    MCURetornaNMostres
}