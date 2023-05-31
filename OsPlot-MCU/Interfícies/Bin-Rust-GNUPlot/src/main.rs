use std::thread;
use std::fmt::Debug;
use std::io::{BufReader, BufRead, Error};
use clap::{Command, Arg, value_parser};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::select;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::oneshot;
use nix::sys::signal::Signal;
use nix::sys::{stat, signal};
use nix::unistd::{self, Pid};
use tempfile::tempdir;


#[macro_use]
mod script_plot;
mod missatges_mcu;
use missatges_mcu::*;
mod parser;
use parser::*;

#[derive(Debug, PartialEq)]
enum MsgBucleSerial {
    IniciaTrigger,
    ParaTrigger,
    FactorOversampling(u8),
    NMostres(u16),
    NivellTrigger(u8),
    OrdreInvàlida
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
    ($str: expr) => { eprintln!($str); break };
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
                        MsgBucleSerial::NivellTrigger(t) => {
                            if let Some(e) = port.canvia_nivell_trigger(t).await {
                                trenca_bucle!("No s'ha pogut canviar el nivell del trigger: {}", e);
                            }
                        }
                        MsgBucleSerial::OrdreInvàlida => panic!("No es pot rebre l'enum ordre invàl·lida al bucle"),
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
                    TipusMsgSerial::MCUNivellTriggerCanviat => println!("Nivell de trigger canviat"),
                    TipusMsgSerial::MCUNivellTrigger(_) => (),
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
    let matches = Command::new("OsPlot MCU CLI")
        .about("Un oscil·loscopi fet amb Arduino")
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("El port on està l'Arduino")
                .use_value_delimiter(false)
                .required(true)
                .value_parser(value_parser!(String))
        )
        .get_matches();
    let port_name: &String = matches.get_one("port").unwrap();
    let port_result = Port::nou(port_name);
    if let Err(e) = port_result {
        eprintln!("No s'ha pogut obrir \"{}\". Error: {}", port_name, e);
        std::process::exit(1);
    }
    let port = port_result.unwrap();

    // Tasca del stdin
    let mut stdin = BufReader::new(std::io::stdin());
    let (tx_stdin, mut rx_stdin) = mpsc::channel(8);
    let tx = tx_stdin.clone();
    thread::spawn(move || {
        loop {
            let mut entrada = String::new();
            let bytes_llegits = stdin.read_line(&mut entrada).unwrap();
            tx.blocking_send((entrada, bytes_llegits)).unwrap_or(());
        }
    });
    // Ctrl-C Handler
    ctrlc::set_handler(move || {
        tx_stdin.blocking_send(("".to_string(), 0)).unwrap_or(());
    }).expect("No s'ha pogut instal·lar el handler del Ctrl-C.");

    // Bucle del serial
    let (tx_bucle_serial, rx) = mpsc::channel(8);
    let (tx, mut rx_future_bucle_serial) = oneshot::channel();
    let tasca_bucle_serial = tokio::spawn(async {
        bucle_serial(port, rx).await;
        tx.send(0).expect("S'ha tancat la comunicació entre el bucle i el loop principal.");
    });

    // Bucle de stdin
    loop {
        select! {
            bytes_llegits = rx_stdin.recv() => {
                if bytes_llegits.is_none() {
                    eprintln!("Error inesperat en la stdin.");
                    break;
                }
                let (entrada, bytes_llegits) = bytes_llegits.unwrap_or(("".to_string(), 0));
                if bytes_llegits == 1 {
                    continue;
                }
                else if bytes_llegits == 0 {
                    if let Err(_e) = tx_bucle_serial.send(MsgBucleSerial::ParaTrigger).await {
                        eprintln!("Ja s'ha tancat el bucle del serial");
                    }
                    break;
                }
                let mut ordres = entrada.trim().split(' ');
                let mut msg = MsgBucleSerial::OrdreInvàlida;
                match ordres.next() {
                    Some("inicia") => {
                        msg = MsgBucleSerial::IniciaTrigger;
                    }
                    Some("os") => {
                        if let Some(factor_oversampling) = ordre_os(&mut ordres) {
                            msg = MsgBucleSerial::FactorOversampling(factor_oversampling);
                        }
                    }
                    Some("n") => {
                        if let Some(n_mostres) = ordre_n(&mut ordres) {
                            msg = MsgBucleSerial::NMostres(n_mostres);
                        }
                    }
                    Some("tr") => {
                        if let Some(nivell_trigger) = ordre_tr(&mut ordres) {
                            msg = MsgBucleSerial::NivellTrigger(nivell_trigger);
                        }
                    }
                    Some("surt") => {
                        if tx_bucle_serial.send(MsgBucleSerial::ParaTrigger).await.is_err() {
                            eprintln!("Ja s'ha tancat el bucle del serial");
                        }
                        break;
                    }
                    _ => println!("Ordre invàl·lida"),
                }
                if msg != MsgBucleSerial::OrdreInvàlida {
                    if tx_bucle_serial.send(msg).await.is_err() { break; }
                }
            }
            _ = &mut rx_future_bucle_serial => {
                break;
            }
        }
    }
    if tx_bucle_serial.send(MsgBucleSerial::ParaTrigger).await.is_ok() {
        tasca_bucle_serial.await.unwrap();
    }
}
