use std::{fs, io};
use std::fs::File;
use std::io::{Error, Read, Write};
use std::marker::PhantomData;
use std::path::Path;

pub use nom::number::complete::*;
pub use nom_derive::NomLE;
use nom_derive::Parse;
pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
use binwrite::{BinWrite, WriterOption};

use crate::lz;

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct ResourceObjectZ {
    friendly_name_crc32: u32,
    #[nom(Cond = "i.len() != 0")]
    #[nom(LengthCount = "le_u32")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    pub crc32s: Option<Vec<u32>>,
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
    #[nom(Count(16))]
    data: Vec<f32>,
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

#[derive(NomLE)]
pub struct PascalArray<T> {
    #[nom(LengthCount(le_u32))]
    data: Vec<T>,
}

impl<T> BinWrite for PascalArray<T>
where
    T: BinWrite,
{
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        BinWrite::write_options(&(self.data.len() as u32), writer, options)?;
        BinWrite::write_options(&self.data, writer, options)
    }
}

pub fn write_option<W, T>(option: &Option<T>, writer: &mut W, options: &WriterOption) -> Result<(), Error>
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
    fn pack(self: &Self, input_path: &Path, output_path: &Path) -> Result<(), io::Error>;
    fn unpack(self: &Self, input_path: &Path, output_path: &Path) -> Result<(), io::Error>;
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
    for<'a> T: Parse<&'a [u8]> + Serialize + Deserialize<'a>,
    for<'a> U: Parse<&'a [u8]> + Serialize + Deserialize<'a>,
{
    fn pack(self: &Self, _input_path: &Path, _output_path: &Path) -> Result<(), Error> {
        todo!()
    }

    fn unpack(self: &Self, input_path: &Path, output_path: &Path) -> Result<(), Error> {
        fs::create_dir_all(output_path)?;

        let mut input_file = File::open(input_path)?;

        let mut object_header_buffer = [0; 24];
        input_file.read(&mut object_header_buffer)?;

        #[derive(NomLE)]
        #[allow(dead_code)]
        struct ObjectHeader {
            data_size: u32,
            class_object_size: u32,
            decompressed_size: u32,
            compressed_size: u32,
            class_crc32: u32,
            crc32: u32,
        }
        let object_header = match ObjectHeader::parse(&object_header_buffer) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let mut header = vec![0; object_header.class_object_size as usize];
        input_file.read(&mut header)?;

        let mut data = vec![0; object_header.decompressed_size as usize];

        if object_header.compressed_size != 0 {
            let mut compresssed_data = vec![0; object_header.compressed_size as usize];
            input_file.read(&mut compresssed_data)?;
            lz::lzss_decompress(
                &compresssed_data[..],
                object_header.compressed_size as usize,
                &mut data[..],
                object_header.decompressed_size as usize,
                false,
            )?;
        } else {
            input_file.read(&mut data)?;
        }

        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let header = match T::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let body = match U::parse(&data) {
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
