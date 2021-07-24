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
        dds.get_data(0).unwrap().write(body)?;

        // let bmp_path = input_path.join("data.bmp");
        // let bmp_file = File::open(bmp_path)?;
        //
        // let mut object: BitmapObject = serde_json::from_reader(json_file)?;
        // let bmp_decoder = BmpDecoder::new(bmp_file).unwrap();
        //
        // let (width, height) = (&bmp_decoder).dimensions();
        //
        // object.bitmap_header.width = width;
        // object.bitmap_header.height = height;
        // object.bitmap_header.data_size = (&bmp_decoder).total_bytes() as u32 / 4;
        //
        // object.bitmap_header.write(header)?;
        //
        // let mut buf: Vec<u32> = vec![0; (&bmp_decoder).total_bytes() as usize / 4];
        //
        // if object.bitmap_header.dxt_version0 == 14 {
        //     assert_eq!((&bmp_decoder).color_type(), ColorType::Rgb8);
        //     assert_eq!(DXT1.color_type(), ColorType::Rgb8);
        // } else {
        //     assert_eq!((&bmp_decoder).color_type(), ColorType::Rgba8);
        //     assert_eq!(DXT5.color_type(), ColorType::Rgba8);
        // }
        // bmp_decoder.read_image(buf.as_bytes_mut()).unwrap();
        //
        // let out_bmp_path = input_path.join("dataaa.bmp");
        // let mut output_bmp_file = File::create(out_bmp_path)?;
        // let mut bmp_encoder = BmpEncoder::new(&mut output_bmp_file);
        // bmp_encoder
        //     .encode(
        //         buf.as_bytes(),
        //         object.bitmap_header.width,
        //         object.bitmap_header.height,
        //         if object.bitmap_header.dxt_version0 == 14 {
        //             ColorType::Rgb8
        //         } else {
        //             ColorType::Rgba8
        //         },
        //     )
        //     .unwrap();
        //
        // let out_dxt_path = input_path.join("dataaa.dxt");
        // let mut output_dxt_file = File::create(out_dxt_path)?;
        // let dxt_encoder = DxtEncoder::new(&output_dxt_file);
        // dxt_encoder
        //     .encode(
        //         buf.as_bytes(),
        //         object.bitmap_header.width,
        //         object.bitmap_header.height,
        //         if object.bitmap_header.dxt_version0 == 14 {
        //             DXT1
        //         } else {
        //             DXT5
        //         },
        //     )
        //     .unwrap();

        // body.resize(output_dxt_file.stream_position()? as usize, 0);
        // output_dxt_file.seek(SeekFrom::Start(0))?;
        // output_dxt_file.read(body.as_bytes_mut())?;

        Ok(())
    }

    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        // let bmp_path = output_path.join("data.bmp");
        // let mut output_bmp_file = File::create(bmp_path)?;

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
            }, Some(bitmap_header.mip_map_count as u32), None).unwrap();

        dds.get_mut_data(0).unwrap() = &mut *Vec::from(body);

        dds.write(&mut output_dds_file).unwrap();

        // let data_cursor = Cursor::new(&body);
        // let dxt_decoder = DxtDecoder::new(
        //     data_cursor,
        //     bitmap_header.width,
        //     bitmap_header.height,
        //     if bitmap_header.dxt_version0 == 14 {
        //         DXTVariant::DXT1
        //     } else {
        //         DXTVariant::DXT5
        //     },
        // )
        // .unwrap();
        //
        // let mut buf: Vec<u32> = vec![0; dxt_decoder.total_bytes() as usize / 4];
        // dxt_decoder.read_image(buf.as_bytes_mut()).unwrap();
        //
        // let mut bmp_encoder = BmpEncoder::new(&mut output_bmp_file);
        // bmp_encoder
        //     .encode(
        //         buf.as_bytes(),
        //         bitmap_header.width,
        //         bitmap_header.height,
        //         if bitmap_header.dxt_version0 == 14 {
        //             ColorType::Rgb8
        //         } else {
        //             ColorType::Rgba8
        //         },
        //     )
        //     .unwrap();

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

        let dds_path = input_path.join("data.dds");
        let mut dds_file = File::open(dds_path)?;
        let dds = Dds::read(&mut dds_file).unwrap();

        let mut object: BitmapObjectAlternate = serde_json::from_reader(json_file)?;

        object.bitmap.width = dds.get_width();
        object.bitmap.height = dds.get_height();

        object.bitmap_header.write(header)?;

        object.bitmap.data.resize(0, 0);
        object.bitmap.write(body)?;

        dds.data.write(body)?;

        Ok(())
    }

    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let dds_path = output_path.join("data.dds");
        let mut output_dds_file = File::create(dds_path)?;

        let bitmap_header = match BitmapZHeaderAlternate::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let bitmap = match BitmapZAlternate::parse(&body) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let mut dds = Dds::new_d3d(bitmap.height, bitmap.width, None, if bitmap_header.dxt_version0 == 7 {
                D3DFormat::A8L8
            }else if bitmap_header.dxt_version0 == 14 {
                D3DFormat::DXT1
            } else {
                D3DFormat::DXT5
            }, Some(0), None).unwrap();
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
