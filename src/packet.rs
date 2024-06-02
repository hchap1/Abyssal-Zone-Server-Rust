#[derive(Clone)]
pub struct PlayerData {
    x_position: f64,
    y_position: f64,
    crouching: bool,
    frame: f64,
    direction: f64,
    username: String
}

impl From<&str> for PlayerData {
    fn from(data: &str) -> Self {
        println!("DATA: {data}");
        let components: Vec<&str> = data.split('!').collect::<Vec<&str>>()[0].split(',').collect();
        if components.len() == 6 {
            if let (Ok(x), Ok(y), Ok(c), Ok(f), Ok(d), n) = (
                components[0].parse::<f64>(),
                components[1].parse::<f64>(),
                components[2].parse::<bool>(),
                components[3].parse::<f64>(),
                components[4].parse::<f64>(),
                components[5]
            ) 
            {
                return PlayerData { x_position: x, y_position: y, crouching: c, frame: f, direction: d, username: String::from(n) };
            }
        }
        PlayerData { x_position: 0.0, y_position: 0.0, crouching: false, frame: 0.0, direction: 0.0, username: String::new() }
    }
}

pub struct Packet {
    pub packet: String
}

impl From<&Vec<PlayerData>> for Packet {
    fn from(data: &Vec<PlayerData>) -> Packet{
        let mut packet: Packet = Packet { packet: String::from("_|") };
        for player_data in data {
            let string_data: String = format!("{},{},{},{},{},{}!", player_data.x_position, 
            player_data.y_position, player_data.crouching, player_data.frame, 
            player_data.direction, player_data.username);
            if packet.packet.len() > 2 { packet.packet.push('/'); }
            packet.packet.push_str(&string_data);
        }
        return packet;
    }
}