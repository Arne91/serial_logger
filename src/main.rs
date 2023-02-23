use chrono::{Datelike, Timelike};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use std::time::Duration;
use std::{env, env::consts::OS as OperationSystem, fs};

fn create_file(fp: &str) -> File {
    let b = std::path::Path::new(fp).exists();

    if b == true {
        OpenOptions::new()
            .append(true)
            .write(true)
            .open(fp)
            .unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(fp)
            .unwrap()
    }
}
fn create_folder(fp: &str) -> std::io::Result<()> {
    let b = std::path::Path::new(fp).exists();

    if b == true {
        Ok(())
    } else {
        fs::create_dir_all(fp)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    if OperationSystem != "linux"{
        panic!("System is not Linux!");
    }
    let args: Vec<String> = env::args().collect();
    let serial_path: &String;
    let baud_rate;
    match args.len() {
        0 => panic!("Unmöglich"),
        1 | 2 => panic!("Zu wenige Parameter!"),
        3 => {
            serial_path = &args[1];
            baud_rate = args[2].parse::<u32>().unwrap();
        }
        _ => panic!("Zu viele Parameter"),
    }
    let binding = home::home_dir().unwrap();
    let home_path = binding.as_os_str().to_str().unwrap();

    let date = chrono::offset::Local::now();
    let hour_minute = format!("{:02}{:02}", date.time().hour(), date.time().minute());

    let fp = format!(
        "{}/log_files/{:04}/{:04}-{:02}/{:04}-{:02}-{:02}",
        home_path,
        date.year(),
        date.year(),
        date.month(),
        date.year(),
        date.month(),
        date.day()
    );
    let file_name = format!("{}/{}.log", fp, hour_minute);
    create_folder(&fp)?;
    println!("Log to --> {}", file_name);
    let mut file = create_file(&file_name);
    let file_header = format!("{}\n", date.to_rfc2822());
    file.write(file_header.as_bytes())?;

    loop {
        let port = serialport::new(serial_path, baud_rate)
            .timeout(Duration::from_secs(10))
            .open();
        match port {
            Ok(port) => {
                let mut port = BufReader::new(port);
                let mut line_buffer = String::new();

                'inner: loop {
                    line_buffer.clear();
                    let res = port.read_line(&mut line_buffer);
                    match res {
                        Ok(_) => {
                            print!("{}", line_buffer);
                            if line_buffer.as_bytes()[0] == 0x0D{
                                file.write(line_buffer[1..].as_bytes())?;   // delete the first byte. It contains a line feed. We don't need that on linux
                            } else {
                                file.write(line_buffer.as_bytes())?;
                            }
                        }
                        Err(_) => {
                            // when there was a timeout, because nothing came on the serial port:
                            // goto the outer loop and open a new port to listen to.
                            break 'inner;
                        }
                    }
                }
            }

            Err(e) => {
                eprintln!("error {}", e);
            }
        }
    }
}
