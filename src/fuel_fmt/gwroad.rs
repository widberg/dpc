use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;

use crate::fuel_fmt::common::{ResourceObjectZ};

#[derive(Serialize, Deserialize, NomLE)]
struct GwRoadZUnknown00
{
    unknown0: u8,
    unknown1: u8,
    unknown2: u8,
    unknown3: u8,
    unknown4: u8,
}

#[derive(Serialize, Deserialize, NomLE)]
struct GwRoadZUnknown0
{
    flag: u8,
    unknown00_count: u32,
    #[nom(Count(unknown00_count))]
    unknown00s: Vec<GwRoadZUnknown00>
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
    unknown0_count: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    #[nom(Count(unknown0_count))]
    unknown0s: Vec<GwRoadZUnknown0>,
    unknown5_count: u32,
    unknown6: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: u32,
    #[nom(Count(unknown5_count))]
    unknown5s: Vec<GwRoadZUnknown5>,
    unknown10: u32,
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
