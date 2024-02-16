use std::io::{self, Read, Write};
use super::client::State;
use quartz_nbt as nbt;

#[derive(Debug, Clone)]
pub struct GenericPacket {
    pub typ: i32,
    pub data: Vec<u8>
}

impl<T: Read + Sized> PacketReader for T {}
pub trait PacketReader where Self: Read + Sized {
    fn read_bytes(&mut self, size: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_be<T: BigEndianNumeric<S>, const S: usize>(&mut self) -> io::Result<T> {
        Ok(T::_from_be_bytes(self.read_bytes(S)?.try_into().unwrap()))
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
            let read = self.read_be::<u8, 1>()?;
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
            State::Login if packet.typ == 0 => {
                *state = State::Login;
                Ok(Box::new(LoginStartPacket::new(packet)?))
            },
            _ => {
                warn!("unknown packet at state {:?} and content {:?}", state, packet);
                Ok(Box::new(UnknownPacket))
            },
        }
    }
}

pub trait ServerPacket where Self: std::fmt::Debug + Send {
    fn handle(&self, client_idx: usize, server: &mut super::Server);
}

pub struct PacketWriter {
    pub typ: i32,
    pub buffer: Vec<u8>
}

impl PacketWriter {
    pub fn new(typ: i32) -> Self {
        Self {
            typ,
            buffer: Vec::new()
        }
    }

    pub fn write_be<T: BigEndianNumeric<S>, const S: usize>(&mut self, d: T) {
        self.buffer.extend(d._to_be_bytes())
    }

    pub fn write_varint<T: Into<i32>>(&mut self, d: T) {
        let mut d = d.into();
        for _ in 1.. {
            if d & 0x80 == 0 {
                self.write_be(d as u8);
                return;
            }

            self.write_be(d as u8 | 0x80);
            d >>= 7;
        }
    }

    pub fn write_nbt_compound(
        &mut self,
        nbt: &nbt::NbtCompound,
        name: Option<&str>,
        flavor: nbt::io::Flavor
    ) {
        nbt::io::write_nbt(&mut self.buffer, name, nbt, flavor).unwrap();
    }

    pub fn write_string(&mut self, d: &str) {
        self.write_varint(d.len() as i32);
        self.buffer.extend(d.as_bytes());
    }

    pub fn export<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        fn write_varint(mut d: i32) -> Vec<u8> {
            let mut w = Vec::new();
            loop {
                if d & 0x80 == 0 {
                    w.push(d as u8);
                    return w;
                }

                w.push(d as u8 | 0x80);
                d >>= 7;
            }
        }

        let typ = write_varint(self.typ);
        w.write_all(&write_varint((typ.len() + self.buffer.len()) as i32))?;
        w.write_all(&typ)?;
        w.write_all(&self.buffer)
    }
}

pub trait BigEndianNumeric<const S: usize> where Self: Copy + Sized {
    fn _to_be_bytes(self) -> [u8; S];
    fn _from_be_bytes(b: [u8; S]) -> Self;
}

impl BigEndianNumeric<1> for bool {
    fn _to_be_bytes(self) -> [u8; 1] { (self as u8).to_be_bytes() }
    fn _from_be_bytes(b: [u8; 1]) -> Self { u8::from_be_bytes(b) != 0 }
}

impl BigEndianNumeric<1> for u8 {
    fn _to_be_bytes(self) -> [u8; 1] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 1]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<2> for u16 {
    fn _to_be_bytes(self) -> [u8; 2] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 2]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<4> for u32 {
    fn _to_be_bytes(self) -> [u8; 4] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 4]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<8> for u64 {
    fn _to_be_bytes(self) -> [u8; 8] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 8]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<16> for u128 {
    fn _to_be_bytes(self) -> [u8; 16] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 16]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<1> for i8 {
    fn _to_be_bytes(self) -> [u8; 1] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 1]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<2> for i16 {
    fn _to_be_bytes(self) -> [u8; 2] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 2]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<4> for i32 {
    fn _to_be_bytes(self) -> [u8; 4] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 4]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<8> for i64 {
    fn _to_be_bytes(self) -> [u8; 8] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 8]) -> Self { Self::from_be_bytes(b) }
}

impl BigEndianNumeric<16> for i128 {
    fn _to_be_bytes(self) -> [u8; 16] { self.to_be_bytes() }
    fn _from_be_bytes(b: [u8; 16]) -> Self { Self::from_be_bytes(b) }
}

pub trait ClientPacket where Self: std::fmt::Debug + Send {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()>;
}

pub mod handshake;
use handshake::*;
pub mod status;
use status::*;
pub mod login;
use login::*;

#[derive(Debug)]
pub struct UnknownPacket;

impl ServerPacket for UnknownPacket {
    fn handle(&self, _client_idx: usize, _server: &mut super::Server) {
    }
}
