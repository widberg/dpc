use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

use binwrite::BinWrite;
use nom_derive::Parse;
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{FUELObjectFormatTrait, HasReferences, ResourceObjectZ};
use std::fs;
use zerocopy::AsBytes;

pub struct BinaryObjectFormat;

impl BinaryObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl FUELObjectFormatTrait for BinaryObjectFormat {
    fn pack(
        self: &Self,
        input_path: &Path,
        header: &mut Vec<u8>,
        body: &mut Vec<u8>,
    ) -> Result<(Vec<u32>, Vec<u32>), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        let bin_path = input_path.join("data.bin");
        let mut bin_file = File::open(&bin_path)?;

        #[derive(Deserialize)]
        struct Object {
            resource_object: ResourceObjectZ,
        }

        let object: Object = serde_json::from_reader(json_file)?;

        object.resource_object.write(header)?;

        let metadata = fs::metadata(&bin_path)?;
        body.resize(metadata.len() as usize, 0);
        bin_file.read(body.as_bytes_mut())?;

        Ok((
            object.resource_object.hard_links(),
            object.resource_object.soft_links(),
        ))
    }

    fn unpack(
        self: &Self,
        header: &[u8],
        body: &[u8],
        output_path: &Path,
    ) -> Result<(Vec<u32>, Vec<u32>), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let bin_path = output_path.join("data.bin");
        let mut output_bin_file = File::create(bin_path)?;

        let resource_object = match ResourceObjectZ::parse(&header) {
            Ok((_, h)) => h,
            Err(_) => return Err(Error::from(ErrorKind::Other)),
        };

        #[derive(Serialize)]
        struct Object {
            resource_object: ResourceObjectZ,
        }

        let object = Object { resource_object };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        output_bin_file.write(&body)?;

        Ok((
            object.resource_object.hard_links(),
            object.resource_object.soft_links(),
        ))
    }
}
