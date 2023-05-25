use std::fs::File;
use std::io::{Write, self, BufRead};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use clap::{Arg, Command, value_parser};

use nix::sys::signal::{Signal, self};
use nix::unistd::{self, Pid};
use nix::sys::stat;
use tempfile::tempdir;

#[macro_use]
mod script_plot;
mod missatges_mcu;
use missatges_mcu::*;
mod missatges_bucle;
use missatges_bucle::*;
mod parser;
use parser::*;

const MIDA_BUFFERS: usize = 4096;

#[inline(always)]
fn bucle_serial(mut port: Port, rx_bucle_serial: Receiver<MsgBucleSerial>) {
    let fs = port.retorna_fs()
        .expect("No s'ha pogut llegir la freqüència de mostreig");
    let mut factor_oversampling = port.retorna_factor_oversampling()
        .expect("No s'ha pogut obtenir el factor d'oversampling inicial");
    let mut n_mostres = 0;

    if let Some(e) = port.inicia_trigger() {
        panic!("Error en iniciar el trigger: {:?}", e);
    }
    println!("Fs: {}, Oversampling: {}", fs, factor_oversampling);

    // GNUPlot
    // Crea el directori i la cua
    let dir_temp = tempdir()
        .expect("No s'ha pogut crear el directori temporal");
    let nom_cua = dir_temp.path().join("osplot.pipe");
    unistd::mkfifo(&nom_cua, stat::Mode::S_IRWXU)
        .expect("No s'ha pogut crear la cua");
    let nom_script = dir_temp.path().join("plot.gnu");
    let mut script = File::create(nom_script.clone())
        .expect("No s'ha pogut obrir el fitxer del script temporal");
    script.write(script_plot!().as_bytes())
            .expect("No s'ha pogut escriure el script de GNUPlot");
    drop(script);

    let mut gnuplot = std::process::Command::new("gnuplot")
        .arg("-e")
        .arg(format!("cua=\"{}\"", nom_cua.as_path().display().to_string()))
        .arg(nom_script)
        .spawn()
        .expect("No s'ha pogut obrir GNUPlot");

    let mut serial_buf = [0u8];
    let mut vector_dades = [0u8; MIDA_BUFFERS];
    let mut vector_temps = [0f32; MIDA_BUFFERS];
    for c in 0..MIDA_BUFFERS {
        vector_temps[c] = (c as f32) * (factor_oversampling as f32) / fs;
    }
    let mut index_dades: usize = 0;
    let nom_cua = nom_cua.to_str().unwrap();
    #[cfg(debug_assertions)]
    let mut temps_inici = Instant::now();
    let mut temps_frames = Instant::now();
    loop {
        match rx_bucle_serial.try_recv() {
            Ok(msg) => {
                match msg  {
                    MsgBucleSerial::Altres(AltresMsgBucleSerial::ParaLlegir) => break,
                    MsgBucleSerial::FactorOversampling(fo) => {
                        factor_oversampling = fo;
                        for c in 0..MIDA_BUFFERS {
                            vector_temps[c] = (c as f32) * (factor_oversampling as f32) / fs;
                        }
                        if let Some(e) = port.modifica_factor_oversampling(factor_oversampling) {
                            //println!("Error en modificar el factor d'oversampling: {}", e);
                        }
                    },
                    MsgBucleSerial::NMostres(n) => {
                        n_mostres = n;
                        if let Some(e) = port.modifica_n_mostres(n_mostres) {
                            //println!("Error en modificar el nombre de mostres: {}", e);
                        }
                    }
                }
                index_dades = 0;
                port.inicia_trigger();
            }
            Err(e) => {
                if e != std::sync::mpsc::TryRecvError::Empty {
                    eprintln!("Error al rebre missatge pel canal: {:?}", e);
                    break;
                }
            }
        }
        match port.llegeix_1(&mut serial_buf) {
            Ok(_n) => {}
            Err(e) => {
                if e.kind() != std::io::ErrorKind::TimedOut {
                    eprintln!("Error al llegir byte del port sèrie: {:?}", e);
                    break;
                }
            }
        }
        vector_dades[index_dades] = serial_buf[0];
        if serial_buf[0] == BYTE_ESCAPAMENT {
            match port.llegeix_1(&mut serial_buf) {
                Ok(_n) => {}
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::TimedOut {
                        eprintln!("Error al llegir byte del port sèrie: {:?}", e);
                        break;
                    }
                }
            }
            if serial_buf[0] != BYTE_ESCAPAMENT {
                #[cfg(debug_assertions)]
                {
                    println!("Mostres: {} {:?}", index_dades,
                    temps_inici.elapsed().as_millis());
                    temps_inici = Instant::now();
                }
                if temps_frames.elapsed() > Duration::from_millis(17) {
                    temps_frames = Instant::now();
                    let mut file = File::create(nom_cua)
                        .expect("No s'ha pogut obrir la cua");
                    for c in 0..index_dades {
                        file.write(&vector_temps[c].to_le_bytes()).unwrap();
                        file.write(&[vector_dades[c]]).unwrap();
                    }
                }
                
                index_dades = 0;
            }
            else { index_dades += 1 }
        }
        else { index_dades += 1 };
    }
    if gnuplot.try_wait().unwrap().is_none() {
        signal::kill(Pid::from_raw(gnuplot.id().try_into().unwrap()), Signal::SIGINT).unwrap();
    }
    gnuplot.wait().unwrap();
}

