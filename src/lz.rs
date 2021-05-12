use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::prelude::*;
use std::vec::Vec;

pub fn lzss_decompress(compressed_buffer: &Vec<u8>, decompressed_buffer: &mut Vec<u8>, is_in_place: bool) {
    // Magic Numbers
    const WINDOW_LOG: u32 = 0xe;
    const WINDOW_MASK: u32 = 0x3fff;

	let decompressed_buffer_size = decompressed_buffer.len();
	let mut compressed_buffer_cursor = Cursor::new(compressed_buffer);
	let mut decompressed_buffer_cursor = Cursor::new(decompressed_buffer);

    loop {
        let mut flags: u32 = compressed_buffer_cursor.read_u32::<BigEndian>().unwrap(); // read as big endian
        let len: u32 = flags & 0x3; // 0b11
        let temp_shift: u32 = WINDOW_LOG - len;
        let temp_mask: u32 = WINDOW_MASK >> len;

        for _ in 0..30 {
            if (flags & 0x80000000) != 0 {
                let temp: u32 = compressed_buffer_cursor.read_u16::<BigEndian>().unwrap() as u32; // read as big endian

				let start: usize = decompressed_buffer_cursor.position() as usize - ((temp & temp_mask) as usize + 1);
				let length: usize = (temp >> temp_shift) as usize + 3;
				let end: usize = start + length;

				for i in start..end {
					let byte: u8 = decompressed_buffer_cursor.get_ref()[i];
					decompressed_buffer_cursor.write(&[byte]).unwrap();
				}
            } else {
				let mut t = [0];
				compressed_buffer_cursor.read(&mut t).unwrap();
				decompressed_buffer_cursor.write(&t).unwrap();
            }

            if (decompressed_buffer_cursor.position() as usize == decompressed_buffer_size) || (is_in_place && (decompressed_buffer_cursor.position() > compressed_buffer_cursor.position())) {
                return;
            }

			flags <<= 1
        }
    }
}

#[cfg(test)]
mod test {
	use crate::lz::lzss_decompress;
	use test_generator::test_resources;
	use std::fs::File;
	use std::io::Read;
	use std::io::Write;
	use byteorder::{LittleEndian, ReadBytesExt};
	use std::path::PathBuf;

    #[test_resources("data/*.in")]
    fn test_fuel_lz(path: &str) {
		let mut compressed_file = File::open(path).unwrap();

		let decompressed_buffer_len = compressed_file.read_u32::<LittleEndian>().unwrap();
		let compressed_buffer_len = compressed_file.read_u32::<LittleEndian>().unwrap();

		let mut compressed_buffer = vec![0; compressed_buffer_len as usize];
		compressed_file.read(&mut compressed_buffer).unwrap();

		let mut decompressed_buffer = vec![0; decompressed_buffer_len as usize];

		lzss_decompress(&compressed_buffer, &mut decompressed_buffer, false);

		let mut out_path = PathBuf::from(path);
		out_path.set_extension("out");
		let mut decompressed_file = File::create(out_path).unwrap();
		decompressed_file.write(&decompressed_buffer).unwrap();
	}
}
