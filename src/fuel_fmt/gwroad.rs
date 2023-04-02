use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, HasReferences, ResourceObjectZ, Vec2f};

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
#[serde(from = "GwRoadZShadow")]
#[nom(Exact)]
pub struct GwRoadZ {
    #[serde(skip)]
    road_count: u32,
    gen_road_min: Vec2f,
    gen_road_max: Vec2f,
    #[nom(Count(road_count))]
    roads: Vec<GwRoadZRoad>,
    #[serde(skip)]
    unknown5_count: u32,
    unknown5_min: Vec2f,
    unknown5_max: Vec2f,
    #[nom(Count(unknown5_count))]
    unknown5s: Vec<GwRoadZUnknown5>,
    unknown_crc32: u32,
}

#[derive(Deserialize)]
pub struct GwRoadZShadow {
    gen_road_min: Vec2f,
    gen_road_max: Vec2f,
    roads: Vec<GwRoadZRoad>,
    unknown5_min: Vec2f,
    unknown5_max: Vec2f,
    unknown5s: Vec<GwRoadZUnknown5>,
    unknown_crc32: u32,
}

impl From<GwRoadZShadow> for GwRoadZ {
    fn from(shadow: GwRoadZShadow) -> Self {
        GwRoadZ {
            road_count: shadow.roads.len() as u32,
            gen_road_min: shadow.gen_road_min,
            gen_road_max: shadow.gen_road_max,
            roads: shadow.roads,
            unknown5_count: shadow.unknown5s.len() as u32,
            unknown5_min: shadow.unknown5_min,
            unknown5_max: shadow.unknown5_max,
            unknown5s: shadow.unknown5s,
            unknown_crc32: shadow.unknown_crc32
        }
    }
}

impl HasReferences for GwRoadZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        if self.unknown_crc32 != 0 { v.push(self.unknown_crc32) }
        v
    }
}

pub type GwRoadObjectFormat = FUELObjectFormat<ResourceObjectZ, GwRoadZ>;
