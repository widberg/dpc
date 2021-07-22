use std::io::Cursor;
use std::io::{Error, Write};
use std::path::Path;

use binwrite::BinWrite;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use image::codecs::dxt::{DXTVariant, DxtDecoder};
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageDecoder};
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};
use zerocopy::AsBytes;

use crate::fuel_fmt::common::{write_option, FUELObjectFormatTrait};
use crate::File;

#[derive(BinWrite)]
#[binwrite(little)]
// https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dds-header
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct BitmapZHeader {
    friendly_name_crc32: u32,
    dw_caps2: u16,
    #[serde(skip_serializing)]
    width: u32,
    #[serde(skip_serializing)]
    height: u32,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
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
    width: u32,
    #[serde(skip_serializing)]
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
    fn pack(self: &Self, _input_path: &Path, _output_path: &Path) -> Result<(), Error> {
        todo!()
    }

    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let png_path = output_path.join("data.png");
        let output_png_file = File::create(png_path)?;

        let bitmap_header = match BitmapZHeader::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let data_cursor = Cursor::new(&body);
        let dxt_decoder = DxtDecoder::new(
            data_cursor,
            bitmap_header.width,
            bitmap_header.height,
            if bitmap_header.dxt_version0 == 14 {
                DXTVariant::DXT1
            } else {
                DXTVariant::DXT5
            },
        )
        .unwrap();

        let mut buf: Vec<u32> = vec![0; dxt_decoder.total_bytes() as usize / 4];
        dxt_decoder.read_image(buf.as_bytes_mut()).unwrap();

        let png_encoder = PngEncoder::new(output_png_file);
        png_encoder
            .encode(
                buf.as_bytes(),
                bitmap_header.width,
                bitmap_header.height,
                if bitmap_header.dxt_version0 == 14 {
                    ColorType::Rgb8
                } else {
                    ColorType::Rgba8
                },
            )
            .unwrap();

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
    fn pack(self: &Self, _input_path: &Path, _output_path: &Path) -> Result<(), Error> {
        todo!()
    }

    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let png_path = output_path.join("data.png");
        let output_png_file = File::create(png_path)?;

        let bitmap_header = match BitmapZHeaderAlternate::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let bitmap = match BitmapZAlternate::parse(&body) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        if bitmap_header.dxt_version0 == 7 {
            let png_encoder = PngEncoder::new(output_png_file);
            png_encoder
                .encode(
                    bitmap.data.as_bytes(),
                    bitmap.width,
                    bitmap.height,
                    ColorType::L16,
                )
                .unwrap();
        } else {
            let data_cursor = Cursor::new(&bitmap.data[..]);
            let dxt_decoder = DxtDecoder::new(
                data_cursor,
                bitmap.width,
                bitmap.height,
                if bitmap_header.dxt_version0 == 14 {
                    DXTVariant::DXT1
                } else {
                    DXTVariant::DXT5
                },
            )
            .unwrap();

            let mut buf: Vec<u32> = vec![0; dxt_decoder.total_bytes() as usize / 4];
            dxt_decoder.read_image(buf.as_bytes_mut()).unwrap();

            let png_encoder = PngEncoder::new(output_png_file);
            png_encoder
                .encode(
                    buf.as_bytes(),
                    bitmap.width,
                    bitmap.height,
                    if bitmap_header.dxt_version0 == 14 {
                        ColorType::Rgb8
                    } else {
                        ColorType::Rgba8
                    },
                )
                .unwrap();
        }

        let object = BitmapObjectAlternate {
            bitmap_header,
            bitmap,
        };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        Ok(())
    }
}
