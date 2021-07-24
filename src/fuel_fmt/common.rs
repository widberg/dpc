use std::fs::File;
use std::io::{Error, Write};
use std::marker::PhantomData;
use std::path::Path;

use binwrite::{BinWrite, WriterOption};
pub use nom::number::complete::*;
pub use nom::*;
pub use nom_derive::NomLE;
use nom_derive::Parse;
pub use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ResourceObjectZ {
    friendly_name_crc32: u32,
    #[nom(Cond = "i.len() != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    pub crc32s: Option<PascalArray<u32>>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
pub struct Vec3f {
    x: f32,
    z: f32,
    y: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
pub struct Vec4f {
    x: f32,
    z: f32,
    y: f32,
    w: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
pub struct Vec3i32 {
    x: i32,
    z: i32,
    y: i32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
pub struct Vec2f {
    x: f32,
    y: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
pub struct Mat4f {
    data: FixedVec<f32, 16>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

pub fn write_option<W, T>(
    option: &Option<T>,
    writer: &mut W,
    options: &WriterOption,
) -> Result<(), Error>
    where
        W: Write,
        T: BinWrite,
{
    if let Some(value) = option {
        BinWrite::write_options(value, writer, options)
    } else {
        Ok(())
    }
}

#[derive(NomLE)]
pub struct PascalArray<T> {
    #[nom(LengthCount(le_u32))]
    data: Vec<T>,
}

impl<T> BinWrite for PascalArray<T>
where
    T: BinWrite,
{
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<(), Error> {
        BinWrite::write_options(&(self.data.len() as u32), writer, options)?;
        BinWrite::write_options(&self.data, writer, options)
    }
}

impl<T> PascalArray<T> {
    pub fn len(self: &Self) -> usize {
        self.data.len()
    }
}

impl<T> Serialize for PascalArray<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for PascalArray<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PascalArray {
            data: Vec::deserialize(deserializer)?,
        })
    }
}

#[derive(NomLE)]
pub struct PascalString {
    #[nom(
        Map = "|x: Vec<u8>| String::from_utf8_lossy(&x[..]).to_string()",
        Parse = "|i| length_count!(i, le_u32, le_u8)"
    )]
    data: String,
}

impl BinWrite for PascalString
{
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<(), Error> {
        BinWrite::write_options(&(self.data.len() as u32), writer, options)?;
        BinWrite::write_options(&self.data, writer, options)
    }
}

impl Serialize for PascalString
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PascalString
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(PascalString {
            data: String::deserialize(deserializer)?,
        })
    }
}

#[derive(NomLE)]
pub struct PascalStringNULL {
    #[nom(
        Map = "|x: Vec<u8>| String::from_utf8_lossy(&x[0..x.len() - 1]).to_string()",
        Parse = "|i| length_count!(i, le_u32, le_u8)"
    )]
    data: String,
}

impl BinWrite for PascalStringNULL
{
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<(), Error> {
        BinWrite::write_options(&(self.data.len() as u32 + 1u32), writer, options)?;
        BinWrite::write_options(&self.data, writer, options)?;
        BinWrite::write_options(&[0u8], writer, options)
    }
}

impl Serialize for PascalStringNULL
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PascalStringNULL
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(PascalStringNULL {
            data: String::deserialize(deserializer)?,
        })
    }
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(NomLE)]
pub struct FixedVec<T: BinWrite, const U: usize> {
    #[nom(Count(U))]
    data: Vec<T>,
}

impl<T: BinWrite, const U: usize> Serialize for FixedVec<T, U>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de, T: BinWrite, const U: usize> Deserialize<'de> for FixedVec<T, U>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(FixedVec {
            data: Vec::deserialize(deserializer)?,
        })
    }
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ObjectZ {
    friendly_name_crc32: u32,
    crc32_or_zero: u32,
    #[nom(Cond = "i.len() != 90", Count = "crc32_or_zero as usize + 1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    crc32s: Option<Vec<u32>>,
    rot: Quat,
    transform: Mat4f,
    unknown2: f32,
    unknown0: f32,
    unknown1: u16,
}

pub trait FUELObjectFormatTrait {
    fn pack(self: &Self, input_path: &Path, header: &mut Vec<u8>, body: &mut Vec<u8>) -> Result<(), Error>;
    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error>;
}

pub struct FUELObjectFormat<T, U> {
    x: PhantomData<T>,
    y: PhantomData<U>,
}

impl<T, U> FUELObjectFormat<T, U> {
    pub fn new<'a>() -> &'a Self {
        &Self {
            x: PhantomData,
            y: PhantomData,
        }
    }
}

impl<T, U> FUELObjectFormatTrait for FUELObjectFormat<T, U>
where
    for<'a> T: Parse<&'a [u8]> + Serialize + Deserialize<'a> + BinWrite,
    for<'a> U: Parse<&'a [u8]> + Serialize + Deserialize<'a> + BinWrite,
{
    fn pack(self: &Self, input_path: &Path, header: &mut Vec<u8>, body: &mut Vec<u8>) -> Result<(), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        #[derive(Serialize, Deserialize)]
        struct Object<T, U> {
            header: T,
            body: U,
        }

        let object: Object<T, U> = serde_json::from_reader(json_file)?;

        object.header.write(header)?;
        object.body.write(body)?;

        Ok(())
    }

    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let header = match T::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let body = match U::parse(&body) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        #[derive(Serialize, Deserialize)]
        struct Object<T, U> {
            header: T,
            body: U,
        }

        let object = Object { header, body };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        Ok(())
    }
}
