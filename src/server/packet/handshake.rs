use super::*;

#[derive(Debug)]
pub struct HandshakePacket {
    pub proto: i32,
    pub address: String,
    pub port: u16,
    pub next: i32
}

impl ServerPacket for HandshakePacket {
    fn handle(&self, _client_idx: usize, _server: &mut super::super::Server) { }
}

impl HandshakePacket {
    pub fn new(packet: GenericPacket) -> io::Result<Self> {
        let mut data = &packet.data[..];
        let proto = data.read_varint()?;
        let address = data.read_string()?;
        let port = data.read_u16()?;
        let next = data.read_varint()?;

        Ok(Self { proto, address, port, next })
    }
}
