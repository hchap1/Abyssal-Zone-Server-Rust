use core::fmt;
use std::fs::read_to_string;
use std::fmt::Display;
use rand::Rng;

pub struct Tilemap<const SIZE: usize> {
    pub tilemap: Vec<Vec<usize>>,
    spawn_room: [usize; 2],
    spawn_coordinates: [usize; 2],
}

impl<const SIZE: usize> Tilemap<SIZE> {
    pub fn new() -> Self {
        let rooms: [[Room; SIZE]; SIZE] = randomize_rooms();
        let mut tilemap = Tilemap { 
            tilemap: vec![vec![0; 64]; 64], 
            spawn_room: [1, 1], 
            spawn_coordinates: [0, 0] 
        };
        for (i, row) in rooms.iter().enumerate() {
            for (j, room) in row.iter().enumerate() {
                let start_row = i * 16;
                let start_col = j * 16;
                for (ti, tile_row) in room.tilemap.iter().enumerate() {
                    tilemap.tilemap[start_row + ti][start_col..start_col + 16].copy_from_slice(tile_row);
                }
            }
        }
        tilemap
    }
}

#[derive(Clone, Copy)]
pub struct Room {
    pub tilemap: [[usize; 16]; 16],
}

impl Room {
    fn new() -> Self {
        Room { tilemap: [[0; 16]; 16] }
    }
}

impl From<usize> for Room {
    fn from(id: usize) -> Self {
        let mut room = Room { tilemap: [[0; 16]; 16] };
        let filename = format!("assets/levels/{}.tilemap", id);
        match read_to_string(&filename) {
            Ok(file_string) => {
                let lines: Vec<Vec<usize>> = file_string
                    .lines()
                    .map(|line| line.split(' ').map(|x| x.parse::<usize>().unwrap_or(0)).collect())
                    .collect();
                for (y, line) in lines.iter().enumerate() {
                    for (x, val) in line.iter().enumerate() {
                        room.tilemap[y][x] = *val;
                    }
                }
            }
            Err(e) => {
                eprintln!("ERROR! {}", e);
            }
        }
        room
    }
}

pub fn randomize_rooms<const SIZE: usize>() -> [[Room; SIZE]; SIZE] {
    println!("Starting...");
    let mut rng = rand::thread_rng();
    let mut array: [[Room; SIZE]; SIZE] = [[Room::new(); SIZE]; SIZE];
    println!("Array initalized.");
    for r in 0..SIZE {
        for t in 0..SIZE {
            let rand_room: usize = rng.gen_range(1..3);
        }
    }
    array
}
