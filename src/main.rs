mod network;
mod packet;
mod tilemap;
use crate::network::Server;
use crate::tilemap::Room;
use std::{thread, time::Duration};

fn main() {
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

fn not_main() {
    let test_room: Room = 0.into();
    println!("Created new Room!\n{:?}", test_room.tilemap);
}
