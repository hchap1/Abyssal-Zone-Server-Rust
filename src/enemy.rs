use crate::astar::{astar, Ai, Position};
use crate::packet::PlayerData;
use std::sync::{Arc, Mutex};

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
    tilemap: Vec<Vec<usize>>
}

impl Controller {
    pub fn new(players: Vec<PlayerData>, tilemap: Vec<Vec<usize>>) -> Self {
        let test_spider: Enemy = Enemy { 
            name: String::from("Spider"), ai: Ai::Spider, speed: 2.0f32, x: 12.0f32, y: 5.0f32, path_index: 2, path: None 
        };
        let test_ground: Enemy = Enemy { 
            name: String::from("Ground"), ai: Ai::Ground, speed: 4.0f32, x: 12.0f32, y: 5.0f32, path_index: 2, path: None 
        };
        Controller { enemies: vec![test_spider, test_ground], players: players, tilemap: tilemap }
    }
    pub fn update_players(&mut self, players: Vec<PlayerData>) {
        self.players = players;
    }
    pub fn update_enemies(&mut self) {
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