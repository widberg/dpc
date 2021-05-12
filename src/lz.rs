use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::prelude::*;

pub fn lz_fuel_decompress(compressed_buffer: &Vec<u8>, decompressed_buffer: &mut Vec<u8>, is_in_place: bool) {
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

				let mut window_buffer = vec![];
				window_buffer.clone_from_slice(&decompressed_buffer_cursor.get_ref()[decompressed_buffer_size - ((temp & temp_mask) as usize + 1)..]);
				let mut window_buffer_cursor = Cursor::new(window_buffer);
				
				for _ in 0..((temp >> temp_shift) + 3) {
					let mut t = [0];
					window_buffer_cursor.read(&mut t).unwrap();
					decompressed_buffer_cursor.write(&t).unwrap();
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
