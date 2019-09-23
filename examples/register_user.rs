extern crate philipshue;

use std::env;
use std::thread;
use std::time::Duration;

use philipshue::bridge;
use philipshue::errors::{HueError, HueErrorKind, BridgeError};

mod discover;
use discover::discover;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage : {:?} <devicetype>", args[0]);
    } else {
        let ip = discover().pop().unwrap();

        loop {
            match bridge::register_user(&ip, &args.remove(1)) {
                Ok(bridge) => {
                    println!("User registered: {}, on IP: {}", bridge, ip);
                    break;
                }
                Err(HueError(HueErrorKind::BridgeError { error: BridgeError::LinkButtonNotPressed, .. }, _)) => {
                    println!("Please, press the link on the bridge. Retrying in 5 seconds");
                    thread::sleep(Duration::from_secs(5));
                }
                Err(e) => {
                    println!("Unexpected error occured: {}", e);
                    return;
                }
            }
        }
    }
}
