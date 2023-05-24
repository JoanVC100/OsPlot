use std::io::{Write, Read, ErrorKind, BufReader, self};

use std::io::Error;
use std::time::Duration;
use serialport::{TTYPort};

pub const BAUDRATE: u32 = 1_000_000;
pub const BYTE_ESCAPAMENT: u8 = 128;
pub type FreqMostreig = f32;

enum MsgCapçaleraPC {
    PCIniciaTrigger = 0,
    PCCanviarFactorOversampling,
    PCRetornaFactorOversampling,
    PCRetornaFS,
    PCCanviarNMostres,
    PCRetornaNMostres
}
enum MsgCapçaleraMCU {
    MCUOk = 129,
    MCUError
}

pub struct Port {
    e: TTYPort,
    l: BufReader<TTYPort>
}

impl Port {
    pub fn nou(port_name: &String) -> Result<Self, Error> {
        let port_resultat = serialport::new(port_name, BAUDRATE)
            .timeout(Duration::from_millis(400))
            .open_native();
        if port_resultat.is_err() {
            return Err(port_resultat.unwrap_err().into());
        }
        let port = port_resultat.unwrap();
        let port_escriptura = port.try_clone_native().unwrap();
        let mut port_lectura = BufReader::new(port);
        let mut buf = vec![0u8; 1000];
        let mut intents = 5;
        while let Err(e) = port_lectura.read_exact(&mut buf) {
            if e.kind() != std::io::ErrorKind::TimedOut {
                panic!("Error al llegir byte del port sèrie: {:?}", e);
            }
            else if intents == 0 {
                break;
            }
            intents -= 1;
        }
        return Ok(Self{e: port_escriptura, l: port_lectura});
    }

    pub fn llegeix_1(&mut self, serial_buf_rx: &mut [u8; 1]) -> io::Result<()>  {
        return self.l.read_exact(serial_buf_rx);
    }

    fn escriu_i_llegeix_confirmació(&mut self, serial_buf_tx: &[u8], serial_buf_rx: &mut [u8]) -> Option<Error> {
        self.e.write(serial_buf_tx).unwrap();
        while let Err(e) = self.l.read_exact(&mut serial_buf_rx[..1]) {
            if e.kind() != std::io::ErrorKind::TimedOut {
                panic!("Error al llegir byte del port sèrie: {:?}", e);
            }
        }
        if serial_buf_rx[0] == MsgCapçaleraMCU::MCUOk as u8 { return None; }
        return Some(Error::from(ErrorKind::InvalidData));
    }

    fn llegeix_parametre(&mut self, serial_buf_rx: &mut [u8]) -> Option<Error> {
        while let Err(e) = self.l.read_exact(serial_buf_rx) {
            if e.kind() != std::io::ErrorKind::TimedOut {
                panic!("Error al llegir byte del port sèrie: {:?}", e);
            }
        }
        return None;
    }

    pub fn inicia_trigger(&mut self) -> Option<Error> {
        let mut serial_buf_rx = [0u8];
        return self.escriu_i_llegeix_confirmació(&[MsgCapçaleraPC::PCIniciaTrigger as u8], &mut serial_buf_rx)
    }

    pub fn retorna_fs(&mut self) -> Result<FreqMostreig, Error> {
        let mut serial_buf_rx = [0u8; 4];
        if let Some(e) = self.escriu_i_llegeix_confirmació(&[MsgCapçaleraPC::PCRetornaFS as u8], &mut serial_buf_rx) {
            return Err(e);
        }
        if let Some(e) = self.llegeix_parametre(&mut serial_buf_rx) {
            return Err(e);
        }
        return Ok(FreqMostreig::from_le_bytes(serial_buf_rx));
    }
    
    pub fn retorna_factor_oversampling(&mut self) -> Result<u8, Error> {
        let mut serial_buf_rx = [0u8; 1];
        if let Some(e) = self.escriu_i_llegeix_confirmació(&[MsgCapçaleraPC::PCRetornaFactorOversampling as u8], &mut serial_buf_rx) {
            return Err(e);
        }
        if let Some(e) = self.llegeix_parametre(&mut serial_buf_rx) {
            return Err(e);
        }
        return Ok(u8::from_le_bytes(serial_buf_rx));
    }

    pub fn retorna_n_mostres(&mut self) -> Result<u16, Error> {
        let mut serial_buf_rx = [0u8; 2];
        if let Some(e) = self.escriu_i_llegeix_confirmació(&[MsgCapçaleraPC::PCRetornaNMostres as u8], &mut serial_buf_rx) {
            return Err(e);
        }
        if let Some(e) = self.llegeix_parametre(&mut serial_buf_rx) {
            return Err(e);
        }
        return Ok(u16::from_le_bytes(serial_buf_rx));
    }
    
    pub fn modifica_factor_oversampling(&mut self, factor_oversampling: u8) -> Option<Error> {
        let serial_buf_tx = [MsgCapçaleraPC::PCCanviarFactorOversampling as u8, factor_oversampling];
        let mut serial_buf_rx = [0u8];
        return self.escriu_i_llegeix_confirmació(&serial_buf_tx, &mut serial_buf_rx);
    }
}