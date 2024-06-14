use std::fs::read_to_string;
use rand::Rng;

pub struct Tilemap {
    pub tilemap: Vec<Vec<usize>>,
    pub spawn_coordinates: [usize; 2],
}

impl From<Vec<Vec<usize>>> for Tilemap {
    fn from(mut room_map: Vec<Vec<usize>>) -> Self {
        let size: usize = room_map.len();
        let mut rng = rand::thread_rng();
        let spawn_room: [usize; 2] = [rng.gen_range(0..size), rng.gen_range(0..size)];
        //let mut end_room: [usize; 2] = [rng.gen_range(0..size), rng.gen_range(0..size)];
        //while end_room == spawn_room { end_room = [rng.gen_range(0..size), rng.gen_range(0..size)]; }
        let mut tilemap = Tilemap { 
            tilemap: vec![vec![0; size * 16]; size * 16], 
            spawn_coordinates: [spawn_room[0] * 16 + 7, spawn_room[1] * 16 + 3] 
        };
        room_map[spawn_room[1]][spawn_room[0]] = 0;
        for room_row in 0..size {
            for room_column in 0..size {
                let room: Room = Room::from(room_map[room_row][room_column]);
                for tile_row in 0..16 {
                    for tile_column in 0..16 {
                        tilemap.tilemap[room_row * 16 + tile_row][room_column * 16 + tile_column] = room.tilemap[tile_row][tile_column];
                    }
                }
            }
        }
        tilemap
    }
}

impl Tilemap {
    pub fn get_as_string(&self) -> String {
        let size: usize = self.tilemap.len();
        let mut output: String = String::new();
        for row in 0..size {
            output += self.tilemap[size - row - 1].iter().map(|x| x.to_string() + ",").collect::<String>().as_str();
            if output.len() > 0 { output.remove(output.len() - 1); }
            if size - row > 1 { output.push('/'); }
        }
        output
    }
}

pub struct Room {
    tilemap: Vec<Vec<usize>>
}

impl Room {
    fn new() -> Self {
        Room { tilemap: vec![vec![0; 16]; 16] }
    }
}

impl From<usize> for Room {
    fn from(id: usize) -> Self {
        let mut room = Room::new();
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

pub fn randomize_rooms(size: usize, stop: usize) -> Vec<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let mut room_map: Vec<Vec<usize>> = vec![vec![0; size]; size];
    for row in 0..size {
        for column in 0..size {
            room_map[row][column] = rng.gen_range(1..stop+1);
        }
    }
    room_map
}