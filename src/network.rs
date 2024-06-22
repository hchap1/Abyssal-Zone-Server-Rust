use std::mem::replace;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, sleep, JoinHandle};
use std::time::{Instant, Duration};
use get_if_addrs::{get_if_addrs, Interface};
use crate::packet::{Packet, PlayerData, tilemap_packet};
use crate::tilemap::Tilemap;
use crate::enemy::{Controller, get_enemy_packet};
use crate::util::split_with_delimiter;

#[derive(PartialEq)]
pub enum Status {
    Running,
    Disconnected,
    Error
}

#[derive(Debug)]
pub enum Error {
    BindError,
}

struct Listener {
    client: Option<(TcpStream, SocketAddr)>,
    initial_packet: Vec<String>
}

struct Receiver {
    username: String,
    incoming: Vec<String>,
    outgoing: Vec<String>,
    status: Status
}

pub struct JoinCode {
    _addr: String,
    code: String,
}

pub struct Client {
    player_data: Option<PlayerData>,
    socket_thread: Option<JoinHandle<()>>,
    send_thread: Option<JoinHandle<()>>,
    _addr: String,
    _running: bool,
    incoming: Vec<String>,
    outgoing: Vec<String>,
    status: Status,
    num: isize
}

pub struct Server {
    clients: Vec<Arc<Mutex<Client>>>,
    connection_thread: Option<JoinHandle<()>>,
    distribute_thread: Option<JoinHandle<()>>,
    _listen_thread: Option<JoinHandle<()>>,
    running: bool,
    join_code: Option<JoinCode>,
    enemy_controller: Arc<Mutex<Controller>>,
    controller_thread: Option<JoinHandle<()>>,
    enemy_thread: Option<JoinHandle<()>>
}


fn listen(listener: Arc<Mutex<Listener>>, tcp_listener: TcpListener) {
    loop {
        match tcp_listener.accept() {
            Ok((mut stream, addr)) => {
                println!("Listener accepted client: {addr}");
                let mut listener = listener.lock().unwrap();
                for packet in &listener.initial_packet {
                    let _ = stream.write_all(packet.as_bytes());
                    sleep(Duration::from_millis(2));
                }
                listener.client = Some((stream, addr));
            }
            Err(_) => {
                let mut listener = listener.lock().unwrap();
                listener.client = None;
            }
        } 
        sleep(Duration::from_millis(10));    
    }
}

fn recv(receiver: Arc<Mutex<Receiver>>, mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                let mut receiver = receiver.lock().unwrap();
                receiver.status = Status::Disconnected;
            }
            Ok(_) => match std::str::from_utf8(&buffer) {
                Ok(data) => {
                    let mut receiver = receiver.lock().unwrap();
                    let message: String = data.to_string();
                    let mut packets: Vec<String> = split_with_delimiter(message, "!");
                    println!("Received packets: {:?}", packets);
                    receiver.incoming.append(&mut packets);
                }
                Err(e) => {
                    println!("Error: {e}");
                }
            },
            Err(_) => {
                let mut receiver = receiver.lock().unwrap();
                receiver.status = Status::Error;
            }
        }
        buffer = [0; 512];
        sleep(Duration::from_millis(10));
    }
}

fn distribute_receiver_outgoing(receiver: Arc<Mutex<Receiver>>, mut stream: TcpStream) {
    loop {
        {
            let mut receiver = receiver.lock().unwrap();
            for message in &receiver.outgoing {
                let bytes_written: &[u8] = message.as_bytes();
                let _ = stream.write_all(bytes_written);
            }
            receiver.outgoing = vec![];
        }
    }
}

fn num_to_letter(num: usize) -> char {
    assert!(num <= 26, "CHAR INDEX OUT OF BOUNDS");
    (b'A' + num as u8) as char
}

fn letter_to_num(char: char) -> usize {
    char as usize
}

impl From<&Vec<Interface>> for JoinCode {
    fn from(if_addrs: &Vec<Interface>) -> Self {
        let mut ip: String = String::from("0.0.0.0");
        for iface in if_addrs {
            if let get_if_addrs::IfAddr::V4(iface_v4) = &iface.addr {
                if !iface.is_loopback() {
                    let addr_string: String = format!("{}", iface_v4.ip);
                    if addr_string.starts_with("192.168.") {
                        ip = addr_string;
                    }
                }
            }
        }
        let port: u16 = 50000;
        let sections: Vec<String> = ip.split('.').map(|x| String::from(x)).collect();
        let important_digits: String = sections[2].clone() + &sections[3];
        let dot_index: usize = sections[2].len();
        let port_index: usize = dot_index + sections[3].len();
        let mut code: String = String::from(num_to_letter(dot_index));
        code.push(num_to_letter(port_index));
        code += &important_digits;
        code += &port.to_string();
        JoinCode {
            _addr: ip + ":" + &port.to_string(),
            code: code,
        }
    }
}

