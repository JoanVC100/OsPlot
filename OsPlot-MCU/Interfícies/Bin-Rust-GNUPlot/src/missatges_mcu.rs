use std::{time::Duration, io::Error, thread};
use std::fmt::Debug;

use tokio::io::{BufReader, AsyncWriteExt, AsyncBufReadExt, AsyncReadExt};
use tokio_serial::{SerialStream, SerialPortBuilderExt, SerialPort};


pub const BAUDRATE: u32 = 1_000_000;
pub const BYTE_ESCAPAMENT: u8 = 128;
pub type FreqMostreig = f32;
pub const MIDA_BUFFERS: usize = 4096;

enum MsgCapçaleraPC {
    PCIniciaTrigger = 0,
    PCCanviarFactorOversampling,
    PCRetornaFactorOversampling,
    PCCanviarNMostres,
    PCRetornaNMostres,
    PCRetornaFs,
    PCCanviarNivellTrigger,
    PCRetornaNivellTrigger
}

enum MsgCapçaleraMCU {
    MCUFinestra = 129,
    MCUFactorOversamplingCanviat,
    MCUFactorOversampling,
    MCUNMostresCanviades,
    MCUNMostres,
    MCUFs,
    MCUNivellTriggerCanviat,
    MCUNivellTrigger,
    MCUError = 255,
}
impl MsgCapçaleraMCU {
    fn try_from(v: u8) -> Result<Self, Error> {
        return match v {
            129 => Ok(Self::MCUFinestra),
            130 => Ok(Self::MCUFactorOversamplingCanviat),
            131 => Ok(Self::MCUFactorOversampling),
            132 => Ok(Self::MCUNMostresCanviades),
            133 => Ok(Self::MCUNMostres),
            134 => Ok(Self::MCUFs),
            135 => Ok(Self::MCUNivellTriggerCanviat),
            136 => Ok(Self::MCUNivellTrigger),
            _ => Ok(Self::MCUError)
        }
    }
}

pub enum TipusMsgSerial {
    MCUFinestra,
    MCUFactorOversamplingCanviat,
    MCUFactorOversampling(u8),
    MCUNMostresCanviades,
    MCUNMostres(u16),
    MCUFs(FreqMostreig),
    MCUNivellTriggerCanviat,
    MCUNivellTrigger(u8),
    MCUError
}
impl Debug for TipusMsgSerial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MCUFinestra => write!(f, "MCUFinestra"),
            Self::MCUFactorOversamplingCanviat => write!(f, "MCUFactorOversamplingCanviat"),
            Self::MCUFactorOversampling(arg0) => f.debug_tuple("MCUFactorOversampling").field(arg0).finish(),
            Self::MCUNMostresCanviades => write!(f, "MCUNMostresCanviades"),
            Self::MCUNMostres(arg0) => f.debug_tuple("MCUNMostres").field(arg0).finish(),
            Self::MCUFs(arg0) => f.debug_tuple("MCUFs").field(arg0).finish(),
            Self::MCUNivellTriggerCanviat => write!(f, "MCUNivellTriggerCanviat"),
            Self::MCUNivellTrigger(arg0) => f.debug_tuple("MCUNivellTrigger").field(arg0).finish(),
            Self::MCUError => write!(f, "MCUError"),
        }
    }
}

pub struct Port {
    l: BufReader<SerialStream>,
    serial_buf: Vec<u8>
}

impl Port {
    pub fn nou(port_name: &String) -> Result<Self, Error> {
        let port_resultat = tokio_serial::new(port_name, BAUDRATE)
            .open_native_async();
        if port_resultat.is_err() {
            return Err(port_resultat.unwrap_err().into());
        }
        let port = port_resultat.unwrap();
        thread::sleep(Duration::from_secs(3));
        port.clear(tokio_serial::ClearBuffer::All).unwrap();
        return Ok(Self {l: BufReader::new(port), serial_buf: Vec::with_capacity(MIDA_BUFFERS)});
    }

