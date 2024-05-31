use std::mem::replace;
use std::io::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, sleep, JoinHandle};
use std::time::Duration;
use get_if_addrs::{get_if_addrs, Interface};
use crate::packet::{Packet, PlayerData};

#[derive(PartialEq)]
pub enum Status {
    Running,
    Disconnected,
    Error
}

struct Listener {
    client: Option<(TcpStream, SocketAddr)>
}

struct Receiver {
    incoming: Vec<String>,
    outgoing: Option<String>,
    status: Status
}

fn listen(listener: Arc<Mutex<Listener>>, tcp_listener: TcpListener) {
    loop {
        match tcp_listener.accept() {
            Ok((stream, addr)) => {
                println!("Listener accepted client: {addr}");
                let mut listener = listener.lock().unwrap();
                listener.client = Some((stream, addr));
            }
            Err(e) => {
                let mut listener = listener.lock().unwrap();
                listener.client = None;
            }
        } 
        sleep(Duration::from_millis(1));    
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
                    receiver.incoming.push(data.to_string());
                    if let Some(message) = replace(&mut receiver.outgoing, None) {
                        let bytes_written: &[u8] = message.as_bytes();
                        let _ = stream.write_all(bytes_written);
                    }
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
        sleep(Duration::from_millis(1));
    }
}

fn num_to_letter(num: usize) -> char {
    assert!(num <= 26, "CHAR INDEX OUT OF BOUNDS");
    (b'A' + num as u8) as char
}

fn letter_to_num(char: char) -> usize {
    char as usize
}

#[derive(Debug)]
pub enum Error {
    BindError,
}

pub struct JoinCode {
    _addr: String,
    code: String,
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

pub struct Client {
    player_data: Option<PlayerData>,
    socket_thread: Option<JoinHandle<()>>,
    addr: String,
    running: bool,
    outgoing: Option<String>,
    status: Status
}

pub struct Server {
    clients: Vec<Arc<Mutex<Client>>>,
    connection_thread: Option<JoinHandle<()>>,
    distribute_thread: Option<JoinHandle<()>>,
    listen_thread: Option<JoinHandle<()>>,
    running: bool,
    join_code: Option<JoinCode>,
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Arc<Mutex<Client>> {
        let receiver: Arc<Mutex<Receiver>> = Arc::new(Mutex::new(Receiver { incoming: vec![], outgoing: None, status: Status::Running }));
        let recv_receiver = Arc::clone(&receiver);
        let recv_thread = spawn(move || {
            recv(receiver, stream);
        });
        let client = Client {
            player_data: None,
            socket_thread: None,
            addr: addr.to_string(),
            running: true,
            outgoing: None,
            status: Status::Running
        };
        let client = Arc::new(Mutex::new(client));
        let client_clone = Arc::clone(&client);
        let socket_thread = spawn(move || {
            receive(client_clone, recv_receiver)
        });
        {
            let mut client = client.lock().unwrap();
            client.socket_thread = Some(socket_thread);
        }
        return client;
    }

    pub fn send(&mut self, message: String) {
        self.outgoing = Some(message);
    }
}

fn receive(client: Arc<Mutex<Client>>, receiver: Arc<Mutex<Receiver>>) {
    loop {
        {
            let mut client = client.lock().unwrap();
            let incoming = {
                let mut receiver = receiver.lock().unwrap();
                if receiver.status != Status::Running {
                    client.status = Status::Disconnected;
                }
                if let Some(_) = &client.outgoing {
                    receiver.outgoing = replace(&mut client.outgoing, None);
                }
                replace(&mut receiver.incoming, vec![])
            };
            // Only look at most recent message.
            if let Some(data) = incoming.last() {
                client.player_data = Some(data.as_str().into());
            }
        }
        sleep(Duration::from_millis(1));
    }
}

impl Server {
    pub fn new() -> Result<Arc<Mutex<Server>>, Error> {
        if let Ok(tcp_listener) = TcpListener::bind("0.0.0.0:50000") {
            let listener = Arc::new(Mutex::new(Listener { client: None }));
            let accept_listener = Arc::clone(&listener);
            let listen_thread = spawn(move || {
                listen(accept_listener, tcp_listener);
            });
            let interfaces = get_if_addrs().unwrap();
            let server = Server {
                clients: vec![],
                connection_thread: None,
                distribute_thread: None,
                listen_thread: Some(listen_thread),
                running: true,
                join_code: Some((&interfaces).into()),
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

            {
                let mut server = server.lock().unwrap();
                server.connection_thread = Some(connection_thread);
                server.distribute_thread = Some(distribute_thread);
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
    loop {
        {
            let mut server = server.lock().unwrap();
            let client_option = {
                let mut listener = listener.lock().unwrap();
                replace(&mut listener.client, None)
            };
        
            if let Some((client, addr)) = client_option {
                let c: Arc<Mutex<Client>> = Client::new(client, addr);
                println!("Server polled client: {addr}");
                server.clients.push(c);
            }
        
            if !server.running {
                break;
            }
        }        
        sleep(Duration::from_millis(1));
    }
}

fn send(server: Arc<Mutex<Server>>) {
    loop {
        let mut active_player_data: Vec<PlayerData> = vec![];
        {
            let mut server = server.lock().unwrap();
            if !server.running { break; }
            for client in &server.clients {
                let client = client.lock().unwrap();
                if let Some(player_data) = &client.player_data {
                    active_player_data.push(player_data.clone());
                }
            }
            let packet: Packet = (&active_player_data).into();
            let mut to_remove: Vec<usize> = vec![];
            let mut count: usize = 0;
            for client in &server.clients {
                let mut c = client.lock().unwrap();
                c.send(packet.packet.clone());
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
        sleep(Duration::from_millis(1));
    }
}