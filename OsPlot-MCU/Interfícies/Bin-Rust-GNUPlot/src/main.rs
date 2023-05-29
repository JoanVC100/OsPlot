use std::mem;
use std::{time::Duration, io::Error, thread};
use std::fmt::Debug;

#[macro_use]
mod script_plot;
mod missatges_mcu;
use missatges_mcu::*;
mod parser;
use nix::sys::signal::Signal;
use nix::sys::{stat, signal};
use nix::unistd::{self, Pid};
use parser::*;
use tempfile::tempdir;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::select;
use tokio::sync::mpsc::{self, Receiver};
#[derive(Debug)]
enum MsgBucleSerial {
    IniciaTrigger,
    ParaTrigger,
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
/*impl Debug for SValorsOctave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("").field("x", &self.temps).field("y", &self.dada).finish()
    }
}*/
type ValorsOctave = SValorsOctave;
macro_rules! trenca_bucle {
    ($str: expr) => { eprintln!($str); break }
    ($str: expr, $err: expr) => { eprintln!($str, $err); break }
}
async fn bucle_serial(mut port: Port, mut rx_ordres: Receiver<MsgBucleSerial>) {
    let mut buffer_paquet = Vec::<u8>::with_capacity(4096);
    // Solicita la freqüència de mostreig
    if let Some(e) = port.solicita_fs().await {
        eprintln!("No s'ha pogut escriure al port sèrie per obtenir la Fs: {}", e);
        return;
    }
    let paquet = obté_paquet(&mut port, &mut buffer_paquet).await;
    let Ok(TipusMsgSerial::MCUFs(mut fs)) = paquet else {
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

    // GNUPlot
    // Crea el directori i la cua
    let dir_temp = tempdir()
        .expect("No s'ha pogut crear el directori temporal");
    let nom_cua = dir_temp.path().join("osplot.pipe");
    unistd::mkfifo(&nom_cua, stat::Mode::S_IRWXU)
        .expect("No s'ha pogut crear la cua");
    let nom_script = dir_temp.path().join("plot.gnu");
    let mut script = File::create(nom_script.clone()).await
        .expect("No s'ha pogut obrir el fitxer del script temporal");
    script.write(script_plot!().as_bytes()).await
        .expect("No s'ha pogut escriure el script de GNUPlot");
    script.flush().await.unwrap();
    drop(script);

    let mut gnuplot = tokio::process::Command::new("gnuplot")
        .arg("-e")
        .arg(format!("cua=\"{}\"", nom_cua.as_path().display().to_string()))
        .arg(nom_script)
        .spawn()
        .expect("No s'ha pogut obrir GNUPlot");
    let nom_cua = nom_cua.to_str().unwrap();
    
    let mut vector_octave = [ValorsOctave::default(); MIDA_BUFFERS];
    let actualitza_temps = |vector_octave: &mut [SValorsOctave; 4096], fs, factor_oversampling, | {
        for c in 0..MIDA_BUFFERS {
            vector_octave[c].temps = (c as f32) * (factor_oversampling as f32) / fs;
        }
    };
    actualitza_temps(&mut vector_octave, fs, factor_oversampling);
    if let Some(e) = port.inicia_trigger().await{
        eprintln!("No s'ha pogut iniciar el trigger: {}", e);
        return;
    }
    loop {
        let future_gnuplot = gnuplot.wait();
        select! {
            _ = future_gnuplot => { // Ha mort el procés durant una escriptura
                trenca_bucle!("El procés de GNUPlot ha mort. Sortint...");
            }
            resultat = rx_ordres.recv() => {
                if let Some(msg) = resultat {
                    match msg {
                        MsgBucleSerial::IniciaTrigger => {
                            if let Some(e) = port.inicia_trigger().await {
                                trenca_bucle!("No s'ha pogut iniciar el trigger: {}", e);
                            }
                        }
                        MsgBucleSerial::ParaTrigger => break,
                        MsgBucleSerial::FactorOversampling(fo) => {
                            if let Some(e) = port.canvia_factor_oversampling(fo).await {
                                trenca_bucle!("No s'ha pogut canviar el factor d'oversampling: {}", e);
                            }
                            factor_oversampling = fo;
                            actualitza_temps(&mut vector_octave, fs, factor_oversampling);
                        }
                        MsgBucleSerial::NMostres(n) => {
                            if let Some(e) = port.canvia_n_mostres(n).await {
                                trenca_bucle!("No s'ha pogut canviar el número de mostres: {}", e);
                            }
                        }
                    }
                }
                else {
                    println!("S'ha perdut la comunicació amb el mpsc central.");
                    break;
                }
            }
            resultat_paquet = obté_paquet(&mut port, &mut buffer_paquet) => {
                let paquet = resultat_paquet.unwrap();
                match paquet {
                    TipusMsgSerial::MCUFinestra => {
                        for (val, &byte) in vector_octave.iter_mut().zip(buffer_paquet.iter()) {
                            val.dada = byte;
                        }
                        let vector_octave: &[u8] = unsafe {
                            let ptr = vector_octave.as_ptr() as *const u8;
                            std::slice::from_raw_parts(ptr, buffer_paquet.len()*std::mem::size_of::<ValorsOctave>())
                        };
                        let mut cua_gnuplot = File::create(nom_cua).await
                            .expect("No s'ha pogut obrir la cua");
                        let future_cua = cua_gnuplot.write_all(vector_octave);
                        let future_gnuplot = gnuplot.wait();
                        select! {
                            resultat = future_cua => {
                                resultat.expect("No s'ha pogut escriure a la cua");
                                cua_gnuplot.flush().await.expect("No s'ha pogut acabar d'escriure a la cua");
                            }
                            _ = future_gnuplot => { // Ha mort el procés durant una escriptura
                                println!("El procés de GNUPlot ha mort. Sortint...");
                                break;
                            }
                        }
                    }
                    TipusMsgSerial::MCUFactorOversamplingCanviat => println!("Factor d'oversampling canviat"),
                    TipusMsgSerial::MCUFactorOversampling(fo) => {
                        factor_oversampling = fo;
                        actualitza_temps(&mut vector_octave, fs, factor_oversampling);
                    }
                    TipusMsgSerial::MCUNMostresCanviades => println!("Mostres canviades"),
                    TipusMsgSerial::MCUNMostres(_) => (),
                    TipusMsgSerial::MCUFs(fss) => {
                        fs = fss;
                        actualitza_temps(&mut vector_octave, fs, factor_oversampling);
                    }
                    TipusMsgSerial::MCUError => println!("Paquet erroni"),
                }
            }
        }
    }
    port.solicita_fs().await;
    if gnuplot.try_wait().unwrap().is_none() {
        signal::kill(Pid::from_raw(gnuplot.id().unwrap().try_into().unwrap()), Signal::SIGINT).unwrap();
    }
    gnuplot.wait().await.unwrap();
}

#[tokio::main]
async fn main() {
    let s = "/dev/ttyACM0";
    let port = Port::nou(&s.to_string()).unwrap();
    let (tx_bucle_serial, mut rx) = mpsc::channel(8);
    bucle_serial(port, rx).await;
}
