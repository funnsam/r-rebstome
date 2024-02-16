use super::*;
use quartz_nbt::{NbtCompound, NbtList, compound};

#[derive(Debug)]
pub struct LoginStartPacket {
    pub player_name: String
}

impl ServerPacket for LoginStartPacket {
    fn handle(&self, client_idx: usize, server: &mut super::super::Server) {
        server.old_clients[client_idx].send_packet(&LoginSuccessPacket {
            uuid: 0,
            player_name: self.player_name.clone()
        }).unwrap();

        info!("{}",  compound! {
                "minecraft:dimension_type": compound! {
                    "type": "minecraft:dimension_type",
                    "value": NbtList::from(vec![
                        compound! {
                            "name": "minecraft:overworld",
                            "id": 0_i32,
                            "element": compound! {
                                "piglin_safe": 0_i8,
                                "natural": 1_i8,
                                "ambient_light": 1.0_f32,
                                "infiniburn": "#minecraft:infiniburn_overworld",
                                "respawn_anchor_works": 0_i8,
                                "has_skylight": 1_i8,
                                "bed_works": 0_i8,
                                "effects": "minecraft:overworld",
                                "has_raids": 0_i8,
                                "min_y": 0_i32,
                                "height": 1_i32,
                                "logical_height": 0_i32,
                                "coordinate_scale": 1.0_f64,
                                "ultrawarm": 0_i8,
                                "has_ceiling": 0_i8,
                            },
                        }
                    ]),
                },
                "minecraft:worldgen/biome": compound! {
                    "type": "minecraft:worldgen/biome",
                    "value": NbtList::from(vec![
                        compound! {
                            "name": "minecraft:ocean",
                            "id": 0_i32,
                            "element": compound! {
                                "precipitation": "none",
                                "depth": 0.0_f32,
                                "temperature": 0.0_f32,
                                "scale": 0.0_f32,
                                "downfall": 0.0_f32,
                                "category": "ocean",
                                "effects": compound! {
                                    "sky_color": 0x7fa1ff_i32,
                                    "water_fog_color": 0x7fa1ff_i32,
                                    "fog_color": 0x7fa1ff_i32,
                                    "water_color": 0x7fa1ff_i32,
                                }
                            },
                        },
                    ]),
                },
            }.to_pretty_snbt());

        server.old_clients[client_idx].send_packet(&JoinGamePacket {
            eid: 0,
            hardcore: false,
            cgm: 1,
            pgm: -1,
            dim_names: vec!["minecraft:ocean".to_string()],
            dim_codec: compound! {
                "minecraft:dimension_type": compound! {
                    "type": "minecraft:dimension_type",
                    "value": NbtList::from(vec![
                        compound! {
                            "name": "minecraft:overworld",
                            "id": 0_i32,
                            "element": compound! {
                                "piglin_safe": 0_i8,
                                "natural": 1_i8,
                                "ambient_light": 1.0_f32,
                                "infiniburn": "#minecraft:infiniburn_overworld",
                                "respawn_anchor_works": 0_i8,
                                "has_skylight": 1_i8,
                                "bed_works": 0_i8,
                                "effects": "minecraft:overworld",
                                "has_raids": 0_i8,
                                "min_y": 0_i32,
                                "height": 1_i32,
                                "logical_height": 0_i32,
                                "coordinate_scale": 1.0_f64,
                                "ultrawarm": 0_i8,
                                "has_ceiling": 0_i8,
                            },
                        }
                    ]),
                },
                "minecraft:worldgen/biome": compound! {
                    "type": "minecraft:worldgen/biome",
                    "value": NbtList::from(vec![
                        compound! {
                            "name": "minecraft:ocean",
                            "id": 0_i32,
                            "element": compound! {
                                "precipitation": "none",
                                "depth": 0.0_f32,
                                "temperature": 0.0_f32,
                                "scale": 0.0_f32,
                                "downfall": 0.0_f32,
                                "category": "ocean",
                                "effects": compound! {
                                    "sky_color": 0x7fa1ff_i32,
                                    "water_fog_color": 0x7fa1ff_i32,
                                    "fog_color": 0x7fa1ff_i32,
                                    "water_color": 0x7fa1ff_i32,
                                }
                            },
                        },
                    ]),
                },
            },
            dimension: compound! {
                "piglin_safe": 0_i8,
                "natural": 1_i8,
                "ambient_light": 1.0_f32,
                "infiniburn": "#minecraft:infiniburn_overworld",
                "respawn_anchor_works": 0_i8,
                "has_skylight": 1_i8,
                "bed_works": 0_i8,
                "effects": "minecraft:overworld",
                "has_raids": 0_i8,
                "min_y": 0_i32,
                "height": 1_i32,
                "logical_height": 0_i32,
                "coordinate_scale": 1.0_f64,
                "ultrawarm": 0_i8,
                "has_ceiling": 0_i8,
            },
            dim_current: "minecraft:ocean".to_string(),
            seed_hash: 0,
            max_players: 1,
            view_dist: 8, // TODO:
            sim_dist: 8,
            reduce_debug: false,
            respawn_screen: false,
            debug_world: false,
            flat_world: true,
        }).unwrap();
    }
}

impl LoginStartPacket {
    pub fn new(packet: GenericPacket) -> io::Result<Self> {
        let mut data = &packet.data[..];
        let player_name = data.read_string()?;

        Ok(Self {
            player_name
        })
    }
}

#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub uuid: u128,
    pub player_name: String
}

impl ClientPacket for LoginSuccessPacket {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut p = PacketWriter::new(0x02);
        p.write_be(self.uuid);
        p.write_string(&self.player_name);

        p.export(w)
    }
}

#[derive(Debug)]
pub struct JoinGamePacket {
    pub eid: i32,
    pub hardcore: bool,
    pub cgm: u8,
    pub pgm: i8,
    pub dim_names: Vec<String>,
    pub dim_codec: NbtCompound,
    pub dimension: NbtCompound,
    pub dim_current: String,
    pub seed_hash: i64,
    pub max_players: i32,
    pub view_dist: i32,
    pub sim_dist: i32,
    pub reduce_debug: bool,
    pub respawn_screen: bool,
    pub debug_world: bool,
    pub flat_world: bool
}

impl ClientPacket for JoinGamePacket {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut p = PacketWriter::new(0x26);
        p.write_be(self.eid);
        p.write_be(self.hardcore);
        p.write_be(self.cgm);
        p.write_be(self.pgm);
        p.write_varint(self.dim_names.len() as i32);
        for d in self.dim_names.iter() {
            p.write_string(d);
        }
        p.write_nbt_compound(&self.dim_codec, None, quartz_nbt::io::Flavor::Uncompressed);
        p.write_nbt_compound(&self.dimension, None, quartz_nbt::io::Flavor::Uncompressed);
        p.write_string(&self.dim_current);
        p.write_be(self.seed_hash);
        p.write_varint(self.max_players);
        p.write_varint(self.view_dist);
        p.write_varint(self.sim_dist);
        p.write_be(self.reduce_debug);
        p.write_be(self.respawn_screen);
        p.write_be(self.debug_world);
        p.write_be(self.flat_world);

        p.export(w)
    }
}
