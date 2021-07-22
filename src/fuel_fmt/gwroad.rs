use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, ResourceObjectZ, Vec2f};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct GwRoadZPoint {
    encoded_vec2hf: u32,
    a: u8,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct GwRoadZRoad {
    road_type: u8,
    point_count: u16,
    #[nom(Count(point_count))]
    points: Vec<GwRoadZPoint>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct GwRoadZUnknown5 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
    #[nom(Count(unknown0 as usize & 0xFFFF))]
    unknown8s: Vec<u32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct GwRoadZ {
    road_count: u32,
    gen_road_min: Vec2f,
    gen_road_max: Vec2f,
    #[nom(Count(road_count))]
    roads: Vec<GwRoadZRoad>,
    unknown5_count: u32,
    unknown5_min: Vec2f,
    unknown5_max: Vec2f,
    #[nom(Count(unknown5_count))]
    unknown5s: Vec<GwRoadZUnknown5>,
    unknown_crc32: u32,
}

pub type GwRoadObjectFormat = FUELObjectFormat<ResourceObjectZ, GwRoadZ>;
