use std::io::{self, Read};
use std::net::TcpStream;
use super::client::State;

#[derive(Debug, Clone)]
pub struct GenericPacket {
    pub typ: i32,
    pub data: Vec<u8>
}

impl PacketReader for TcpStream {}
impl PacketReader for &[u8] {}
pub trait PacketReader where Self: Read + Sized {
    fn read_bytes(&mut self, size: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_i8(&mut self) -> io::Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        Ok(u16::from_be_bytes(self.read_bytes(2)?.try_into().unwrap()))
    }

    fn read_i16(&mut self) -> io::Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        Ok(u32::from_be_bytes(self.read_bytes(4)?.try_into().unwrap()))
    }

    fn read_i32(&mut self) -> io::Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        Ok(u64::from_be_bytes(self.read_bytes(8)?.try_into().unwrap()))
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    #[inline(never)]
    fn read_varint(&mut self) -> io::Result<i32> {
        Ok(self._read_varint_with_size()?.0)
    }

    #[inline(never)]
    fn read_varint_with_size(&mut self) -> io::Result<(i32, usize)> {
        self._read_varint_with_size()
    }

    #[inline(always)]
    fn _read_varint_with_size(&mut self) -> io::Result<(i32, usize)> {
        let mut result = 0i32;
        let mut size = 0;
        for i in 0.. {
            let read = self.read_u8()?;
            result |= (read as i32 & 0x7f) << (i * 7);
            if i > 5 {
                panic!("too big");
            }

            size = i + 1;

            if read & 0x80 == 0 {
                break;
            }
        }

        Ok((result, size))
    }

    fn read_string(&mut self) -> io::Result<String> {
        let size = self.read_varint()?;
        Ok(String::from_utf8(self.read_bytes(size as usize)?).unwrap())
    }

    fn read_generic_packet(&mut self) -> io::Result<GenericPacket> {
        let size = self.read_varint()?;
        let (typ, ts) = self.read_varint_with_size()?;

        Ok(GenericPacket {
            typ,
            data: self.read_bytes((size - ts as i32) as usize)?,
        })
    }

    fn read_packet(&mut self, state: &mut State) -> io::Result<Box<dyn ServerPacket>> {
        let packet = self.read_generic_packet()?;

        match state {
            State::Handshake if packet.typ == 0 => {
                let packet = HandshakePacket::new(packet)?;
                *state = match packet.next {
                    1 => State::Status,
                    2 => State::Login,
                    _ => State::Handshake
                };

                Ok(Box::new(packet))
            },
            State::Status if packet.typ == 0 => Ok(Box::new(StatusRequestPacket::new())),
            State::Status if packet.typ == 1 => Ok(Box::new(PingPacket::new(packet)?)),
            _ => todo!("{state:?} {packet:?}"),
        }
    }
}

pub trait ServerPacket: std::fmt::Debug + Send {
    fn handle(&self, client_idx: usize, server: &mut super::Server);
}

pub mod handshake;
use handshake::*;
pub mod status;
use status::*;