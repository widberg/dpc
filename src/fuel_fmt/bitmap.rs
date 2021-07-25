use std::io::{Error, Write};
use std::path::Path;

use binwrite::BinWrite;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{write_option, FUELObjectFormatTrait};
use crate::File;
use ddsfile::{Dds, D3DFormat};

#[derive(BinWrite)]
#[binwrite(little)]
// https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dds-header
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct BitmapZHeader {
    friendly_name_crc32: u32,
    dw_caps2: u16,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    width: u32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    height: u32,
    data_size: u32,
    u1: u8,
    bitmap_type: u8,
    zero: u16,
    u7: f32,
    dxt_version0: u8,
    mip_map_count: u8,
    u2: u8,
    u3: u8,
    dxt_version1: u8,
    u4: u8,
}

#[derive(Serialize, Deserialize)]
struct BitmapObject {
    bitmap_header: BitmapZHeader,
}

// alternate

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct BitmapZHeaderAlternate {
    friendly_name_crc32: u32,
    zero0: u32,
    unknown0: u8,
    dxt_version0: u8,
    unknown1: u8,
    zero1: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct BitmapZAlternate {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    width: u32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    height: u32,
    zero0: u32,
    unknown0: u32,
    #[nom(Cond = "LittleEndian::read_u32(&i[0..4]) == 0")]
    #[binwrite(with(write_option))]
    zero1: Option<u32>,
    unknown1: u16,
    unknown2: u8,
    #[nom(Count = "i.len()")]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct BitmapObjectAlternate {
    bitmap_header: BitmapZHeaderAlternate,
    bitmap: BitmapZAlternate,
}

pub struct BitmapObjectFormat;

impl BitmapObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl FUELObjectFormatTrait for BitmapObjectFormat {
    fn pack(self: &Self, input_path: &Path, header: &mut Vec<u8>, body: &mut Vec<u8>) -> Result<(), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        let dds_path = input_path.join("data.dds");
        let mut dds_file = File::open(dds_path)?;

        let dds = Dds::read(&mut dds_file).unwrap();

        let mut object: BitmapObject = serde_json::from_reader(json_file)?;
        object.bitmap_header.width = dds.get_width();
        object.bitmap_header.height = dds.get_height();
        object.bitmap_header.write(header)?;
        dds.data.write(body)?;

        Ok(())
    }

    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let bitmap_header = match BitmapZHeader::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let dds_path = output_path.join("data.dds");
        let mut output_dds_file = File::create(dds_path)?;

        let mut dds = Dds::new_d3d(bitmap_header.height, bitmap_header.width, None, if bitmap_header.dxt_version0 == 14 {
                D3DFormat::DXT1
            } else {
                D3DFormat::DXT5
            }, Some(bitmap_header.mip_map_count as u32), None
        )
        .unwrap();

        dds.data = Vec::from(body);

        dds.write(&mut output_dds_file).unwrap();


        let object = BitmapObject { bitmap_header };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        Ok(())
    }
}

pub struct BitmapObjectFormatAlt;

impl BitmapObjectFormatAlt {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl FUELObjectFormatTrait for BitmapObjectFormatAlt {
    fn pack(self: &Self, input_path: &Path, header: &mut Vec<u8>, body: &mut Vec<u8>) -> Result<(), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        let mut object: BitmapObjectAlternate = serde_json::from_reader(json_file)?;

        object.bitmap_header.write(header)?;

        let dds_path = input_path.join("data.dds");
        let mut dds_file = File::open(dds_path)?;

        let dds = Dds::read(&mut dds_file).unwrap();

        object.bitmap.width = dds.get_width();
        object.bitmap.height = dds.get_height();

        object.bitmap.data.clear();
        object.bitmap.write(body)?;

        dds.data.write(body).unwrap();

        Ok(())
    }

    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let bitmap_header = match BitmapZHeaderAlternate::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let bitmap = match BitmapZAlternate::parse(body) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let dds_path = output_path.join("data.dds");
        let mut output_dds_file = File::create(dds_path)?;

        let mut dds = Dds::new_d3d(bitmap.height, bitmap.width, None, if bitmap_header.dxt_version0 == 7 {
          D3DFormat::A8L8
        } else if bitmap_header.dxt_version0 == 14 {
            D3DFormat::DXT1
        } else {
            D3DFormat::DXT5
        }, Some(0), None
        )
            .unwrap();

        dds.data = bitmap.data.clone();

        dds.write(&mut output_dds_file).unwrap();

        let object = BitmapObjectAlternate {
            bitmap_header,
            bitmap,
        };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        Ok(())
    }
}
