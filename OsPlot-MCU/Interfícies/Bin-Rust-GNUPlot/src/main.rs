use std::fs::File;
use std::io::{Write, Read};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::thread::{sleep, self};

use clap::{Arg, Command, value_parser};
use nix::unistd;
use nix::sys::stat;
use tempfile::tempdir;
use serialport::{TTYPort};

#[macro_use]
mod script_plot;

const BAUDRATE: u32 = 1_000_000;
const BYTE_ESCAPAMENT: u8 = 128;

enum MsgCapçaleraPC {
    PCIniciaTrigger = 0,
    PCCanviarFS,
    PCRetornaPossiblesFS,
    PCCanviarNMostres,
    PCRetornaNMostres
}

enum MsgCapçaleraMCU {
    MCUOk = 129,
    MCUError,
    MCURetornaFSS,
    MCURetornaNMostres
}

#[inline(always)]
fn bucle_serial(mut port: TTYPort) {
    let continua_llegint_arc = Arc::new(AtomicBool::new(true));
    let c = continua_llegint_arc.clone();
    ctrlc::set_handler(move || {
        c.clone().store(false, Ordering::Relaxed);
        println!();
    }).expect("Error setting Ctrl-C handler");
    let continua_llegint = continua_llegint_arc.clone();

    // Crea el directori i fitxers temporals
    let dir_temp = tempdir()
        .expect("No s'ha pogut crear el directori temporal");
    let nom_cua = dir_temp.path().join("osplot.pipe");
    unistd::mkfifo(&nom_cua, stat::Mode::S_IRWXU)
        .expect("No s'ha pogut crear la cua");
    // Inicia GNUPlot
    let c = continua_llegint.clone();
    let nom = nom_cua.clone();
    thread::spawn(move || {
        let nom_script = dir_temp.path().join("plot.gnu");
        let mut script = File::create(nom_script.clone())
            .expect("No s'ha pogut obrir el fitxer del script temporal");
        script.write(script_plot!().as_bytes())
            .expect("No s'ha pogut escriure el script de GNUPlot");
        drop(script);
        let mut gnuplot = std::process::Command::new("gnuplot")
            .arg("-e")
            .arg(format!("cua=\"{}\"", nom.to_str().unwrap().to_string()))
            .arg(nom_script)
            .spawn()
            .expect("No s'ha pogut obrir GNUPlot");
        gnuplot.wait().unwrap();
        c.store(false, Ordering::Relaxed);
    });

    let mut serial_buf: [u8; 1] = [0];
    let mut vector_dades: [u8; 1000] = [0; 1000];
    let mut vector_temps: [f32; 1000] = [0.; 1000];
    for c in 0..1000 {
        vector_temps[c] = (c as f32) / 9615.;
    }
    let mut index_dades: usize = 0;
    let nom_cua = nom_cua.to_str().unwrap();
    #[cfg(debug_assertions)]
    let mut temps_inici = Instant::now();
    while continua_llegint.load(Ordering::Relaxed) {
        match port.read_exact(&mut serial_buf) {
            Ok(_n) => {}
            Err(e) => {
                if e.kind() != std::io::ErrorKind::TimedOut {
                    eprintln!("{:?}", e);
                    break;
                }
            }
        }
        vector_dades[index_dades] = serial_buf[0];
        if serial_buf[0] == BYTE_ESCAPAMENT {
            match port.read_exact(&mut serial_buf) {
                Ok(_n) => {}
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::TimedOut {
                        eprintln!("{:?}", e);
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
                let mut file = File::create(nom_cua)
                    .expect("No s'ha pogut obrir la cua");
                for c in 0..index_dades {
                    file.write(&vector_temps[c].to_le_bytes()).unwrap();
                    file.write(&[vector_dades[c]]).unwrap();
                }
                index_dades = 0;
            }
            else { index_dades += 1 }
        }
        else { index_dades += 1 };
    }
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

    let port = serialport::new(port_name, BAUDRATE)
        .timeout(Duration::from_millis(400))
        .open_native();

    match port {
        Ok(mut port) => {
            sleep(Duration::from_secs(5));
            let mut serial_buf: Vec<u8> = vec![0; 1];
            match port.write(&serial_buf) {
                Ok(_n) => {
                    match port.read_exact(&mut serial_buf) {
                        Ok(_t) => {
                            drop(serial_buf);
                            bucle_serial(port);
                        },
                        Err(e) => eprintln!("{:?}", e),
                    }
                },
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Err(e) => {
            eprintln!("No s'ha pogut obrir \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
