#[derive(Clone)]
pub struct PlayerData {
    x_position: f64,
    y_position: f64,
    crouching: bool,
    username: String
}

impl From<&str> for PlayerData {
    fn from(data: &str) -> Self {
        let components: Vec<&str> = data.split('!').collect::<Vec<&str>>()[0].split(',').collect();
        if components.len() == 4 {
            if let (Ok(x), Ok(y), Ok(c), n) = (
                components[0].parse::<f64>(),
                components[1].parse::<f64>(),
                components[2].parse::<bool>(),
                components[3]
            ) 
            {
                return PlayerData { x_position: x, y_position: y, crouching: c, username: String::from(n) };
            }
        }
        PlayerData { x_position: 0.0, y_position: 0.0, crouching: false, username: String::new() }
    }
}

pub struct Packet {
    pub packet: String
}

impl From<&Vec<PlayerData>> for Packet {
    fn from(data: &Vec<PlayerData>) -> Packet{
        let mut packet: Packet = Packet { packet: String::from("_|") };
        for player_data in data {
            let string_data: String = format!("{},{},{},{}!", player_data.x_position, player_data.y_position, player_data.crouching, player_data.username);
            if packet.packet.len() > 2 { packet.packet.push('/'); }
            packet.packet.push_str(&string_data);
        }
        return packet;
    }
}