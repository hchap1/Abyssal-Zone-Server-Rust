use crate::{tilemap::Tilemap, vector::Vector};

#[derive(Clone)]
pub struct PlayerData {
    pub position: Vector,
    crouching: bool,
    frame: i8,
    direction: i8,
    pub username: String
}

impl PlayerData {
    pub fn new() -> Self {
        PlayerData { position: Vector::component(0f32, 0f32), crouching: false, frame: 0, direction: 0, username: String::from("NONE") }
    }
    pub fn parse_updates(&mut self, packets: &Vec<String>) {
        for packet in packets {
            let components: Vec<String> = packet.split('!').map(|x| String::from(x)).collect::<Vec<String>>()[0].split('>').map(|x| String::from(x)).collect();
            let mut identifier: String = components[0].clone();
            identifier.remove(0);
            let data: Vec<String> = components[1].split(',').map(|x| String::from(x)).collect::<Vec<String>>();
            self.username = data[0].clone();
            if identifier == "pp" {
                if let (Ok(x), Ok(y)) = (data[1].parse::<f32>(), data[2].parse::<f32>()) {
                    self.position = Vector::component(x, y);
                }
            }
            if identifier == "pf" {
                if let Ok(frame) = data[1].parse::<i8>() {
                    self.frame = frame;
                }
            }
            if identifier == "pd" {
                if let Ok(direction) = data[1].parse::<i8>() {
                    self.direction = direction;
                }
            }
            if identifier == "pc" {
                self.crouching = data[1] == "1";
            }
        }
    }
}

pub fn tilemap_packet(tilemap: Tilemap) -> Vec<String> {
    let mut transmissions: Vec<String> = vec![
            String::from("<tilemap_info>1!"),
            format!("<sp>{},{}!", tilemap.spawn_coordinates[0], tilemap.spawn_coordinates[1])
        ];
    for row in &tilemap.tilemap {
        let mut delim: String = String::new();
        for item in row {
            delim += &item.to_string();
            delim.push(',');
        }
        let mut temp: Vec<char> = delim.chars().collect();
        if let Some(last) = temp.last_mut() {
            *last = '!';
        }
        delim = temp.into_iter().collect();
        transmissions.push(format!("<tmr>{delim}"));
    }
    transmissions.push(String::from("<tilemap_info>0!"));
    transmissions
}