use crate::tilemap::Tilemap;

#[derive(Clone)]
pub struct PlayerData {
    x_position: f32,
    y_position: f32,
    crouching: bool,
    frame: i8,
    direction: i8,
    username: String
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

pub struct Packet {
    pub packet: String
}

impl From<&Vec<PlayerData>> for Packet {
    fn from(data: &Vec<PlayerData>) -> Packet{
        let mut packet: Packet = Packet { packet: String::from("_|") };
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

impl From<Tilemap> for Packet {
    fn from(tilemap: Tilemap) -> Self {
        let mut packet: Packet = Packet { packet: String::from("<initial>") };
        let string_data = format!("{},{}|{}!",
            tilemap.spawn_coordinates[0], 
            tilemap.spawn_coordinates[1],
            tilemap.get_as_string());   
        packet.packet += &string_data;
        packet 
    }
}