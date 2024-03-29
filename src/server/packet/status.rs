use super::*;

#[derive(Debug)]
pub struct StatusRequestPacket {
}

impl ServerPacket for StatusRequestPacket {
    fn handle(&self, client_idx: usize, server: &mut super::super::Server) {
        let responce = json::object! {
            version: {
                name: "1.18.2",
                protocol: 758,
            },
            players: {
                max: 1,
                online: 0,
                sample: [],
            },
            description: {
                text: server.config.motd.clone(),
                color: "white",
                bold: true,
            },
        }.dump();

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
        let payload = data.read_be::<i64, 8>()?;

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
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut p = PacketWriter::new(0x00);
        p.write_string(&self.responce);

        p.export(w)
    }
}

#[derive(Debug)]
pub struct PongPacket {
    pub payload: i64
}

impl ClientPacket for PongPacket {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut p = PacketWriter::new(0x01);
        p.write_be(self.payload as u64);

        p.export(w)
    }
}