impl From<&String> for JoinCode {
    fn from(code: &String) -> Self {
        if let (Some(dot_char), Some(port_char)) = (code.chars().nth(0), code.chars().nth(1)) {
            let dot_index: usize = letter_to_num(dot_char);
            let port_index: usize = letter_to_num(port_char);
            let port: String = code[dot_index + 2..code.len() - 1].to_string();
            let important_digits: String = code[2..port_index].to_string();
            let a: String = important_digits[0..dot_index].to_string();
            let b: String = important_digits[dot_index..port_index].to_string();
            let mut addr: String = String::from("192.168.") + &a + &b;
            addr.push(':');
            addr += &port;
            JoinCode {
                _addr: addr,
                code: code.clone(),
            }
        } else {
            JoinCode {
                _addr: String::from("NA"),
                code: code.clone(),
            }
        }
    }
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr, active_players: Vec<String>, active_enemies: Vec<String>, num: isize) -> Arc<Mutex<Client>> {
        let mut outgoing: Vec<String> = vec![];
        for player in &active_players {
            outgoing.push(format!("pcon>{player}"));
        } 
        let receiver: Arc<Mutex<Receiver>> = Arc::new(Mutex::new(Receiver { username: num.to_string(), incoming: vec![], outgoing: outgoing, status: Status::Running }));
        let recv_receiver = Arc::clone(&receiver);
        let recv_stream = stream.try_clone().unwrap();
        let send_receiver = Arc::clone(&receiver);
        let _recv_thread = spawn(move || {
            recv(receiver, stream);
        });
        let client = Client {
            player_data: None,
            socket_thread: None,
            send_thread: None,
            _addr: addr.to_string(),
            _running: true,
            incoming: vec![],
            outgoing: vec![],
            status: Status::Running,
            num: num
        };
        let client = Arc::new(Mutex::new(client));
        let client_clone = Arc::clone(&client);
        let socket_thread = spawn(move || {
            receive(client_clone, recv_receiver)
        });
        let send_thread = spawn(move || {
            distribute_receiver_outgoing(send_receiver, recv_stream)
        });
        {
            let mut client = client.lock().unwrap();
            client.socket_thread = Some(socket_thread);
            client.send_thread = Some(send_thread);
        }
        return client;
    }

    pub fn send(&mut self, message: String) {
        self.outgoing.push(message);
    }

    pub fn send_all(&mut self, messages: &Vec<String>) {
        for m in messages {
            self.outgoing.push(m.clone());
        }
    }
}

fn receive(client: Arc<Mutex<Client>>, receiver: Arc<Mutex<Receiver>>) {
    loop {
        {
            let mut client = client.lock().unwrap();
            let mut incoming = {
                let mut receiver = receiver.lock().unwrap();
                if receiver.status != Status::Running {
                    client.status = Status::Disconnected;
                }
                if client.outgoing.len() > 0 {
                    let mut messages: Vec<String> = replace(&mut client.outgoing, vec![]);
                    receiver.outgoing.append(&mut messages);
                }
                replace(&mut receiver.incoming, vec![])
            };
            if let Some(_) = client.player_data {
                client.player_data.as_mut().unwrap().parse_updates(&incoming);
            }
            client.incoming.append(&mut incoming);
        }
        sleep(Duration::from_millis(10));
    }
}

