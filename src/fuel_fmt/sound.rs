use std::io::Write;
use std::io::{Cursor, Result};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::File;

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SoundZHeader {
    friendly_name_crc32: u32,
    #[serde(skip_serializing)]
    sample_rate: u32,
    #[nom(Cond = "sample_rate != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_size: Option<u32>,
    #[nom(Cond = "sample_rate != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sound_type: Option<u16>,
    #[nom(Cond = "sample_rate != 0 && i.len() == 2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zero: Option<u16>,
}

#[derive(Serialize, Deserialize)]
struct SoundObject {
    sound_header: SoundZHeader,
}

pub fn fuel_fmt_extract_sound_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
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

    let object = SoundObject { sound_header };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
