use std::fs;
use std::io::{Error, Read, Write};
use std::io::Cursor;
use std::path::Path;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use image::{ColorType, ImageDecoder};
use image::codecs::dxt::{DxtDecoder, DXTVariant};
use image::codecs::png::PngEncoder;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};
use zerocopy::AsBytes;

use crate::{File, lz};
use crate::fuel_fmt::common::FUELObjectFormatTrait;

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

        let png_path = output_path.join("data.png");
        let output_png_file = File::create(png_path)?;

        let bitmap_header = match BitmapZHeader::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let data_cursor = Cursor::new(&data);
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

        let png_path = output_path.join("data.png");
        let output_png_file = File::create(png_path)?;

        let bitmap_header = match BitmapZHeaderAlternate::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let bitmap = match BitmapZAlternate::parse(&data) {
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

