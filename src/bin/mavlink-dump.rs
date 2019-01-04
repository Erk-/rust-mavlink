extern crate mavlink;
use std::sync::Arc;
use std::thread;
use std::env;
use std::time::Duration;

use std::fs;
use std::fs::File;

fn main() {
    let path = "test.tlog";
    let mut f = File::open(path).unwrap();

    loop {
        match mavlink::read(&mut f) {
            Ok((_,msg)) => {
                println!("{:#?}",msg);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
    
    
    /*
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: mavlink-dump (tcp|udpin|udpout|serial):(ip|dev):(port|baud)");
        return;
    }

    let vehicle = Arc::new(mavlink::connect(&args[1]).unwrap());
    
    vehicle.send(&mavlink::request_parameters()).unwrap();
    vehicle.send(&mavlink::request_stream()).unwrap();

    thread::spawn({
        let vehicle = vehicle.clone();
        move || {
            loop {
                vehicle.send(&mavlink::heartbeat_message()).ok();
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    loop {
        if let Ok(msg) = vehicle.recv() {
            println!("{:?}", msg);
        } else {
            break;
        }
    }
    */
}