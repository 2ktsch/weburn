use std::cmp::PartialEq;
use std::env;
use std::error::Error;
use std::net::{IpAddr};
use std::str::{from_utf8, FromStr};
use std::string::ToString;
use std::time::{Duration};
use serial2::{SerialPort};
use reqwest;
use reqwest::Client;
use md5;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

// These are from the new lightburn settings
const FOCUS: &str = "M13X_Y_\n"; // [u8; 13] = [77, 49, 51, 48, 88, 50, 48, 48, 89, 49, 53, 48, 10];
const GET_POSITION: &str = "?\n"; // [u8; 2] = [63, 10]);
const RED_DOT_ON: &str = "M21S1\n"; // [u8; 6] = [77, 50, 49, 83, 49, 10];
const RED_DOT_OFF: &str = "M21S0\n"; // [u8; 6] = [77, 50, 49, 83, 48, 10];
const STOP: [u8; 1] = [24]; // [Cancel]
const PAUSE: &str = "!"; // [u8; 1] = [33];
const RESUME: &str = "~"; // [u8; 1] = [126];
const ENABLE_UMODE: &str = "M16\n"; // [u8; 4] = [77, 49, 54, 10];
const DISABLE_UMODE: &str = "M17\n"; // [u8; 4] = [77, 49, 55, 10];
const END_GCODE: &str = "M6\n";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {

    let laser_port = env::var("PORT").unwrap_or(8080.to_string()).parse::<u16>().unwrap_or(8080u16);

    // This is intended for grouping the gcode;
    // the buffer in the machine is consumed faster than the individual http requests can be sent (which is why I did bulk mode)
    let _buffer_size = env::var("BUFFER_SIZE").unwrap_or(1024.to_string()).parse::<u32>().unwrap_or(1024);

    let full_cancel: bool = env::var("AUTO_CANCEL_BULK_END").unwrap_or("false".to_string()).parse::<bool>().unwrap_or(false);

    let debug: bool = env::var("DEBUG").unwrap_or("false".to_string()).parse::<bool>().unwrap_or(false);

    let mut mode = "passthrough";
    println!("running in {} mode", mode);

    let laser_ip = env::var("LASER_IP").expect("Laser ip address must be set... how else do you thing we'll connect?");
    if IpAddr::from_str(laser_ip.as_str()).is_err() {
        panic!("Need a valid ip address to communicate with the laser!")
    }

    let serial_port = env::var("SERIAL_PORT").expect("Can't really guess the serial port");

    let wecreat_client = Client::builder().build()?;
    let mut port = SerialPort::open(serial_port, 2000000)?;
    let _ = port.set_read_timeout(Duration::from_secs(u64::MAX));
    let mut buffer  = [0; 256];
    port.flush().expect("");
    fn send_ok(port: &SerialPort) {
        let _ = port.write("ok\n".as_bytes()).expect("error writing to serial port");
    }
    loop {
        match mode {
            "passthrough" => {
                println!("ready to go!");
                loop {
                    let read = port.read(&mut buffer)?;
                    let msg = &buffer[..read];
                    let body = msg.to_vec().clone();
                    if debug { print!("{}", from_utf8(body.as_slice()).unwrap()); }
                    if from_utf8(body.as_slice()).unwrap().contains("M14S0") {
                        send_ok(&port);
                        mode = "bulk";
                        break;
                    }
                    let sum = format!("{:x}", md5::compute(body.as_slice())).to_uppercase();
                    print!("{:?}", body.as_slice());
                    let res = wecreat_client.post(format!("http://{}:{}/test/cmd/mcu?md5={}", laser_ip, laser_port, sum)).body(body).send().await?;

                    let j: WeCreatCmdResponse = res.json().await.unwrap();
                    match j.code {
                        0 => {
                            send_ok(&port);
                            println!("{:?}", j)
                        },
                        _ => {}
                    }
                }
            },
            "bulk" => {
                println!("now in bulk mode");
                // let mut modo = Modes::Connecting;
                let mut modo = Modes::Receiving;
                loop {
                    if debug { println!(); }
                    if debug { println!("Getting status..."); }
                    let res = wecreat_client.post(format!("http://{}:{}/process/status", laser_ip, laser_port)).send().await?;
                    let m: WeCreatStatusResponse = res.json().await.unwrap();
                    if debug { println!("Machine responded: {:?}", m); }
                    // if modo != Modes::Connecting {
                    //     modo = match m.status {
                    //         2 =>  Modes::Processing,
                    //         3 => Modes::Processing,
                    //         _ => Modes::Receiving
                    //     };
                    // }
                    match modo {
                        Modes::_Connecting => {
                            loop {
                                println!("Waiting for serial from LightBurn");
                                let read = port.read(&mut buffer)?;
                                let msg = &buffer[..read];
                                let cuerpo = msg.to_vec().clone();
                                if from_utf8(&*cuerpo).unwrap().contains("G0\n") {
                                    println!("G0 {:?}", cuerpo);
                                    send_ok(&port);
                                    sleep(Duration::from_millis(500)).await;
                                    send_ok(&port);
                                } else if from_utf8(&*cuerpo).unwrap().contains("$I\n") {
                                    send_ok(&port);
                                    println!("replying 'ok'");
                                    modo = Modes::Receiving;
                                    break;
                                }
                            }
                        },
                        Modes::Receiving => {
                            let mut body: Vec<u8> = vec![];
                            println!("Receiving serial data from LightBurn");
                            loop {
                                let read = port.read(&mut buffer)?;
                                let msg = &buffer[..read];
                                let cuerpo = msg.to_vec().clone();
                                cuerpo.iter().for_each(|x| { body.push(*x) });
                                send_ok(&port);
                                // print!("\r{} bytes captured.", body.len());
                                if from_utf8(cuerpo.as_slice()).expect("wtf?").contains("M2") {
                                    println!("Looks like that's it.");
                                    // println!("{} bytes captured.", body.len());
                                    // modo = Modes::Processing;
                                    break;
                                }
                                if from_utf8(cuerpo.as_slice()).expect("wtf?").contains("M6") {
                                    println!("Looks like that's it.");
                                    // println!("{} bytes captured.", body.len());
                                    // modo = Modes::Processing;
                                    break;
                                }
                                // TODO refactor into a multithreaded program! :) this was a quick hack
                                // if from_utf8(cuerpo.as_slice()).expect("wtf?").contains(from_utf8(&[24]).unwrap()) {
                                //     println!("I guess we're done getting GCODE.");
                                //     println!("{} bytes captured.", body.len());
                                //     // modo = Modes::Processing;
                                //     break;
                                // }
                            }
                            let sum = format!("{:x}", md5::compute(body.as_slice())).to_uppercase();
                            println!("Got {} bytes, md5: {}", body.len(), sum);
                            if debug { println!("bulk sending gcode:\n{:?}", from_utf8(&body.to_ascii_uppercase()).unwrap_or("")); }
                            let res = wecreat_client.post(format!("http://{}:{}/process/upload?md5={}", laser_ip, laser_port, sum)).body(body).send().await?;
                            let j: WeCreatUploadResponse = res.json().await.unwrap();
                            println!("{:?}", j);
                            if j.code == 0 {
                                modo = Modes::WaitingForUser;
                                println!("Sent! Remember to press the button so it starts :)");
                            } else {
                                let _ = port.write("error\n".as_bytes());
                                modo = Modes::Receiving;
                            }
                        },
                        Modes::WaitingForUser => {
                            loop {
                                let res = wecreat_client.post(format!("http://{}:{}/process/status", laser_ip, laser_port)).send().await?;
                                let l: WeCreatStatusResponse = res.json().await.unwrap();
                                if debug { print!("\rstatus after code 2: {:?}", l); }
                                if l.status == 2 {
                                    modo = Modes::Processing;
                                    if debug { println!(); }
                                    break;
                                }
                                let _ = sleep(Duration::from_secs(1)).await;
                            }
                        }
                        Modes::Processing => {
                            println!("Thanks for pressing the button!");
                            loop {
                                let res = wecreat_client.post(format!("http://{}:{}/process/status", laser_ip, laser_port)).send().await?;
                                let l: WeCreatStatusResponse = res.json().await.unwrap();
                                if debug { print!("\rstatus after code 2: {:?}", l); }
                                match l.status {
                                    2 => (),
                                    3 => (),
                                    _ => {
                                        modo = Modes::Done;
                                        break;
                                    }
                                };
                                let _ = sleep(Duration::from_secs(5)).await;
                            }
                        },
                        Modes::Done => {
                            let _ = sleep(Duration::from_secs(5)).await;
                            println!("Done! exiting bulk mode");
                            if full_cancel { let _res = wecreat_client.post(format!("http://{}:{}/process/control?action=2", laser_ip, laser_port)).send().await?; }
                            // modo = Modes::Receiving;
                            mode = "passthrough";
                            break;
                        }
                    }
                }
            }
            _ => ()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct WeCreatCmdResponse {
    result: isize,
    data: Option<String>,
    code: isize
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct WeCreatUploadResponse {
    result: Option<String>,
    data: Option<String>,
    code: isize
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct WeCreatStatusResponse {
    code: isize,
    result: Option<String>,
    status: isize
}

#[derive(PartialEq)]
enum Modes {
    _Connecting,
    Receiving,
    WaitingForUser,
    Processing,
    Done,
}


// The idea behind this is to send position updates back to Lightburn, but I don't know if the WeCreat Vision
// a. sends feedback or b. when to send it
// (useful in bulk, but it interrupts the microcontroller too much.)
fn _translate_position(k: &str) -> String {
    if k.len() > 15usize {
        let d = k.replace("M27 ", "").replace("\n", "");
        let x = d.find("X").unwrap_or(0);
        let xc = &d[x+1..].find(",").unwrap_or(d.len()) + x;
        println!("x: {}, {}", x, xc);
        let y = d.find("Y").unwrap_or(0);
        let yc = &d[y+1..].find(",").unwrap_or(d.len()) + y;
        println!("x: {}, {}", y, yc);
        let z = d.find("Z").unwrap_or(0);
        let zc = &d[z+1..].find(",").unwrap_or(d.len()) + z;
        println!("x: {}, {}", z, zc);
        let pos = format!("<IDLE,Mpos:{},{},{},Wpos:0.000,0.000,0.000>", &d[x..xc], &d[y..yc], &d[z..zc]);
        println!("{:?}", pos);
        return pos;
    }
    String::new()
}