impl Server {
    pub fn new(tilemap: Tilemap) -> Result<Arc<Mutex<Server>>, Error> {
        if let Ok(tcp_listener) = TcpListener::bind("0.0.0.0:50000") {
            let tilemap_packets: Vec<String> = tilemap_packet(tilemap.clone());
            let controller = Arc::new(Mutex::new(Controller::new(vec![], tilemap.tilemap, tilemap.spawn_locations)));
            let player_data_ref = Arc::clone(&controller);
            let enemy_movement_ref = Arc::clone(&controller);
            let listener = Arc::new(Mutex::new(Listener { client: None, initial_packet: tilemap_packets }));
            let accept_listener = Arc::clone(&listener);
            let listen_thread = spawn(move || {
                listen(accept_listener, tcp_listener);
            });
            let interfaces = get_if_addrs().unwrap();
            let server = Server {
                clients: vec![],
                connection_thread: None,
                distribute_thread: None,
                _listen_thread: Some(listen_thread),
                running: true,
                join_code: Some((&interfaces).into()),
                enemy_controller: controller,
                controller_thread: None,
                enemy_thread: None
            };
            let server = Arc::new(Mutex::new(server));

            let accept_server = Arc::clone(&server);
            let connection_thread = spawn(move || {
                accept(accept_server, listener);
            });
            let send_server = Arc::clone(&server);
            let distribute_thread = spawn(move || {
                send(send_server);
            });
            let controller_server = Arc::clone(&server);
            let controller_thread = spawn(move || {
                push_updates_to_enemies(controller_server, player_data_ref);
            });
            let enemy_thread = spawn(move || {
                enemy_cycle(enemy_movement_ref);
            });
            {
                let mut server = server.lock().unwrap();
                server.connection_thread = Some(connection_thread);
                server.distribute_thread = Some(distribute_thread);
                server.controller_thread = Some(controller_thread);
                server.enemy_thread      = Some(enemy_thread);
            }
            Ok(server)
        } else {
            Err(Error::BindError)
        }
    }

    pub fn get_joincode(&self) -> String {
        match &self.join_code {
            Some(join_code) => join_code.code.clone(),
            None => String::from("<NONE>"),
        }
    }
}

fn accept(server: Arc<Mutex<Server>>, listener: Arc<Mutex<Listener>>) {
    let mut count: isize = 0;
    loop {
        {
            let mut server = server.lock().unwrap();
            let client_option = {
                let mut listener = listener.lock().unwrap();
                replace(&mut listener.client, None)
            };
            let mut active_players: Vec<String> = vec![];
            for client in server.clients.iter() {
                let c = client.lock().unwrap();
                if let Some(pd) = &c.player_data {
                    active_players.push(pd.username.clone());
                }
            }
            if let Some((client, addr)) = client_option {
                let c: Arc<Mutex<Client>> = Client::new(client, addr, active_players, vec![], count);
                println!("Server polled client: {addr}");
                server.clients.push(c);
                count += 1;
            }
        
            if !server.running {
                break;
            }
        }        
        sleep(Duration::from_millis(10));
    }
}

fn send(server: Arc<Mutex<Server>>) {
    loop {
        let mut packets: Vec<String> = vec![];
        {
            let mut server = server.lock().unwrap();
            if !server.running { break; }
            for client in &server.clients {
                let mut client = client.lock().unwrap();
                packets.append(&mut client.incoming);
                let _ = replace(&mut client.incoming, vec![]);
            }
            let mut to_remove: Vec<usize> = vec![];
            let mut count: usize = 0;

            for client in &server.clients {
                let mut c = client.lock().unwrap();
                c.send_all(&packets);
            }

            for client in &server.clients {
                let c = client.lock().unwrap();
                if c.status != Status::Running {
                    to_remove.push(count);
                }
                else {
                    count += 1;
                }
            }
            for index in to_remove {
                server.clients.remove(index);
            }
        }
        sleep(Duration::from_millis(10));
    }
}

fn push_updates_to_enemies(server: Arc<Mutex<Server>>, controller: Arc<Mutex<Controller>>) {
    loop {
        let mut active_player_data: Vec<PlayerData> = vec![];
        {
            let server = server.lock().unwrap();
            if !server.running { break; }
            for client in &server.clients {
                let client = client.lock().unwrap();
                if let Some(player_data) = &client.player_data {
                    active_player_data.push(player_data.clone());
                }
            }
        }
        {
            let mut controller = controller.lock().unwrap();
            controller.update_players(active_player_data);
            controller.update_enemies();
        }
        sleep(Duration::from_millis(500));
    }
}

fn enemy_cycle(controller: Arc<Mutex<Controller>>) {
    let mut previous_time = Instant::now();
    loop {
        let current_time = Instant::now();
        let deltatime: f32 = current_time.duration_since(previous_time).as_secs_f32();
        previous_time = current_time;
        {
            let mut controller = controller.lock().unwrap();
            controller.move_enemies(deltatime);
        }
        sleep(Duration::from_millis(16));
    }
}