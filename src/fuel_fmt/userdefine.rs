use binwrite::BinWrite;
use nom_derive::NomLE;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormat, PascalString, ResourceObjectZ};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct UserDefineZ {
    data: PascalString,
}

pub type UserDefineObjectFormat = FUELObjectFormat<ResourceObjectZ, UserDefineZ>;
