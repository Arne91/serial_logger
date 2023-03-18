use chrono::{Datelike, Timelike, DateTime};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use std::time::Duration;
use std::{env, env::consts::OS as OperationSystem, fs};

/// create the file if it doesn't exist.
/// 
/// If the file doesn't exist, the file is created. Additionally it gives a boolean back with true.
/// If the file is created, the boolean is true. If it is not created, it returns a false.
/// It also returns the file descriptor.
fn create_file(fp: &str) -> (File, bool) {
    let b = std::path::Path::new(fp).exists();

    if b == true {
        (OpenOptions::new()
            .append(true)
            .write(true)
            .open(fp)
            .unwrap(), false)
    } else {
        (OpenOptions::new()
            .write(true)
            .create(true)
            .open(fp)
            .unwrap(), true)
    }
}

/// create the folder if it doesn't exist.
/// 
/// If the folder doesn't exist, the folder will be created.
fn create_folder(fp: &str) -> std::io::Result<()> {
    let b = std::path::Path::new(fp).exists();

    if b == true {
        Ok(())
    } else {
        fs::create_dir_all(fp)?;
        Ok(())
    }
}

// fn get_time<Tz: TimeZone>(date: DateTime<Tz>, date_str: &mut String){
//     let tmp_date = format!("{:02}:{:02}:{:02}.{:02}",date.time().hour(),date.time().minute(),date.time().second(),date.time().nanosecond()/1000000).as_str();
//     date_str = &mut tmp_date.to_string();
// }

fn main() -> std::io::Result<()> {
    if OperationSystem != "linux"{
        panic!("System is not Linux!");
    }
    let args: Vec<String> = env::args().collect();
    let serial_path: &String;
    let baud_rate;
    let mut log_timestamp = true;
    match args.len() {
        0=> panic!("impossible"),
        1 => {
            println!("Usage: serial_logger <PATH TO SERIAL DEV> <BAUDRATE> (optional: <LOG WITH TIMESTAMP (default: true)> <OUTPUT_PATH> (default: home directory of user))");
            return Ok(());
        }
        3 | 4 | 5 => {
            serial_path = &args[1];
            baud_rate = args[2].parse::<u32>().unwrap();
            if args.len() > 3 {
                log_timestamp = args[3].parse::<bool>().unwrap();
            }
        }
        2|_ => panic!("Too less parameters"),
    }
    
    let binding = home::home_dir().unwrap();
    let home_path: &str;

    if args.len() > 4 {
        home_path = &args[4].as_str();
    }
    else {
        home_path = binding.as_os_str().to_str().unwrap();
    }
    
    let get_time = |date: DateTime<chrono::Local>| ->String{
        format!("{:02}:{:02}:{:02}.{:03}\t",date.time().hour(),date.time().minute(),date.time().second(),date.time().nanosecond()/1000000)
    };


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
    let (mut file, is_created) = create_file(&file_name);
    let file_header = format!("{}\n", date.to_rfc2822());
    
    // only if the file is created, the file header should be written
    if is_created{
        file.write(file_header.as_bytes())?;
    }

    let is_readable = |x| -> bool {
        match x {
            0x0A|0x20..=0x7E => true,
            _ => false,
        }
    };
    loop {
        let port = serialport::new(serial_path, baud_rate)
            .timeout(Duration::from_secs(10000))
            .open();
        match port {
            Ok(port) => {
                let mut port = BufReader::new(port);
                let mut line_buffer = String::new();

                'inner: loop {
                    line_buffer.clear();
                    let res = port.read_line(&mut line_buffer);
                    if let Ok(_) = res {
                        print!("{}", line_buffer);
                        if log_timestamp {
                            file.write(get_time(chrono::offset::Local::now()).as_bytes())?;
                        }
                        
                        let mut write_arr = Vec::new();
                        for ch in line_buffer.as_bytes(){
                            if is_readable(*ch){
                                write_arr.push(*ch);
                            }
                        }
                        file.write(&write_arr)?;
                    } else {
                        // when there was a timeout, because nothing came on the serial port:
                        // goto the outer loop and open a new port to listen to.
                        break 'inner;
                    }
                }
            }

            Err(e) => {
                eprintln!("error {}", e);
            }
        }
    }
}
