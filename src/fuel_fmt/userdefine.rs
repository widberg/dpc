use binwrite::BinWrite;
use nom_derive::{NomLE, Parse};
use serde::{Deserialize, Serialize};

use crate::fuel_fmt::common::{
    FUELObjectFormatTrait, HasReferences, PascalString, ResourceObjectZ,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs;
use std::fs::File;
use std::io::{Cursor, Error, ErrorKind, Read, Write};
use std::path::Path;
use zerocopy::AsBytes;

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct UserDefineZ {
    data: PascalString,
}

pub struct UserDefineObjectFormat;

impl UserDefineObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl FUELObjectFormatTrait for UserDefineObjectFormat {
    fn pack(
        self: &Self,
        input_path: &Path,
        header: &mut Vec<u8>,
        body: &mut Vec<u8>,
    ) -> Result<(Vec<u32>, Vec<u32>), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        let txt_path = input_path.join("data.txt");

        #[derive(Deserialize)]
        struct Object {
            resource_object: ResourceObjectZ,
        }

        let object: Object = serde_json::from_reader(json_file)?;

        object.resource_object.write(header)?;

        let metadata = fs::metadata(&txt_path)?;
        let mut body_cursor = Cursor::new(body);
        body_cursor.write_u32::<LittleEndian>(metadata.len() as u32)?;
        body_cursor.write(fs::read(txt_path).unwrap().as_bytes())?;

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

        let txt_path = output_path.join("data.txt");
        let mut output_txt_file = File::create(txt_path)?;

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

        let mut body_cursor = Cursor::new(&body);
        let text_size = body_cursor.read_u32::<LittleEndian>()?;
        let mut text_vec = Vec::new();
        text_vec.resize(text_size as usize, 0);
        body_cursor.read(&mut text_vec[..])?;
        output_txt_file.write(&text_vec)?;

        Ok((
            object.resource_object.hard_links(),
            object.resource_object.soft_links(),
        ))
    }
}
