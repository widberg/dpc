use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{write_option, FUELObjectFormat, HasReferences, ObjectZ, PascalArray, DynSphere, DynBox};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct LodZSoundEntry {
    id: u32,
    sound_crc32: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct LodZUnknown4 {
    a: u32,
    b: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LodZ {
    dyn_spheres: PascalArray<DynSphere>,
    dyn_boxes: PascalArray<DynBox>,
    close_x: f32,
    close_y: f32,
    close_z: f32,
    skin_crc32s: PascalArray<u32>,
    zero: u32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    sound_entries_option: u32,
    #[nom(Cond(sound_entries_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u32, Vec<u8>) { if x.len() != 0 { (1u32, x) } else { (0u32, x) } }))]
    #[binwrite(with(write_option))]
    sound_entries: Option<PascalArray<LodZSoundEntry>>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    sound_entries_option1: u32,
    #[nom(Cond(sound_entries_option1 != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u32, Vec<u8>) { if x.len() != 0 { (1u32, x) } else { (0u32, x) } }))]
    #[binwrite(with(write_option))]
    sound_entries1: Option<PascalArray<LodZSoundEntry>>,
    user_define_crc32: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LodZAlt {
    x: u32,
    #[nom(Cond(x != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unused0: Option<u32>,
    #[nom(Cond(x != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    sphere_col_node_optional: Option<DynSphere>,
    sphere_col_nodes: PascalArray<DynSphere>,
    box_cols: PascalArray<DynBox>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    u0: f32,
    skin_crc32s: PascalArray<u32>,
    u1: u32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    sound_entries_option: u8,
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u8, Vec<u8>) { if x.len() != 0 { (1u8, x) } else { (0u8, x) } }))]
    #[nom(Cond(sound_entries_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    sound_entries: Option<PascalArray<LodZSoundEntry>>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    unknown4_option: u8,
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u8, Vec<u8>) { if x.len() != 0 { (1u8, x) } else { (0u8, x) } }))]
    #[nom(Cond(unknown4_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown4s: Option<PascalArray<LodZUnknown4>>,
    unknown5: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LodZAltAlt {
    x: u32,
    #[nom(Cond(x != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unused0: Option<u32>,
    #[nom(Cond(x != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    sphere_col_node_optional: Option<DynSphere>,
    sphere_col_nodes: PascalArray<DynSphere>,
    box_cols: PascalArray<DynBox>,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    u0: f32,
    skin_crc32s: PascalArray<u32>,
    u1: u32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    sound_entries_option: u32,
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u32, Vec<u8>) { if x.len() != 0 { (1u32, x) } else { (0u32, x) } }))]
    #[nom(Cond(sound_entries_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    sound_entries: Option<PascalArray<LodZSoundEntry>>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    unknown4_option: u32,
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u32, Vec<u8>) { if x.len() != 0 { (1u32, x) } else { (0u32, x) } }))]
    #[nom(Cond(unknown4_option != 0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown4s: Option<PascalArray<LodZUnknown4>>,
    unknown5: u32,
}

impl HasReferences for LodZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        v.append(&mut self.skin_crc32s.data.clone());
        if let Some(sound_entries) = &self.sound_entries {
            v.append(&mut sound_entries.data.iter().map(|x| x.sound_crc32).collect());
        }
        if let Some(sound_entries1) = &self.sound_entries1 {
            v.append(&mut sound_entries1.data.iter().map(|x| x.sound_crc32).collect());
        }
        if self.user_define_crc32 != 0 { v.push(self.user_define_crc32) }
        v
    }
}

impl HasReferences for LodZAlt {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

impl HasReferences for LodZAltAlt {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type LodObjectFormat = FUELObjectFormat<ObjectZ, LodZ>;
pub type LodObjectFormatAlt = FUELObjectFormat<ObjectZ, LodZAlt>;
pub type LodObjectFormatAltAlt = FUELObjectFormat<ObjectZ, LodZAltAlt>;
