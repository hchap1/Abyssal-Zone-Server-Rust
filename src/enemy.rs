use crate::astar::{astar, Ai, Behaviour, Position};
use crate::packet::PlayerData;
use rand::Rng;
use std::time::{Instant, Duration};
use rand::{thread_rng, rngs::ThreadRng};
use crate::astar::is_solid;

fn convert_angle(angle_ccw_from_x: f32) -> f32 {
    const PI: f32 = std::f32::consts::PI;
    let angle_cw_from_y = PI / 2.0 - angle_ccw_from_x;
    if angle_cw_from_y < 0.0 {
        angle_cw_from_y + 2.0 * PI
    } else {
        angle_cw_from_y
    }
}

fn compass_atan(x: f32, y: f32) -> f32 {
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    let angle = convert_angle(y.atan2(x));
    if angle < 0.0 {
        return angle + 2.0 * std::f32::consts::PI;
    }
    angle
}

pub struct Enemy {
    name: String,
    ai: Ai,
    behaviour: Behaviour,
    speed: f32,
    x: f32,
    y: f32,
    old_x: f32,
    old_y: f32,
    path_index: usize,
    path: Option<Vec<Position>>,
    last_hit: Instant
}

impl Enemy {
    fn movement(&mut self, deltatime: f32, players: &Vec<PlayerData>) -> Vec<String> {
        let mut packets: Vec<String> = vec![];
        let mut rotation: f32 = 0.0f32;
        if let Some(path) = &self.path {
            if self.path_index < path.len() {
                let target_position: &Position = &path[self.path_index];
                let mut dx: f32 = target_position.x as f32 - self.x;
                let mut dy: f32 = target_position.y as f32 - self.y;
                rotation = compass_atan(dx, dy);
                let degrot = rotation * (180.0f32/3.1415f32);
                println!("Rotation: {degrot}");
                let mag: f32 = (dx.powf(2.0f32) + dy.powf(2.0f32)).sqrt();
                if mag <= 0.08 { self.path_index += 1; }
                else {
                    dx = (dx / mag) * self.speed * deltatime;
                    dy = (dy / mag) * self.speed * deltatime;
                    if self.ai == Ai::Ground {
                        if dy > 0.0f32 {
                            dy *= 0.3f32;
                        }
                        else {
                            dy *= 1.3f32;
                        }
                    }
                    self.x += dx;
                    self.y += dy;
                }
                if self.old_x != self.x || self.old_y != self.y {
                    packets.push(format!("<ep>{},{},{},{}!", self.name, self.x, self.y, rotation));
                }
                self.old_x = self.x;
                self.old_y = self.y;
            }
        }
        if self.old_x == 0.0f32 && self.old_y == 0.0f32 {
            self.old_x = self.x;
            self.old_y = self.y;
            packets.push(format!("<ep>{},{},{},{}!", self.name, self.x, self.y, rotation));
        }
        for player in players {
            let distance: f32 = ((self.x - player.x_position).powf(2.0f32) + (self.y - player.y_position).powf(2.0f32)).sqrt();
            if distance < 3.0f32 {
                if let Some(path) = &self.path {
                    if let Some(last_node) = path.last() {
                        let distance: f32 = ((last_node.x as f32 - player.x_position).powf(2.0f32) + (last_node.y as f32 - player.y_position).powf(2.0f32)).sqrt();
                        if distance > 5.0f32 {
                            self.path_index = 999;
                        }
                    }
                }
                if Instant::now().duration_since(self.last_hit) > Duration::from_millis(1000) && distance < 0.8f32 {
                    self.path_index = 999;
                    let packet: String = format!("<ph>{},{}!", player.username, -20);
                    packets.push(packet);
                    self.last_hit = Instant::now();
                    break;
                }
            }
        }
        return packets;
    }
}

