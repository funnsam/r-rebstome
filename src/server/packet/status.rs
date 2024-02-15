use super::*;

#[derive(Debug)]
pub struct StatusRequestPacket {
}

impl ServerPacket for StatusRequestPacket {
    fn handle(&self, client_idx: usize, server: &mut super::super::Server) {
        let resp = format!(
r#"{{
    "version": {{
        "name: "1.18.2",
        "protocol": 758
    }},
    "players": {{
        "max": 1,
        "online": 0,
        "sample": []
    }},
    "description": {{
        "text": {:?},
        "italic": true
    }}
}}"#, server.config.motd);
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
        // TODO: respond with pong packet
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
