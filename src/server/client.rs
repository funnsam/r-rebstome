use std::net::TcpStream;
use std::sync::mpsc;
use super::packet::{self, PacketReader};

pub struct Client {
    pub tcp: TcpStream,
    pub state: State,
    pub packets: mpsc::Sender<Box<dyn packet::ServerPacket>>
}

impl Client {
    pub fn listen(&mut self) {
        loop {
            let packet = self.tcp.read_packet(&mut self.state);

            if let Ok(packet) = packet {
                info!("client sent packet {:?} with state {:?}", packet, self.state);
                self.packets.send(packet).unwrap();
            } else {
                info!("client is dead");
                return;
            }
        }
    }

    /* pub fn handle_packet(&mut self, packet: packet::GenericPacket) {
        match (self.state, packet.typ) {
            _ => todo!("{:?} {packet:?}", self.state),
        }
    } */
}

#[derive(Debug)]
pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}
