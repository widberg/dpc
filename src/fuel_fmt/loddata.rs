use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    write_option, FUELObjectFormat, FixedVec, PascalArray, ResourceObjectZ,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LodDataZ {
    unknown_byte0: u8,
    unknown_byte1: u8,
    zero_byte0: u8,
    zero_byte1: u8,
    crc32s: PascalArray<u32>,
    zero0: u32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    #[binwrite(ignore)]
    opt: u8,
    #[binwrite(postprocessor(|x: Vec<u8>| -> (u8, Vec<u8>) { if x.len() != 0 { (1u8, x) } else { (0u8, x) } }))]
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    padding: Option<FixedVec<u8, 24>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    u1: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero1: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    u2: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero2: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero3: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero4: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    scale_x: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    scale_y: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    scale_z: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero5: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero6: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero7: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    u6: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero8: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero9: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero10: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero11: Option<u32>,
}

pub type LodDataObjectFormat = FUELObjectFormat<ResourceObjectZ, LodDataZ>;
