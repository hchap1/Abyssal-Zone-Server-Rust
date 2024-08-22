use std::{fs::read_to_string};
use rand::Rng;
use crate::astar::{Position, astar, Ai};
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
            spawn_coordinates: [spawn_room[0] * 32 + 9, (size - spawn_room[1] - 1) * 32 + 2 + (32 - Room::from(0).tilemap.len())],
            spawn_locations: vec![]
        };
        println!("Chose spawn location: [{},{}]!", tilemap.spawn_coordinates[0], tilemap.spawn_coordinates[1]);
        let mut raw_tilemap: Vec<Vec<usize>> = vec![];
        room_map[spawn_room[1]][spawn_room[0]] = 0.into();
        let mut count: usize = 0;
        for room_row in 0..size {
            for tile_row in 0..32 {
                let mut row: Vec<usize> = vec![];
                for room_column in 0..size {
                    for tile_column in 0..32 {
                        if tile_row < room_map[room_row][room_column].tilemap.len() && tile_column < room_map[room_row][room_column].tilemap.len() {
                            row.push(room_map[room_row][room_column].tilemap[tile_row][tile_column]);
                        }
                        else {
                            row.push(1);
                        }
                    }
                }
                count += 1;
                raw_tilemap.push(row);
            }
        }
        let final_length: usize = raw_tilemap.len();
        for row in 0..final_length {
            tilemap.tilemap.push(replace(&mut raw_tilemap[final_length - row - 1], vec![]));
        }

        for room_row in 0..size {
            for room_column in 0..size {
                tilemap.spawn_locations.push([room_map[room_row][room_column].spawn_x + room_column * 32, room_map[room_row][room_column].spawn_y + room_row * 32 + (32 - room_map[room_row][room_column].tilemap.len())]);
                if let Some(spawn_loc) = tilemap.spawn_locations.last() {
                    println!("Spawn location @ {},{} : TILE_ID={}", spawn_loc[0], spawn_loc[1], tilemap.tilemap[spawn_loc[1]][spawn_loc[0]]);
                }
            }
        }
        for row in &room_map {
            for item in row {
                print!(" {:?}", item.id);
            }
            println!();
        }
        room_map.reverse();
        for room_row in 0..size {
            for room_column in 0..size {
                if room_column < size - 1 {
                    let this_room: &Room = &room_map[room_row][room_column];
                    let next_room: &Room = &room_map[room_row][room_column + 1];
                    if let (Some(this_e), Some(next_e)) = (this_room.entrance_right, next_room.entrance_left) {
                        let start: Position = Position::new(room_column * 32 + this_room.tilemap.len(), room_row * 32 + (32 - this_room.tilemap.len()) + this_e);
                        let end: Position = Position::new(room_column * 32 + 32, room_row * 32 + (32 - next_room.tilemap.len()) + next_e);
                        if let Some(path) = astar(&tilemap.tilemap, start, end, &Ai::Corridor) {
                            for point in &path {
                                let x: usize = point.x.round() as usize;
                                let y: usize = point.y.round() as usize;
                                tilemap.tilemap[y][x] = 2;
                                tilemap.tilemap[y+1][x] = 2;
                                if rng.gen_bool(0.04f64) {
                                    tilemap.tilemap[y+1][x] = 3;
                                }
                                if rng.gen_bool(0.04f64) {
                                    tilemap.tilemap[y+1][x] = 7;
                                }
                            }
                        }
                    }
                }
                if room_row < size - 1 {
                    let this_room: &Room = &room_map[room_row][room_column];
                    let next_room: &Room = &room_map[room_row + 1][room_column];
                    if let (Some(this_e), Some(next_e)) = (this_room.entrance_top, next_room.entrance_bottom) {
                        let start: Position = Position::new(room_column * 32 + this_e, room_row * 32 + 31);
                        let end: Position = Position::new(room_column * 32 + next_e, room_row * 32 + 32 + (32 - next_room.tilemap.len()));
                        if let Some(path) = astar(&tilemap.tilemap, start, end, &Ai::Corridor) {
                            for point in &path {
                                let x: usize = point.x.round() as usize;
                                let y: usize = point.y.round() as usize;
                                tilemap.tilemap[y][x] = 6;
                                tilemap.tilemap[y][x+1] = 6;
                                if rng.gen_bool(0.04f64) {
                                    tilemap.tilemap[y][x+2] = 3;
                                }
                                if rng.gen_bool(0.04f64) {
                                    tilemap.tilemap[y][x+2] = 7;
                                }
                            }
                        }
                    }
                }
            }
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
    entrance_bottom: Option<usize>,
    id: usize
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
            entrance_bottom: None,
            id: 0
        }
    }
}

impl From<usize> for Room {
    fn from(id: usize) -> Self {
        let mut room = Room::new();
        room.id = id;
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
