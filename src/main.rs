mod network;
mod packet;
mod tilemap;
mod astar;
mod enemy;
mod util;

use crate::network::Server;
use crate::tilemap::{Tilemap, randomize_rooms};
use std::{thread, time::Duration};

fn main() {
    let duration: Duration = Duration::from_millis(1000);
    println!("Beginning...");
    let tilemap: Tilemap = Tilemap::from(randomize_rooms(4, 6));
    match Server::new(tilemap) {
        Ok(server) => {
            {
                println!("LAN JOINCODE: {}", server.lock().unwrap().get_joincode());
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