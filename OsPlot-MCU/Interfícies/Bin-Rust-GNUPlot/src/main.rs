use std::fs::File;
use std::io::{Write, Read};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::thread::sleep;

use clap::{Arg, Command, value_parser};
use serialport::{TTYPort};

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

    let mut serial_buf: [u8; 1] = [0];
    let mut dades: [u8; 1000] = [0; 1000];
    let mut index_dades:  usize = 0;
    #[cfg(debug_assertions)]
    let mut temps_inici = Instant::now();
    while continua_llegint.load(Ordering::Relaxed) {
        match port.read_exact(&mut serial_buf) {
            Ok(_n) => {}
            Err(e) => {
                if e.kind() != std::io::ErrorKind::TimedOut {
                    eprintln!("{:?}", e);
                    //continua_llegint.store(false, Ordering::Relaxed);
                    break;
                }
            }
        }
        dades[index_dades] = serial_buf[0];
        if serial_buf[0] == BYTE_ESCAPAMENT {
            match port.read_exact(&mut serial_buf) {
                Ok(_n) => {}
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::TimedOut {
                        eprintln!("{:?}", e);
                        //continua_llegint.store(false, Ordering::Relaxed);
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
                let mut file = File::create("osplot.pipe").unwrap();
                file.write_all(&dades[..index_dades]).unwrap();
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
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
