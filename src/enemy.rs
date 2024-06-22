use crate::astar::{astar, Ai, Position};
use crate::packet::PlayerData;
use std::sync::{Arc, Mutex};
use rand::Rng;
use rand::{thread_rng, rngs::ThreadRng};

pub struct Enemy {
    name: String,
    ai: Ai,
    speed: f32,
    x: f32,
    y: f32,
    path_index: usize,
    path: Option<Vec<Position>>
}

impl Enemy {
    fn movement(&mut self, deltatime: f32) {
        if let Some(path) = &self.path {
            if self.path_index < path.len() {
                let target_position: &Position = &path[self.path_index];
                let mut dx: f32 = target_position.x as f32 - self.x;
                let mut dy: f32 = target_position.y as f32 - self.y;
                let mag: f32 = (dx.powf(2.0f32) + dy.powf(2.0f32)).sqrt();
                if mag <= 0.08 { self.path_index += 1; }
                else {
                    dx = (dx / mag) * self.speed * deltatime;
                    dy = (dy / mag) * self.speed * deltatime;
                    self.x += dx;
                    self.y += dy;
                }
            }
        }
    }
    fn get_data_as_string(&self) -> String {
        format!("{},{},{}!", self.x, self.y, self.name)
    }
}

pub struct Controller {
    enemies: Vec<Enemy>,
    players: Vec<PlayerData>,
    tilemap: Vec<Vec<usize>>,
    spawn_locations: Vec<[usize; 2]>
}

impl Controller {
    pub fn new(players: Vec<PlayerData>, tilemap: Vec<Vec<usize>>, spawn_locations: Vec<[usize; 2]>) -> Self {
        Controller { enemies: vec![], players: players, tilemap: tilemap, spawn_locations: spawn_locations }
    }
    pub fn update_players(&mut self, players: Vec<PlayerData>) {
        self.players = players;
    }
    pub fn update_enemies(&mut self) {
        let mut rng: ThreadRng = thread_rng();
        if self.enemies.len() > 0 {
            for i in 0..self.enemies.len() {
                let start: Position = Position::new(self.enemies[i].x.round() as usize, (self.enemies[i].y + 0.5).round() as usize);
                let mut min_dist: f32 = -1.0f32;
                let mut closest_player_index: usize = 999;
                for (index, player) in self.players.iter().enumerate() {
                    let distance: f32 = ((player.x_position - self.enemies[i].x).powf(2.0f32) + (player.y_position - self.enemies[i].y).powf(2.0f32)).sqrt();
                    if distance < min_dist || min_dist == -1.0f32 {
                        min_dist = distance;
                        closest_player_index = index;
                    }
                }
                if closest_player_index != 999 {
                    let end: Position = Position::new(self.players[closest_player_index].x_position.round() as usize, self.players[closest_player_index].y_position.round() as usize);
                    if self.enemies[i].path == None {
                        self.enemies[i].path = astar(&self.tilemap, start, end, &self.enemies[i].ai);
                        self.enemies[i].path_index = 1;
                    }
                    else if self.enemies[i].path_index >= self.enemies[i].path.clone().unwrap().len() {
                        self.enemies[i].path = astar(&self.tilemap, start, end, &self.enemies[i].ai);
                        self.enemies[i].path_index = 1; 
                    }
                }
            }
        }
        if self.enemies.len() < 5 {
            let frame_probability: f64 = 5.0f64 * 0.016f64; // deltatime
            let random_value: f64 = rng.gen();
            let result: bool = random_value < frame_probability;
            if result && false {
                if self.spawn_locations.len() > 0 {
                    let mut location: Option<[usize; 2]> = None;
                    for _ in 0..10 {
                        let index: usize = rng.gen_range(0..self.spawn_locations.len());
                        if index < self.spawn_locations.len() {
                            let spawn_location: [usize; 2] = self.spawn_locations[index];
                            let mut valid: bool = true;
                            for player in &self.players {
                                let distance: f32 = ((player.x_position - spawn_location[0] as f32).powf(2.0f32) + (player.y_position - spawn_location[1] as f32).powf(2.0f32)).sqrt();
                                if distance <= 10.0f32 {
                                    valid = false;
                                }
                            }
                            if valid {
                                location = Some(spawn_location);
                                break;
                            }
                        }
                    }
                    if let Some(location) = location {
                        self.enemies.push(Enemy { 
                            name: String::from("Spider"), 
                            ai: Ai::Spider,
                            speed: 4.0f32,
                            x: location[0] as f32,
                            y: location[1] as f32,
                            path_index: 1,
                            path: None
                        })
                    }
                }
            }
        }
    }
    pub fn move_enemies(&mut self, deltatime: f32) {
        for enemy in self.enemies.iter_mut() {
            enemy.movement(deltatime);
        }
    }
}

pub fn get_enemy_packet(controller: &Arc<Mutex<Controller>>) -> String {
    let controller = controller.lock().unwrap();
    let mut packet: String = String::new();
    for (index, enemy) in controller.enemies.iter().enumerate() {
        packet += &enemy.get_data_as_string();
        if index < controller.enemies.len() - 1 { packet.push('/'); }
    }
    packet
}