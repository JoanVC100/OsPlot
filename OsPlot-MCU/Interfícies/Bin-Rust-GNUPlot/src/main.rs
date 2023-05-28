use std::mem;
use std::{time::Duration, io::Error, thread};
use std::fmt::Debug;

#[macro_use]
mod script_plot;
mod missatges_mcu;
use missatges_mcu::*;
mod parser;
use parser::*;
use tokio::sync::mpsc::{self, Receiver};
enum MsgBucleSerial {
    IniciaTrigger,
    FactorOversampling(u8),
    NMostres(u16)
}
async fn obté_paquet(port: &mut Port, buffer_paquet: &mut Vec<u8>) -> Result<TipusMsgSerial, Error> {
    let resultat_paquet = port.llegeix_paquet(buffer_paquet).await;
    if let Err(e) = resultat_paquet {
        eprintln!("Error en llegir paquet del port sèrie: {}", e);
        return Err(e);
    }
    return Ok(resultat_paquet.unwrap());
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SValorsOctave {
    pub temps: f32,
    pub dada: u8
}
impl Default for SValorsOctave {
    fn default() -> Self {
        return Self {temps: 0., dada: 0}
    }
}
impl Debug for SValorsOctave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("").field("x", &self.temps).field("y", &self.dada).finish()
    }
}
type ValorsOctave = SValorsOctave;

async fn bucle_serial(mut port: Port, mut rx_ordres: Receiver<MsgBucleSerial>) {
    let mut buffer_paquet = Vec::<u8>::with_capacity(4096);
    // Solicita la freqüència de mostreig
    if let Some(e) = port.solicita_fs().await {
        eprintln!("No s'ha pogut escriure al port sèrie per obtenir la Fs: {}", e);
        return;
    }
    let paquet = obté_paquet(&mut port, &mut buffer_paquet).await;
    let Ok(TipusMsgSerial::MCUFs(fs)) = paquet else {
        eprintln!("No s'ha llegit bé la Fs: {:?} {:?}", buffer_paquet, paquet);
        return;
    };
    // Solicita el factor d'oversampling
    if let Some(e) = port.solicita_factor_oversampling().await {
        eprintln!("No s'ha pogut escriure al port sèrie per obtenir el factor d'oversampling: {}", e);
        return;
    }
    let paquet = obté_paquet(&mut port, &mut buffer_paquet).await;
    let Ok(TipusMsgSerial::MCUFactorOversampling(mut factor_oversampling)) = paquet else {
        eprintln!("No s'ha llegit bé el factor d'oversampling: {:?} {:?}", buffer_paquet, paquet.unwrap_err());
        return;
    };
    // Printar els valors inicials
    println!("Freqüència de mostreig: {} Oversampling: {}", fs, factor_oversampling);
    
    let mut vector_octave = [ValorsOctave::default(); MIDA_BUFFERS];
    for c in 0..MIDA_BUFFERS {
        vector_octave[c].temps = (c as f32) * (factor_oversampling as f32) / fs;
    }

    if let Some(e) = port.inicia_trigger().await{
        eprintln!("No s'ha pogut iniciar el trigger: {}", e);
        return;
    }
    loop {
        if let Ok(msg) = rx_ordres.try_recv() {
            match msg {
                MsgBucleSerial::IniciaTrigger => {
                    if let Some(e) = port.inicia_trigger().await{
                        eprintln!("No s'ha pogut iniciar el trigger: {}", e);
                        return;
                    }
                }
                MsgBucleSerial::FactorOversampling(fo) => {
                    if let Some(e) = port.canvia_factor_oversampling(fo).await{
                        eprintln!("No s'ha pogut canviar el factor d'oversampling: {}", e);
                        return;
                    }
                    factor_oversampling = fo;
                    for c in 0..MIDA_BUFFERS {
                        vector_octave[c].temps = (c as f32) * (factor_oversampling as f32) / fs;
                    }
                }
                MsgBucleSerial::NMostres(n) => {
                    if let Some(e) = port.canvia_n_mostres(n).await{
                        eprintln!("No s'ha pogut canviar el número de mostres: {}", e);
                        return;
                    }
                }
            }
        }
        let resultat_paquet = obté_paquet(&mut port, &mut buffer_paquet).await;
        let paquet = resultat_paquet.unwrap();
        match paquet {
            TipusMsgSerial::MCUFinestra => {
                for (val, &byte) in vector_octave.iter_mut().zip(buffer_paquet.iter()) {
                    val.dada = byte;
                }
                let vector_octave: &[u8] = unsafe {
                    let ptr = vector_octave.as_ptr() as *const u8;
                    std::slice::from_raw_parts(ptr, mem::size_of_val(&vector_octave))
                };
                println!("{:?}", vector_octave);
            }
            TipusMsgSerial::MCUFactorOversamplingCanviat => println!(""),
            TipusMsgSerial::MCUFactorOversampling(_) => todo!(),
            TipusMsgSerial::MCUNMostresCanviades => println!("Mostres canviades"),
            TipusMsgSerial::MCUNMostres(_) => todo!(),
            TipusMsgSerial::MCUFs(_) => todo!(),
            TipusMsgSerial::MCUError => println!("Paquet erroni"),
        }
    }
}

#[tokio::main]
async fn main() {
    let s = "/dev/ttyACM0";
    let port = Port::nou(&s.to_string()).unwrap();
    let (tx_bucle_serial, mut rx) = mpsc::channel(8);
    bucle_serial(port, rx).await;
}
