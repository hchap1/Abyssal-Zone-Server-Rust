use std::{fs::read_to_string, thread::spawn};
use rand::Rng;
use crate::astar::{is_solid, Position, astar, Ai};
use core::mem::replace;

#[derive(Clone)]
pub struct Tilemap {
    pub tilemap: Vec<Vec<usize>>,
    pub spawn_coordinates: [usize; 2],
    pub spawn_locations: Vec<[usize; 2]>
}

impl From<Vec<Vec<Room>>> for Tilemap {
    fn from(mut room_map: Vec<Vec<Room>>) -> Self {
        let size: usize = room_map.len();
        let mut rng = rand::thread_rng();
        let spawn_room: [usize; 2] = [rng.gen_range(0..size), rng.gen_range(0..size)];
        let mut tilemap = Tilemap { 
            tilemap: vec![], 
            spawn_coordinates: [spawn_room[0] * 16 + 9, (size - spawn_room[1] - 1) * 16 + 2],
            spawn_locations: vec![]
        };
        let mut raw_tilemap: Vec<Vec<usize>> = vec![];
        room_map[spawn_room[1]][spawn_room[0]] = 0.into();
        let mut count: usize = 0;
        for room_row in 0..size {
            for tile_row in 0..16 {
                let mut row: Vec<usize> = vec![];
                for room_column in 0..size {
                    row.append(&mut room_map[room_row][room_column].tilemap[tile_row]);
                }
                count += 1;
                raw_tilemap.push(row);
            }
        }
        for room_row in 0..size {
            for room_column in 0..size {
                tilemap.spawn_locations.push([room_map[room_row][room_column].spawn_x + room_column * 16, room_map[room_row][room_column].spawn_y + room_row * 16]);
            }
        }
        let final_length: usize = raw_tilemap.len();
        for row in 0..final_length {
            tilemap.tilemap.push(replace(&mut raw_tilemap[final_length - row - 1], vec![]));
        }
        let mut to_remove: Vec<usize> = vec![];
        for (index, spawn_location) in tilemap.spawn_locations.iter().enumerate() {
            println!("Checking potential: {:?} -> {:?}", spawn_location, tilemap.spawn_coordinates);
            let path: Option<Vec<Position>> = astar(&tilemap.tilemap, Position::from(*spawn_location), Position::from(tilemap.spawn_coordinates), &Ai::Spider);
            if path == None {
                println!("...invalid");
                //to_remove.push(index);
            }
            else {
                println!("...valid");
            }
        }
        to_remove.sort();
        let mut count: usize = 0;
        for index in to_remove {
            tilemap.spawn_locations.remove(index - count);
            count += 1;
        }
        tilemap
    }
}
pub struct Room {
    tilemap: Vec<Vec<usize>>,
    spawn_x: usize,
    spawn_y: usize,
    entrance_right:  Option<usize>,
    entrance_left:   Option<usize>,
    entrance_top:    Option<usize>,
    entrance_bottom: Option<usize>
}

impl Room {
    fn new() -> Self {
        Room { 
            tilemap: vec![],
            spawn_x: 0,
            spawn_y: 0,
            entrance_right:  None, 
            entrance_left:   None, 
            entrance_top:    None, 
            entrance_bottom: None 
        }
    }
}

impl From<usize> for Room {
    fn from(id: usize) -> Self {
        let mut room = Room::new();
        let filename = format!("assets/levels/{}.tilemap", id);
        let mut width: usize = 0;
        match read_to_string(&filename) {
            Ok(file_string) => {
                let mut lines: Vec<Vec<usize>> = file_string
                    .lines()
                    .map(|line| line.split(' ').map(|x| x.parse::<usize>().unwrap_or(0)).collect())
                    .collect();
                if let Some(first_line) = lines.first() {
                    width = first_line.len();
                }
                for y in 0..width {
                    room.tilemap.push(replace(&mut lines[y], vec![]));
                }
                for index in 0..lines.len() - width {
                    match index {
                        0 => {
                            room.spawn_x = lines[width + index][0];
                            room.spawn_y = lines[width + index][1];
                        }
                        1 => {
                            let offset: usize = lines[width + index][0];
                            if offset != 0 { room.entrance_right = Some(offset); }
                        }
                        2 => {
                            let offset: usize = lines[width + index][0];
                            if offset != 0 { room.entrance_left = Some(offset); }
                        }
                        3 => {
                            let offset: usize = lines[width + index][0];
                            if offset != 0 { room.entrance_top = Some(offset); }
                        }
                        4 => {
                            let offset: usize = lines[width + index][0];
                            if offset != 0 { room.entrance_bottom = Some(offset); }
                        }
                        _ => {}
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

pub fn randomize_rooms(size: usize, stop: usize) -> Vec<Vec<Room>> {
    let mut rng = rand::thread_rng();
    let mut room_ids: Vec<Vec<Room>> = vec![];
    for _ in 0..size {
        let mut row: Vec<Room> = vec![];
        for _ in 0..size {
            row.push(Room::from(rng.gen_range(1..stop+1)));
        }
        room_ids.push(row);
    }
    room_ids
}