use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

use nom_derive::Parse;
use serde::{Serialize, Deserialize};
use binwrite::BinWrite;

use crate::fuel_fmt::common::{FUELObjectFormatTrait, ResourceObjectZ};

pub struct BinaryObjectFormat;

impl BinaryObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl FUELObjectFormatTrait for BinaryObjectFormat {
    fn pack(self: &Self, input_path: &Path, header: &mut Vec<u8>, body: &mut Vec<u8>) -> Result<(), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        let bin_path = input_path.join("data.bin");
        let mut bin_file = File::create(bin_path)?;


        #[derive(Deserialize)]
        struct Object {
            resource_object: ResourceObjectZ,
        }

        let object: Object = serde_json::from_reader(json_file)?;

        object.resource_object.write(header)?;
        bin_file.write(body)?;

        Ok(())
    }

    fn unpack(self: &Self, header: &[u8], body: &[u8], output_path: &Path) -> Result<(), Error> {
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

        output_bin_file.write(&body)?;

        Ok(())
    }
}
