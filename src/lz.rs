use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;

pub fn lzss_decompress(
    compressed_buffer: &[u8],
    _compressed_buffer_size: usize,
    decompressed_buffer: &mut [u8],
    decompressed_buffer_size: usize,
    is_in_place: bool,
) -> Result<usize, io::Error> {
    // Magic Numbers
    const WINDOW_LOG: u32 = 14;
    const WINDOW_MASK: u32 = (1 << WINDOW_LOG) - 1;

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
                let start: usize = decompressed_buffer_cursor.position() as usize
                    - ((temp & temp_mask) as usize + 1);
                let end: usize = start + (temp >> temp_shift) as usize + 3;

                for i in start..end {
                    let byte: u8 = decompressed_buffer_cursor.get_ref()[i];
                    decompressed_buffer_cursor.write_u8(byte)?;
                }
            } else {
                let byte = compressed_buffer_cursor.read_u8().unwrap();
                decompressed_buffer_cursor.write_u8(byte)?;
            }

            if (decompressed_buffer_cursor.position() as usize >= decompressed_buffer_size)
                || (is_in_place
                    && (decompressed_buffer_cursor.position()
                        > compressed_buffer_cursor.position()))
            {
                return Ok(decompressed_buffer_cursor.position() as usize);
            }

            flags <<= 1
        }
    }
}

pub fn lzss_compress_optimized(
    decompressed_buffer: &[u8],
    decompressed_buffer_size: usize,
    compressed_buffer: &mut [u8],
    _compressed_buffer_size: usize,
) -> Result<usize, io::Error> {
    const WINDOW_LOG: u32 = 14;
    const WINDOW_MASK: u32 = (1 << WINDOW_LOG) - 1;
    const MATCH_NUM: u32 = 30;
    const MATCH_ITER: u32 = 4;
    const MIN_MATCH_LEN: u32 = 3;
    const MIN_DISTANCE: u32 = 1;

    let mut distances_table = [[0u32; MATCH_ITER as usize]; MATCH_NUM as usize];
    let mut lengths_table = [[0u32; MATCH_ITER as usize]; MATCH_NUM as usize];

    let mut decompressed_buffer_cursor = Cursor::new(decompressed_buffer);
    let mut compressed_buffer_cursor = Cursor::new(compressed_buffer);

    // let mut next: u64 = 0;

    while (decompressed_buffer_cursor.position() as usize) < decompressed_buffer_size {
        // if decompressed_buffer_cursor.position() >= next {
        // 	println!("inp={}/{} out={}\r", decompressed_buffer_cursor.position() as u32, decompressed_buffer_size as u32, compressed_buffer_cursor.position() as u32);
        // 	next = decompressed_buffer_cursor.position() + 0x10000;
        // 	if next > decompressed_buffer_size as u64 {
        // 		next = decompressed_buffer_size as u64;
        // 	}
        // }

        let decompressed_buffer_cursor_position_backup = decompressed_buffer_cursor.position();
        let flag_position: usize = compressed_buffer_cursor.position() as usize;
        compressed_buffer_cursor.seek(SeekFrom::Current(4))?;
        let mut opt_flag: u32 = 0;
        let mut opt_rate: f64 = 0.0;

        for t in 0..MATCH_ITER {
            let mut flag: u32 = 0;
            let mut ulen: u32 = 0;
            let mut clen: u32 = 0;
            decompressed_buffer_cursor
                .seek(SeekFrom::Start(decompressed_buffer_cursor_position_backup))?;
            let temp_wlog: u32 = WINDOW_LOG - t;
            let temp_mlen: u32 = (1 << (16 - temp_wlog)) - 1 + MIN_MATCH_LEN;
            let temp_mask: u32 = WINDOW_MASK >> t;

            for i in 0..MATCH_NUM {
                distances_table[i as usize][t as usize] = 0;
                lengths_table[i as usize][t as usize] = 1;
            }

            for i in 0..MATCH_NUM {
                if decompressed_buffer_cursor.position() as usize >= decompressed_buffer_size {
                    break;
                }

                let pos: u32 = decompressed_buffer_cursor.position() as u32;
                let mut k: u32 = (pos as i32 - (temp_mask + MIN_DISTANCE) as i32) as u32;
                if (k & 0x80000000) != 0 {
                    k = 0;
                }
                let mut l: u32 = (decompressed_buffer_size as u32
                    - decompressed_buffer_cursor.position() as u32)
                    as u32;
                if l > temp_mlen {
                    l = temp_mlen;
                }
                let mut ml: u32 = 0; // max match len
                let mut mj: u32 = 0; // max match pos
                let start: u32 = (pos as i32 - 1) as u32;
                let end: u32 = (k as i32 - 1) as u32;
                let mut j: u32 = start;
                while j != end {
                    let mut rr: u32 = l;
                    for r in 0..l {
                        if decompressed_buffer
                            [(decompressed_buffer_cursor.position() as u32 + r) as usize]
                            != decompressed_buffer[(j + r) as usize]
                        {
                            rr = r;
                            break;
                        }
                    }
                    if rr > ml {
                        ml = rr;
                        mj = pos - j;
                    }

                    j = (j as i32 - 1) as u32;
                }

                if ml < MIN_MATCH_LEN {
                    // literal
                    ulen += 1;
                    decompressed_buffer_cursor.seek(SeekFrom::Current(1))?;
                    clen += 1;
                } else {
                    // match
                    distances_table[i as usize][t as usize] = mj;
                    lengths_table[i as usize][t as usize] = ml;
                    flag |= 1 << (31 - i);
                    ulen += ml;
                    decompressed_buffer_cursor.seek(SeekFrom::Current(ml as i64))?;
                    clen += 2;
                }
            } // for

            let new_rate: f64 = ulen as f64 / (4 + clen) as f64;

            if new_rate > opt_rate {
                opt_rate = new_rate;
                opt_flag = flag | t;
            }
        }

        (&mut compressed_buffer_cursor.get_mut()[flag_position..flag_position + 4])
            .write_u32::<BigEndian>(opt_flag)?;

        let t: u32 = opt_flag & 3;
        decompressed_buffer_cursor
            .seek(SeekFrom::Start(decompressed_buffer_cursor_position_backup))?;
        let temp_wlog: u32 = WINDOW_LOG - t;
        let temp_mask: u32 = WINDOW_MASK >> t;

        for i in 0..MATCH_NUM {
            if decompressed_buffer_cursor.position() as usize >= decompressed_buffer_size {
                break;
            }

            if (opt_flag & (1 << (31 - i))) != 0 {
                // match
                let ml: u32 = lengths_table[i as usize][t as usize];
                let mj: u32 = distances_table[i as usize][t as usize];
                let c: u16 = ((ml - MIN_MATCH_LEN) << temp_wlog) as u16
                    + ((mj - MIN_DISTANCE) & temp_mask) as u16;
                compressed_buffer_cursor.write_u16::<BigEndian>(c)?;
                decompressed_buffer_cursor.seek(SeekFrom::Current(ml as i64))?;
            } else {
                // literal
                let byte = decompressed_buffer_cursor.read_u8()?;
                compressed_buffer_cursor.write_u8(byte)?;
            }
        }
    }

    Ok(compressed_buffer_cursor.position() as usize)
}

