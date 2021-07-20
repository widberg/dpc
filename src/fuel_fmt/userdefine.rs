use nom::number::complete::*;
use nom::*;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, ResourceObjectZ};

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct UserDefineZ {
    #[nom(
        Map = "|x: Vec<u8>| String::from_utf8_lossy(&x[..]).to_string()",
        Parse = "|i| length_count!(i, le_u32, le_u8)"
    )]
    data: String,
}

pub type UserDefineObjectFormat = FUELObjectFormat<ResourceObjectZ, UserDefineZ>;
