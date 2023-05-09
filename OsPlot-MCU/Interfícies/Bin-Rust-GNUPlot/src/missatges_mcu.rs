use std::io::{Write, Read, ErrorKind};

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

fn llegir_bytes<const N: usize>(port: &mut TTYPort) -> Result<[u8; N], Error> {
    #[allow(invalid_value)] // L'array sempre estarà inicialitzada si es retorna
    let mut serial_buf =
        unsafe { mem::transmute::<_, [u8; N]>(MaybeUninit::<[u8; N]>::uninit().assume_init()) };
    match port.read_exact(&mut serial_buf) {
        Ok(_) => Ok(serial_buf),
        Err(e) => Err(e)
    }
}

#[inline(always)]
fn escriu_bytes<const N: usize>(port: &mut TTYPort, serial_buf: [u8; N]) -> Result<(), Error> {
    return port.write_all(&serial_buf);
}

fn escriu_1_llegeix(port: &mut TTYPort, capçalera: u8) -> Option<Error> {
    match escriu_bytes::<1>(port, [capçalera as u8]) {
        Ok(_) => (),
        Err(e) => return Some(e)
    }
    return match llegir_bytes::<1>(port) {
        Ok(byte_llegit) => {
            if byte_llegit[0] != MsgCapçaleraMCU::MCUOk as u8 {
                Some(Error::from(ErrorKind::InvalidData))
            }
            else { None }
        }
        Err(e) => return Some(e)
    }
}

fn escriu_1_llegeix_n<const N: usize>(port: &mut TTYPort, capçalera: u8) -> Result<[u8; N], Error> {
    return match escriu_1_llegeix(port, capçalera) {
        None => llegir_bytes::<N>(port),
        Some(e) => return Err(e)
    }
}

fn escriu_n_llegeix_1<const N: usize>(port: &mut TTYPort, capçalera: u8, dades: [u8; N]) -> Option<Error> {
    match escriu_bytes::<1>(port, [capçalera]) {
        Ok(_) => (),
        Err(e) => return Some(e)
    }
    return match escriu_bytes::<N>(port, dades) {
        Ok(_) => {
            match llegir_bytes::<1>(port) {
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


pub fn inicia_trigger(port: &mut TTYPort) -> Option<Error> {
    return escriu_1_llegeix(port, MsgCapçaleraPC::PCIniciaTrigger as u8);
}

pub fn retorna_fs(port: &mut TTYPort) -> Result<FreqMostreig, Error> {
    match escriu_1_llegeix_n::<4>(port, MsgCapçaleraPC::PCRetornaFS as u8) {
        Ok(fs) => Ok(FreqMostreig::from_le_bytes(fs)),
        Err(e) => Err(e)
    }
}

pub fn retorna_factor_oversampling(port: &mut TTYPort) -> Result<u8, Error> {
    match escriu_1_llegeix_n::<1>(port, MsgCapçaleraPC::PCRetornaFactorOversampling as u8) {
        Ok(fo) => Ok(u8::from_le_bytes(fo)),
        Err(e) => Err(e)
    }
}

pub fn modifica_factor_oversampling(port: &mut TTYPort, factor_oversampling: u8) -> Option<Error> {
    return escriu_n_llegeix_1::<1>(port, MsgCapçaleraPC::PCCanviarFactorOversampling as u8, [factor_oversampling]);
}