#[cfg(test)]
mod test {
    use crate::lz;
    use byteorder::{LittleEndian, ReadBytesExt};
    use checksums::hash_file;
    use checksums::Algorithm;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::PathBuf;
    use test_generator::test_resources;

    #[test_resources("data/*.in")]
    fn test_lzss_optimized(path: &str) {
        let mut compressed_file = File::open(path).unwrap();

        let decompressed_buffer_len = compressed_file.read_u32::<LittleEndian>().unwrap();
        let compressed_buffer_len = compressed_file.read_u32::<LittleEndian>().unwrap() - 8;

        let mut compressed_buffer = vec![0; compressed_buffer_len as usize];
        compressed_file.read(&mut compressed_buffer).unwrap();

        let mut decompressed_buffer = vec![0; decompressed_buffer_len as usize];

        lz::lzss_decompress(
            &compressed_buffer[..],
            compressed_buffer_len as usize,
            &mut decompressed_buffer[..],
            decompressed_buffer_len as usize,
            false,
        )
        .unwrap();

        let mut out_path = PathBuf::from(path);
        out_path.set_extension("out");
        let mut decompressed_file = File::create(&out_path).unwrap();
        decompressed_file.write(&decompressed_buffer).unwrap();

        let mut good_path = PathBuf::from(path);
        good_path.set_extension("out.good");

        assert_eq!(
            hash_file(&good_path.as_path(), Algorithm::SHA1),
            hash_file(&out_path.as_path(), Algorithm::SHA1)
        );

        // compress then decompress it again

        let mut recompressed_buffer = vec![0; decompressed_buffer_len as usize * 2];

        let recompressed_size = lz::lzss_compress_optimized(
            &decompressed_buffer[..],
            decompressed_buffer_len as usize,
            &mut recompressed_buffer[..],
            decompressed_buffer_len as usize * 2,
        )
        .unwrap();
        lz::lzss_decompress(
            &recompressed_buffer[..],
            recompressed_size,
            &mut decompressed_buffer[..],
            decompressed_buffer_len as usize,
            false,
        )
        .unwrap();

        decompressed_file = File::create(&out_path).unwrap();
        decompressed_file.write(&decompressed_buffer).unwrap();

        assert_eq!(
            hash_file(&good_path.as_path(), Algorithm::SHA1),
            hash_file(&out_path.as_path(), Algorithm::SHA1)
        );
    }
}