pub struct Controller {
    id_count: usize,
    enemies: Vec<Enemy>,
    players: Vec<PlayerData>,
    tilemap: Vec<Vec<usize>>,
    spawn_locations: Vec<[usize; 2]>,
    pub packets: Vec<String>
}

impl Controller {
    pub fn new(players: Vec<PlayerData>, tilemap: Vec<Vec<usize>>, spawn_locations: Vec<[usize; 2]>) -> Self {
        Controller { id_count: 0, enemies: vec![], players: players, tilemap: tilemap, spawn_locations: spawn_locations, packets: vec![] }
    }
    pub fn update_players(&mut self, players: Vec<PlayerData>) {
        self.players = players;
    }
    pub fn update_enemies(&mut self) -> Option<String> {
        let mut rng: ThreadRng = thread_rng();
        if self.enemies.len() > 0 {
            for i in 0..self.enemies.len() {
                let start: Position = Position::new(self.enemies[i].x.round() as usize, (self.enemies[i].y).round() as usize);
                let mut min_dist: f32 = -1.0f32;
                let mut closest_player_index: usize = 999;
                for (index, player) in self.players.iter().enumerate() {
                    let distance: f32 = ((player.x_position - self.enemies[i].x).powf(2.0f32) + (player.y_position - self.enemies[i].y).powf(2.0f32)).sqrt();
                    if distance < min_dist || min_dist == -1.0f32 {
                        min_dist = distance;
                        closest_player_index = index;
                    }
                }
                if closest_player_index != 999 && self.enemies[i].behaviour == Behaviour::AttackSingle {
                    let px: f32 = self.players[closest_player_index].x_position;
                    let py: f32 = self.players[closest_player_index].y_position;
                    let name: String = self.players[closest_player_index].username.clone();
                    for player in &self.players {
                        if player.username != name {
                            let dist: f32 = ((px - player.x_position).powf(2.0f32) + (py - player.y_position).powf(2.0f32)).sqrt();
                            if dist <= 5.0f32 {
                                closest_player_index = 999;
                                break;
                            }
                        }
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
        if self.enemies.len() < 1 {
            let frame_probability: f64 = 5.0f64 * 0.02f64; // deltatime
            let random_value: f64 = rng.gen();
            let result: bool = random_value < frame_probability;
            if result || true {
                if self.spawn_locations.len() > 0 {
                    self.id_count += 1;
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
                            valid = true;
                            if valid {
                                location = Some(spawn_location);
                                break;
                            }
                        }
                    }
                    if let Some(location) = location {
                        if !is_solid(self.tilemap[location[1]][location[0]]) && location[0] != 0 && location[1] != 0 {
                            if rng.gen_bool(0.5f64) && false {
                                self.enemies.push(Enemy { 
                                    name: self.id_count.to_string(), 
                                    ai: Ai::Ground,
                                    behaviour: Behaviour::AttackMoving,
                                    speed: 2.0f32,
                                    x: location[0] as f32,
                                    y: location[1] as f32,
                                    old_x: location[0] as f32,
                                    old_y: location[1] as f32,
                                    path_index: 1,
                                    path: None,
                                    last_hit: Instant::now()
                                });
                            }
                            else {
                                self.enemies.push(Enemy { 
                                    name: self.id_count.to_string(), 
                                    ai: Ai::Spider,
                                    behaviour: Behaviour::AttackSingle,
                                    speed: 1.5f32,
                                    x: location[0] as f32,
                                    y: location[1] as f32,
                                    old_x: location[0] as f32,
                                    old_y: location[1] as f32,
                                    path_index: 1,
                                    path: None,
                                    last_hit: Instant::now()
                                });
                            }
                            return Some(self.id_count.to_string());
                        }
                    }
                }
            }
        }
        None
    }
    pub fn move_enemies(&mut self, deltatime: f32) {
        for enemy in self.enemies.iter_mut() {
            self.packets.append(&mut enemy.movement(deltatime, &self.players));
        }
    }
}
