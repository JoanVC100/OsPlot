use std::fs::File;
use std::i128::MAX;
use std::io::{Write, Read};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::thread::{sleep, self};

use clap::{Arg, Command, value_parser};

use nix::unistd;
use nix::sys::stat;
use tempfile::tempdir;
use serialport::{TTYPort};

#[macro_use]
mod script_plot;
mod missatges_mcu;
use missatges_mcu::*;
mod missatges_bucle;
use missatges_bucle::*;

const BAUDRATE: u32 = 1_000_000;
const BYTE_ESCAPAMENT: u8 = 128;
type FreqMostreig = u32;

#[inline(always)]
fn bucle_serial(mut port: TTYPort, mut fs: FreqMostreig) {
    // Emmagatzemar la freqüència de mostreig com un float.
    let mut fs: f32 = fs as f32;

    // Crea els canals de comunicació entre fils
    let (tx_bucle_serial, rx_bucle_serial) = channel();

    // Registra un callback per l'esdeveniment de Ctrl-C.
    let tx = tx_bucle_serial.clone();
    ctrlc::set_handler(move || {
        let msg: MsgBucleSerial = MsgBucleSerial::default();
        tx.send(msg)
            .expect("S'ha tancat la comunicació amb el bucle serial. No s'ha pogut avisar el Ctrl-C.");
    }).expect("Error en registrar l'esdeveniment de Ctrl-C.");

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
    
    // Inicia GNUPlot
    let nom = nom_cua.clone();
    thread::spawn(move || {
        let mut gnuplot = std::process::Command::new("gnuplot")
            .arg("-e")
            .arg(format!("cua=\"{}\"", nom.as_path().display().to_string()))
            .arg(nom_script)
            .spawn()
            .expect("No s'ha pogut obrir GNUPlot");
        gnuplot.wait().unwrap();
        let msg: MsgBucleSerial = MsgBucleSerial::default();
        tx_bucle_serial.send(msg)
            .expect("S'ha tancat la comunicació amb el bucle serial. No s'ha pogut avisar la mort de GNUPlot.");
    });

    let mut serial_buf: [u8; 1] = [0];
    let mut vector_dades: [u8; 1000] = [0; 1000];
    let mut vector_temps: [f32; 1000] = [0.; 1000];
    for c in 0..1000 {
        vector_temps[c] = (c as f32) / fs;
    }
    let mut index_dades: usize = 0;
    let nom_cua = nom_cua.to_str().unwrap();
    #[cfg(debug_assertions)]
    let mut temps_inici = Instant::now();
    loop {
        match rx_bucle_serial.try_recv() {
            Ok(msg) => {
                match msg.capçalera  {
                    CapMsgBucleSerial::ParaLlegir => break
                }
            }
            Err(e) => {
                if e != std::sync::mpsc::TryRecvError::Empty {
                    eprintln!("Error al rebre missatge pel canal: {:?}", e);
                    break;
                }
            }
        }
        match port.read_exact(&mut serial_buf) {
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
            match port.read_exact(&mut serial_buf) {
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
            sleep(Duration::from_secs(3));
            let serial_buf = [MsgCapçaleraPC::PCRetornaPossiblesFS as u8; 1];
            match port.write(&serial_buf) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error en intentar demanar les freqüencies suportades: {:?}", e);
                    return;
                }
            };
            sleep(Duration::from_millis(1000));
            let mut serial_buf = [0; 1];
            match port.read(&mut serial_buf) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error en intentar demanar les freqüencies suportades: {:?}", e);
                    return;
                }
            };
            let mut serial_buf = [0 as u8; MAXIM_FSS*4];
            let n_fs = match port.read(&mut serial_buf) {
                Ok(n) => (n / 4) - 1,
                Err(e) => {
                    eprintln!("Error en rebre les freqüències de mostreig: {:?}", e);
                    0
                }
            };
            let mut fss: Vec<FreqMostreig> = Vec::with_capacity(n_fs);
            for ch in serial_buf.chunks(4) {
                match ((ch[3] as FreqMostreig) << 24) + ((ch[2] as FreqMostreig) << 16) + ((ch[1] as FreqMostreig) << 8) + ch[0] as FreqMostreig {
                    0 => break,
                    fs => fss.push(fs)
                }
            }
            let mut serial_buf: Vec<u8> = vec![0; 1];
            match port.write(&serial_buf) {
                Ok(_n) => {
                    match port.read_exact(&mut serial_buf) {
                        Ok(_t) => {
                            drop(serial_buf);
                            bucle_serial(port, fss[0]);
                        },
                        Err(e) => eprintln!("Error en rebre la confirmació d'inici del trigger: {:?}", e),
                    }
                },
                Err(e) => eprintln!("Error en iniciar el trigger: {:?}", e),
            }
        }
        Err(e) => {
            eprintln!("No s'ha pogut obrir \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
