mod network;
mod packet;
mod tilemap;
mod astar;
mod enemy;

use crate::network::Server;
use crate::tilemap::{Tilemap, randomize_rooms};
use std::{thread, time::Duration};

fn main() {
    let duration: Duration = Duration::from_millis(1000);
    let tilemap: Tilemap = Tilemap::from(randomize_rooms(5, 4));
    match Server::new(tilemap) {
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