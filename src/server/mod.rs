pub mod config;
pub mod client;
pub mod packet;

use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub struct ServerClient {
    pub tcp: TcpStream,
    pub packets: mpsc::Receiver<Box<dyn packet::ServerPacket>>,
}

pub struct Server {
    pub new_clients: mpsc::Receiver<ServerClient>,
    pub old_clients: Vec<ServerClient>,

    pub config: config::Config,
}

impl Server {
    fn listen(address: String, clients: mpsc::Sender<ServerClient>) {
        let listener = TcpListener::bind(address).unwrap();

        loop {
            let (socket, ip) = listener.accept().unwrap();
            info!("new client from {}", ip);

            let (packets_t, packets_r) = mpsc::channel();
            let socket_2 = socket.try_clone().unwrap();

            let mut client = client::Client {
                tcp: socket,
                state: client::State::Handshake,
                packets: packets_t
            };

            thread::spawn(move || client.listen());

            clients.send(ServerClient {
                tcp: socket_2,
                packets: packets_r
            }).unwrap();
        }
    }

    pub fn new(config: config::Config) -> Self {
        let (clients_t, clients_r) = mpsc::channel();

        let address = config.address.clone();
        thread::spawn(move || Self::listen(address, clients_t));

        Self {
            new_clients: clients_r,
            old_clients: Vec::new(),

            config
        }
    }

    pub fn update(&mut self) {
        loop {
            self.update_clients();

            for i in 0..self.old_clients.len() {
                if let Ok(p) = self.old_clients[i].packets.try_recv() {
                    p.handle(i, self);
                }
            }
        }
    }

    pub fn update_clients(&mut self) {
        let client = self.new_clients.recv().unwrap();

        self.old_clients.push(client);
    }
}
