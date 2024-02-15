use super::*;

#[derive(Debug)]
pub struct StatusRequestPacket {
}

impl ServerPacket for StatusRequestPacket {
    fn handle(&self, client_idx: usize, server: &mut super::super::Server) {
        let responce = format!(r#"{{"version":{{"name":"1.18.2","protocol":758}},"players":{{"max":1,"online":0}},"description":{{"text":{:?}}}}}"#, server.config.motd);
        server.old_clients[client_idx].send_packet(&StatusRespondPacket { responce }).unwrap();
    }
}

impl StatusRequestPacket {
    pub fn new() -> Self {
        Self { }
    }
}

#[derive(Debug)]
pub struct PingPacket {
    pub payload: i64
}

impl ServerPacket for PingPacket {
    fn handle(&self, client_idx: usize, server: &mut super::super::Server) {
        server.old_clients[client_idx].send_packet(&PongPacket { payload: self.payload }).unwrap();
    }
}

impl PingPacket {
    pub fn new(packet: GenericPacket) -> io::Result<Self> {
        let mut data = &packet.data[..];
        let payload = data.read_i64()?;

        Ok(Self {
            payload
        })
    }
}

#[derive(Debug)]
pub struct StatusRespondPacket {
    pub responce: String
}

impl ClientPacket for StatusRespondPacket {
    fn write<W: PacketWriter>(&self, w: &mut W) -> io::Result<()> {
        w.write_varint(0x00)?;
        w.write_string(&self.responce)
    }
}

#[derive(Debug)]
pub struct PongPacket {
    pub payload: i64
}

impl ClientPacket for PongPacket {
    fn write<W: PacketWriter>(&self, w: &mut W) -> io::Result<()> {
        w.write_varint(0x01)?;
        w.write_u64(self.payload as u64)
    }
}
