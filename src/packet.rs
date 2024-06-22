use crate::tilemap::Tilemap;

#[derive(Clone)]
pub struct PlayerData {
    pub x_position: f32,
    pub y_position: f32,
    crouching: bool,
    frame: i8,
    direction: i8,
    pub username: String
}

impl From<&str> for PlayerData {
    fn from(data: &str) -> Self {
        let components: Vec<&str> = data.split('!').collect::<Vec<&str>>()[0].split(',').collect();
        if components.len() == 6 {
            if let (Ok(x), Ok(y), c, Ok(f), Ok(d), n) = (
                components[0].parse::<f32>(), // x
                components[1].parse::<f32>(), // y
                components[2],                // Crouching (1: true, 0: false)
                components[3].parse::<i8>(),  // Frame
                components[4].parse::<i8>(),  // Direction (1: RIGHT, -1: LEFT)
                components[5]                 // Name
            ) 
            {
                return PlayerData { x_position: x, y_position: y, crouching: c == "1", frame: f, direction: d, username: String::from(n) };
            }
        }
        PlayerData { x_position: 0.0, y_position: 0.0, crouching: false, frame: 0, direction: 1, username: String::from("NONE") }
    }
}

impl PlayerData {
    pub fn parse_updates(&mut self, packets: &Vec<String>) {
        for packet in packets {
            let components: Vec<String> = packet.split('!').map(|x| String::from(x)).collect::<Vec<String>>()[0].split('>').map(|x| String::from(x)).collect();
            let mut identifier: String = components[0].clone();
            identifier.remove(0);
            let data: Vec<String> = components[1].split(',').map(|x| String::from(x)).collect::<Vec<String>>();
            if identifier == "pp" {
                if let (Ok(x), Ok(y)) = (data[1].parse::<f32>(), data[2].parse::<f32>()) {
                    self.x_position = x;
                    self.y_position = y;
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

pub struct Packet {
    pub packet: String
}

impl Packet {
    pub fn from_enemy_and_player_data(enemy_packet: String, player_data: Vec<PlayerData>) -> Self {
        Packet { packet: enemy_packet + &Packet::from(&player_data).packet }
    }
}

impl From<&Vec<PlayerData>> for Packet {
    fn from(data: &Vec<PlayerData>) -> Packet{
        let mut packet: Packet = Packet { packet: String::from("|") };
        for player_data in data {
            let mut c: char = '0';
            if player_data.crouching {
                c = '1';
            }
            let string_data: String = format!("{},{},{},{},{},{}!", 
                player_data.x_position, 
                player_data.y_position, 
                c,
                player_data.frame, 
                player_data.direction, 
                player_data.username);
            if packet.packet.len() > 2 { packet.packet.push('/'); }
            packet.packet.push_str(&string_data);
        }
        return packet;
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