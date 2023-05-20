use std::io::{Write, Read, ErrorKind, BufReader, self};

use std::io::Error;
use std::mem::{self, MaybeUninit};
use serialport::TTYPort;

pub const MAXIM_FSS: usize = 16;
pub type FreqMostreig = f32;

pub enum MsgCapçaleraPC {
    PCIniciaTrigger = 0,
    PCCanviarFactorOversampling,
    PCRetornaFactorOversampling,
    PCRetornaFS,
    PCCanviarNMostres,
    PCRetornaNMostres
}
pub enum MsgCapçaleraMCU {
    MCUOk = 129,
    MCUError
}

pub struct Port {
    e: TTYPort,
    l: BufReader<TTYPort>
}

impl Port {
    pub fn nou(port: TTYPort) -> Self {
        let port_escriptura = port.try_clone_native().unwrap();
        let port_lectura = BufReader::new(port);
        return Self{e: port_escriptura, l: port_lectura};
    }

    pub fn llegeix_1(&mut self, serial_buf: &mut [u8; 1]) -> io::Result<()>  {
        return self.l.read_exact(serial_buf);
    }

    fn llegir_bytes<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        #[allow(invalid_value)] // L'array sempre estarà inicialitzada si es retorna
        let mut serial_buf =
            unsafe { mem::transmute::<_, [u8; N]>(MaybeUninit::<[u8; N]>::uninit().assume_init()) };
        match self.l.read_exact(&mut serial_buf) {
            Ok(_) => Ok(serial_buf),
            Err(e) => Err(e)
        }
    }

    #[inline(always)]
    fn escriu_bytes<const N: usize>(&mut self, serial_buf: [u8; N]) -> Result<(), Error> {
        return self.e.write_all(&serial_buf);
    }

    fn escriu_1_llegeix(&mut self, capçalera: u8) -> Option<Error> {
        match self.escriu_bytes::<1>([capçalera as u8]) {
            Ok(_) => (),
            Err(e) => return Some(e)
        }
        return match self.llegir_bytes::<1>() {
            Ok(byte_llegit) => {
                if byte_llegit[0] != MsgCapçaleraMCU::MCUOk as u8 {
                    Some(Error::from(ErrorKind::InvalidData))
                }
                else { None }
            }
            Err(e) => return Some(e)
        }
    }

    fn escriu_1_llegeix_n<const N: usize>(&mut self, capçalera: u8) -> Result<[u8; N], Error> {
        return match self.escriu_1_llegeix(capçalera) {
            None => self.llegir_bytes::<N>(),
            Some(e) => return Err(e)
        }
    }

    fn escriu_n_llegeix_1<const N: usize>(&mut self, capçalera: u8, dades: [u8; N]) -> Option<Error> {
        match self.escriu_bytes::<1>([capçalera]) {
            Ok(_) => (),
            Err(e) => return Some(e)
        }
        return match self.escriu_bytes::<N>(dades) {
            Ok(_) => {
                match self.llegir_bytes::<1>() {
                    Ok(byte_llegit) => {
                        if byte_llegit[0] != MsgCapçaleraMCU::MCUOk as u8 {
                            Some(Error::from(ErrorKind::InvalidData))
                        }
                        else { None }
                    }
                    Err(e) => Some(e)
                }
            }
            Err(e) => Some(e)
        }
    }

    pub fn inicia_trigger(&mut self) -> Option<Error> {
        return self.escriu_1_llegeix(MsgCapçaleraPC::PCIniciaTrigger as u8);
    }

    pub fn retorna_fs(&mut self) -> Result<FreqMostreig, Error> {
        match self.escriu_1_llegeix_n::<4>(MsgCapçaleraPC::PCRetornaFS as u8) {
            Ok(fs) => Ok(FreqMostreig::from_le_bytes(fs)),
            Err(e) => Err(e)
        }
    }
    
    pub fn retorna_factor_oversampling(&mut self) -> Result<u8, Error> {
        match self.escriu_1_llegeix_n::<1>(MsgCapçaleraPC::PCRetornaFactorOversampling as u8) {
            Ok(fo) => Ok(u8::from_le_bytes(fo)),
            Err(e) => Err(e)
        }
    }
    
    pub fn modifica_factor_oversampling(&mut self, factor_oversampling: u8) -> Option<Error> {
        return self.escriu_n_llegeix_1::<1>(MsgCapçaleraPC::PCCanviarFactorOversampling as u8, [factor_oversampling]);
    }
}