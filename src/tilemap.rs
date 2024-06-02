use core::fmt;
use std::fs::read_to_string;
use std::fmt::Display;
use rand::Rng;

pub struct Tilemap {
    pub tilemap: [[usize; 64]; 64],
    spawn_room: [usize; 2],
    spawn_coordinates: [usize; 2]
}

impl Tilemap {
    fn new<const SIZE: usize>() {
        let mut r_index: usize = 0;
        let mut t_index: usize = 0;
        let rooms: [[Room; SIZE]; SIZE] = randomize_rooms();
        let mut tilemap: Tilemap = Tilemap { tilemap: [[0; 64]; 64], spawn_room: [1, 1], spawn_coordinates: [0, 0] };
        for row_of_rooms in &rooms {
            for row in 0..SIZE {
                for room in row_of_rooms {
                    for item in room.tilemap[row] {
                        tilemap.tilemap[r_index][t_index] = item;
                        r_index += 1;
                    }
                }
                r_index += 1;
                t_index = 0;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Room {
    pub tilemap: [[usize; 16]; 16]
}

impl Room {
    fn new() -> Self{
        return Room {tilemap: [[0; 16]; 16] };
    }
}

impl From<usize> for Room {
    fn from(id: usize) -> Self {
        let mut room: Room = Room { tilemap: [[0; 16]; 16] };
        let filename: String = format!("assets/levels/{}.tilemap", id);
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
                println!("ERROR! {}", e);
            }
        }
        room
    }
}

fn randomize_rooms<const SIZE: usize>() -> [[Room; SIZE]; SIZE] {
    let mut rng = rand::thread_rng();
    let mut array: [[Room; SIZE]; SIZE] = [[Room::new(); SIZE]; SIZE];
    for r in 0..SIZE {
        for t in 0..SIZE {
            array[r][t] = rng.gen_range(1..5).into();
        }
    }
    array
}
