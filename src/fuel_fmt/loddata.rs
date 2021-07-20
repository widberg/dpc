use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, FixedVec, PascalArray, ResourceObjectZ};

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
    #[allow(dead_code)]
    opt: u8,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<FixedVec<u8, 24>>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    u1: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero1: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    u2: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero2: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero3: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero4: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_x: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_y: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_z: Option<f32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero5: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero6: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero7: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    u6: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero8: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero9: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero10: Option<u32>,
    #[nom(Cond = "opt != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero11: Option<u32>,
}

pub type LodDataObjectFormat = FUELObjectFormat<ResourceObjectZ, LodDataZ>;
