use std::fs;
use std::io::{Error, Read, Write};
use std::io::Cursor;
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};
use binwrite::BinWrite;

use crate::{File, lz};
use crate::fuel_fmt::common::{FUELObjectFormatTrait, write_option};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SoundZHeader {
    friendly_name_crc32: u32,
    #[serde(skip_serializing)]
    sample_rate: u32,
    #[nom(Cond = "sample_rate != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    data_size: Option<u32>,
    #[nom(Cond = "sample_rate != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    sound_type: Option<u16>,
    #[nom(Cond = "sample_rate != 0 && i.len() == 2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero: Option<u16>,
}

pub struct SoundObjectFormat;

impl SoundObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl FUELObjectFormatTrait for SoundObjectFormat {
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

        let wav_path = output_path.join("data.wav");

        let sound_header = match SoundZHeader::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: if sound_header.sample_rate != 0 {
                sound_header.sample_rate
            } else {
                44100
            },
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let number_of_samples = data.len() as u32 / (spec.bits_per_sample / 8) as u32;

        let mut parent_writer = hound::WavWriter::create(wav_path, spec).unwrap();
        let mut writer = parent_writer.get_i16_writer(number_of_samples);

        let mut data_cursor = Cursor::new(&data);

        for _ in 0..number_of_samples {
            writer.write_sample(data_cursor.read_i16::<LittleEndian>()?);
        }

        #[derive(Serialize)]
        struct Object {
            sound_header: SoundZHeader,
        }

        let object = Object { sound_header };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        Ok(())
    }
}
