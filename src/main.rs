mod network;
mod packet;
mod tilemap;

use crate::network::Server;
use crate::tilemap::{Tilemap, Room, randomize_rooms};
use std::{thread, time::Duration};

fn unused_main() {
    let duration: Duration = Duration::from_millis(1000);
    match Server::new() {
        Ok(server) => {
            {
                println!("JOINCODE: {}", server.lock().unwrap().get_joincode());
            }
            loop {
                thread::sleep(duration);
            }
        }
        Err(e) => {
            println!("ERROR: {:?}", e);
        }
    }
}

fn main() {
    println!("Running program...");
    const SIZE: usize = 16;
    println!("SIZE = {SIZE}");
    randomize_rooms::<SIZE>();
}