    async fn escriptura(&mut self, serial_buf: &[u8]) -> Option<Error> {
        return match self.l.write(serial_buf).await {
            Ok(_) => None,
            Err(e) => Some(e)
        };
    }

    pub async fn inicia_trigger(&mut self) -> Option<Error> {
        return self.escriptura(&[MsgCapçaleraPC::PCIniciaTrigger as u8]).await;
    }

    pub async fn solicita_fs(&mut self) -> Option<Error> {
        return self.escriptura(&[MsgCapçaleraPC::PCRetornaFs as u8]).await;
    }

    pub async fn solicita_factor_oversampling(&mut self) -> Option<Error> {
        return self.escriptura(&[MsgCapçaleraPC::PCRetornaFactorOversampling as u8]).await;
    }

    pub async fn solicita_n_mostres(&mut self) -> Option<Error> {
        return self.escriptura(&[MsgCapçaleraPC::PCRetornaNMostres as u8]).await;
    }

    pub async fn canvia_factor_oversampling(&mut self, fo: u8) -> Option<Error> {
        return self.escriptura(&[MsgCapçaleraPC::PCCanviarFactorOversampling as u8, fo]).await;
    }

    pub async fn canvia_n_mostres(&mut self, n: u16) -> Option<Error> {
        return self.escriptura(&[MsgCapçaleraPC::PCCanviarNMostres as u8, n as u8, (n >> 8) as u8]).await;
    }

    pub async fn canvia_nivell_trigger(&mut self, t: u8) -> Option<Error> {
        return self.escriptura(&[MsgCapçaleraPC::PCCanviarNivellTrigger as u8, t]).await;
    }

    pub async fn llegeix_paquet(&mut self, buf_retorn: &mut Vec<u8>) -> Result<TipusMsgSerial, Error> {
        buf_retorn.clear();
        self.serial_buf.clear();
        loop {
            if let Err(e) = self.l.read_until(BYTE_ESCAPAMENT, &mut self.serial_buf).await {
                return Err(e);
            }
            buf_retorn.append(&mut self.serial_buf);
            self.serial_buf.clear();
            let byte_seguent = self.l.read_u8().await;
            if byte_seguent.is_ok() {
                let byte_seguent = byte_seguent.unwrap();
                if byte_seguent != BYTE_ESCAPAMENT {
                    buf_retorn.pop();
                    let byte_seguent = MsgCapçaleraMCU::try_from(byte_seguent);
                    if let Ok(byte_seguent) = byte_seguent {
                        return Ok(match byte_seguent as MsgCapçaleraMCU {
                            MsgCapçaleraMCU::MCUFinestra => TipusMsgSerial::MCUFinestra,
                            MsgCapçaleraMCU::MCUFactorOversamplingCanviat => TipusMsgSerial::MCUFactorOversamplingCanviat,
                            MsgCapçaleraMCU::MCUFactorOversampling => TipusMsgSerial::MCUFactorOversampling(buf_retorn[0]),
                            MsgCapçaleraMCU::MCUNMostresCanviades => TipusMsgSerial::MCUNMostresCanviades,
                            MsgCapçaleraMCU::MCUNMostres => TipusMsgSerial::MCUNMostres(u16::from_le_bytes([buf_retorn[0], buf_retorn[1]])),
                            MsgCapçaleraMCU::MCUFs => TipusMsgSerial::MCUFs(FreqMostreig::from_le_bytes([buf_retorn[0], buf_retorn[1], buf_retorn[2], buf_retorn[3]])),
                            MsgCapçaleraMCU::MCUNivellTriggerCanviat => TipusMsgSerial::MCUNivellTriggerCanviat,
                            MsgCapçaleraMCU::MCUNivellTrigger => TipusMsgSerial::MCUNivellTrigger(buf_retorn[0]),
                            _ => TipusMsgSerial::MCUError,
                        });
                    }
                    else if let Err(e) = byte_seguent{
                        return Err(e);
                    };
                }
            }
            else {
                return Err(byte_seguent.unwrap_err());
            }
        }
    }
    
}