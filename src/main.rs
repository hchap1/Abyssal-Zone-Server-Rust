mod network;
mod packet;
use crate::network::Server;
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
