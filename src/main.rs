mod network;
mod packet;
mod tilemap;
mod astar;

use tilemap::Room;

use crate::network::Server;
use crate::tilemap::{Tilemap, randomize_rooms};
use crate::astar::{astar, Position};
use std::{thread, time::Duration};

fn unused_main() {
    let duration: Duration = Duration::from_millis(1000);
    let tilemap: Tilemap = Tilemap::from(randomize_rooms(5, 3));
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

fn main() {
    let tilemap: Vec<Vec<usize>> = vec![
        vec![2,2,2,2,2],
        vec![2,2,2,2,2],
        vec![4,4,4,4,2],
        vec![2,2,2,2,2],
        vec![2,2,2,2,2]
    ];
    let start: Position = Position::new(0, 0);
    let end: Position = Position::new(0, 4);
    let path: Option<Vec<Position>> = astar(&tilemap, start, end);
    if let Some(positions) = path {
        for position in positions.iter() {
            println!("{}", position.pretty_print());
        }
    }
    else {
        println!("no path...");
    }
}