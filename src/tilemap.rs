use std::env;
use std::fs::read_to_string;
pub struct Room {
    tilemap: [[usize; 16]; 16]
}

impl From<usize> for Room {
    fn from(id: usize) -> Self {
        let mut room: Room = Room { tilemap: [[0; 16]; 16] };
        let filename: String = id.to_string() + ".tilemap";
        match read_to_string(filename) {
            Ok(file_string) => {
                let lines: Vec<Vec<usize>> = file_string
                    .split('\n')
                    .map(|x| x.to_string().split(' ')
                    .map(|y| y.parse::<usize>()
                    .unwrap())
                    .collect())
                    .collect();
                for y in 0..16 {
                    for x in 0..16 {
                        room.tilemap[y][x] = lines[y][x];
                    }
                }
            }   
            Err(e) => {
                println!("ERROR! {e}");
            }
        }
        return room;
    }
}