use crate::astar::{astar, Ai, Behaviour, Position};
use crate::packet::PlayerData;
use rand::Rng;
use std::time::{Instant, Duration};
use rand::{thread_rng, rngs::ThreadRng};
use crate::astar::is_solid;
use crate::vector::Vector;

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
    id: usize,
    behaviour: Behaviour,
    speed: f32,
    position: Vector,
    old_position: Vector,
    path_index: usize,
    path: Option<Vec<Vector>>,
    last_hit: Instant
}   

impl Enemy {
    fn movement(&mut self, deltatime: f32, players: &Vec<PlayerData>) -> Vec<String> {
        let mut packets: Vec<String> = vec![];
        if let Some(path) = &self.path {
            if self.path_index < path.len() {
                let target_position: &Vector = &path[self.path_index];
                let mut delta: Vector = &self.position - target_position;
                if delta.magnitude <= 0.08 { self.path_index += 1; }
                else {
                    if self.ai == Ai::Ground {
                        if delta.y > 0.0f32 {
                            delta.y *= 0.3f32;
                        }
                        else {
                            delta.y *= 1.3f32;
                        }
                        delta.update_polar();
                    }
                    delta.normalize();
                    self.position += delta;
                }
                if self.position != self.old_position {
                    packets.push(format!("<ep>{},{},{},{}!", self.name, self.position.x, self.position.y, self.position.direction));
                }
                self.old_position = self.position.clone();
            }
        }
        if self.old_position.magnitude == 0f32 {
            self.old_position = self.position.clone();
            packets.push(format!("<ep>{},{},{},{}!", self.name, self.position.x, self.position.y, 0f32));
        }
        for player in players {
            let distance: f32 = (&self.position - &player.position).magnitude;
            if distance < 3f32 {
                if let Some(path) = &self.path {
                    if let Some(last_node) = path.last() {
                        let distance: f32 = (&self.position - last_node).magnitude;
                        if distance > 5f32 {
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

    fn spider(location: Vector) -> Self {
        Enemy {
            name: String::from("spider"),
            ai: Ai::Spider,
            id: 0,
            behaviour: Behaviour::AttackSingle,
            speed: 2f32,
            position: location.clone(),
            old_position: location,
            path_index: 1,
            path: None,
            last_hit: Instant::now()
        }
    }

    fn goblin(location: Vector) -> Self {
        Enemy {
            name: String::from("goblin"),
            ai: Ai::Ground,
            id: 1,
            behaviour: Behaviour::AttackGroupFromClose,
            speed: 4f32,
            position: location.clone(),
            old_position: location,
            path_index: 1,
            path: None,
            last_hit: Instant::now()
        }
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
                let start: Position = Position::new(self.enemies[i].position.x.round() as usize, (self.enemies[i].position.y).round() as usize);
                let mut min_dist: f32 = -1.0f32;
                let mut closest_player_index: usize = 999;
                for (index, player) in self.players.iter().enumerate() {
                    let distance: f32 = (&player.position - &self.enemies[i].position).magnitude;
                    if distance < min_dist || min_dist == -1.0f32 {
                        min_dist = distance;
                        closest_player_index = index;
                    }
                }
                if closest_player_index != 999 && self.enemies[i].behaviour == Behaviour::AttackSingle {
                    let name: String = self.players[closest_player_index].username.clone();
                    for player in &self.players {
                        if player.username != name {
                            let dist: f32 = (&self.players[closest_player_index].position - &player.position).magnitude;
                            if dist <= 5.0f32 {
                                closest_player_index = 999;
                                break;
                            }
                        }
                    }

                }
                if closest_player_index != 999 && self.enemies[i].behaviour == Behaviour::AttackGroupFromClose {
                    let name: String = self.players[closest_player_index].username.clone();
                    let mut close_players: usize = 0;
                    for player in &self.players {
                        if player.username != name {
                            let dist: f32 = (&self.players[closest_player_index].position - &player.position).magnitude;
                            if dist <= 5.0f32 {
                                close_players += 1;
                            }
                        }
                    }
                    if close_players <= 3 {
                        closest_player_index = 999;
                    }
                }
                if closest_player_index != 999 {
                    let end: Position = Position::new(self.players[closest_player_index].position.x.round() as usize, self.players[closest_player_index].position.y.round() as usize);
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
            let frame_probability: f64 = 5f64 * 0.02f64;
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
                                let distance: f32 = (Vector::from(spawn_location) * -1f32 + &player.position).magnitude;
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
                            if rng.gen_bool(0.5f64) {
                                self.enemies.push(Enemy::goblin(location.into()));
                            }
                            else {
                                self.enemies.push(Enemy::spider(location.into()));
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
