use std::io::Result;
use std::io::Write;
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{ResourceObjectZ, Vec2f};
use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
struct GwRoadZPoint {
    encoded_vec2hf: u32,
    a: u8,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GwRoadZRoad {
    road_type: u8,
    point_count: u16,
    #[nom(Count(point_count))]
    points: Vec<GwRoadZPoint>,
}

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

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct GwRoadZ {
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

#[derive(Serialize, Deserialize)]
struct GwRoadObject {
    resource_object: ResourceObjectZ,
    gw_road: GwRoadZ,
}

pub fn fuel_fmt_extract_gw_road_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let gw_road = match GwRoadZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = GwRoadObject {
        resource_object,
        gw_road,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