struct MsgStdin {
    str: String,
    bytes_llegits: usize
}

fn main() {
    let matches = Command::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("The device path to a serial port")
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

    // Crea els canals de comunicació entre fils
    let (tx_bucle_serial, rx_bucle_serial) = channel();
    let (tx_stdin, rx_stdin) = channel();

    // Registra un callback per l'esdeveniment de Ctrl-C.
    let tx = tx_stdin.clone();
    ctrlc::set_handler(move || {
        if tx.send(MsgStdin { str: String::new(), bytes_llegits: 0 }).is_err() {
            eprintln!("No es pot enviar l'entrada");
        }
    }).expect("Error en registrar l'esdeveniment de Ctrl-C.");

    
    let th_bucle_serial = thread::spawn(move || {
        bucle_serial(port_result.unwrap(), rx_bucle_serial);
    });

    thread::spawn(move || {
        let stdin = io::stdin();
        loop {
            let mut entrada = String::new();
            let bytes_llegits = stdin.lock().read_line(&mut entrada).unwrap();
            if tx_stdin.send(MsgStdin { str: entrada, bytes_llegits }).is_err() {
                eprintln!("No es pot enviar l'entrada");
                break;
            }
            if bytes_llegits == 0 { break; }
        }
    });
    
    loop {
        let msg_stdin = rx_stdin.recv();
        if msg_stdin.is_err() {
            eprintln!("Ha mort el thread de stdin, abortant");
            tx_bucle_serial.send(MsgBucleSerial::Altres(AltresMsgBucleSerial::ParaLlegir)).unwrap();
            break;
        }
        let msg_stdin = msg_stdin.unwrap();
        let (entrada, bytes_llegits) = (msg_stdin.str, msg_stdin.bytes_llegits);
        if bytes_llegits == 1 {
            continue;
        }
        else if bytes_llegits == 0 {
            if let Err(e) = tx_bucle_serial.send(MsgBucleSerial::Altres(AltresMsgBucleSerial::ParaLlegir)) {
                eprintln!("Ja s'ha tancat el bucle del serial");
            }
            break;
        }

        let mut ordres = entrada.trim().split(' ');
        match ordres.next() {
            Some("os") => {
                if let Some(factor_oversampling) = ordre_os(&mut ordres) {
                    let msg: MsgBucleSerial = MsgBucleSerial::FactorOversampling(factor_oversampling);
                    if tx_bucle_serial.send(msg).is_err() { break; }
                }
            }
            Some("n") => {
                if let Some(n_mostres) = ordre_n(&mut ordres) {
                    let msg: MsgBucleSerial = MsgBucleSerial::NMostres(n_mostres);
                    if tx_bucle_serial.send(msg).is_err() { break; }
                }
            }
            Some("surt") => {
                if tx_bucle_serial.send(MsgBucleSerial::Altres(AltresMsgBucleSerial::ParaLlegir)).is_err() {
                    eprintln!("Ja s'ha tancat el bucle del serial");
                }
                break;
            }
            _ => println!("Ordre invàl·lida"),
        }
    }
    th_bucle_serial.join().unwrap();
}
