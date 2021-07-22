use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

use nom_derive::{NomLE, Parse};
use serde::Serialize;

use crate::fuel_fmt::common::{FUELObjectFormatTrait, ResourceObjectZ};
use crate::lz;

pub struct BinaryObjectFormat;

impl BinaryObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl FUELObjectFormatTrait for BinaryObjectFormat {
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

        let bin_path = output_path.join("data.bin");
        let mut output_bin_file = File::create(bin_path)?;

        let resource_object = match ResourceObjectZ::parse(&header) {
            Ok((_, h)) => h,
            Err(error) => panic!("{}", error),
        };

        #[derive(Serialize)]
        struct Object {
            resource_object: ResourceObjectZ,
        }

        let object = Object { resource_object };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        output_bin_file.write(&data)?;

        Ok(())
    }
}
