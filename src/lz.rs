use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use itertools::Itertools;
use lz4::{Decoder, EncoderBuilder};
use std::cmp::{max, min};
use std::convert::TryInto;
use std::ptr::null_mut;

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

// #[derive(Clone)]
// struct PacketMatch {
//     length: i32,
//     data: i32,
// }
//
// #[derive(Clone)]
// struct Packet {
//     match_length: i32,
//     total_length: i32,
//     matches: Vec<PacketMatch>,
// }
//
// #[derive(Clone)]
// struct Match {
//     pos: i64,
//     prev: *mut Match,
//     next: *mut Match,
// }
//
// impl Match {
//     unsafe fn orphan(&mut self) {
//         self.prev.as_mut().unwrap().next = null_mut();
//         self.prev = null_mut();
//     }
//
//     unsafe fn push_back(&mut self, node: &mut Match) {
//         node.prev = self.prev;
//         if !node.prev.is_null() { node.prev.as_mut().unwrap().next = node; }
//         node.next = self;
//         node.prev = node;
//     }
// }
//
// unsafe fn encode_packet(
//     uncompressed_buffer: &[u8],
//     packet: &mut Packet,
//     mut window_index: u32,
//     uncompressed_buffer_cursor: &mut Cursor<&[u8]>,
//     uncompressed_buffer_size: usize,
//     g_window_buffer: &Vec<Match>,
// ) -> bool {
//     let mut remaining_length: u32 = (1 << packet.match_length) + 2;
//     let v20: u32 = 0x10000 >> packet.match_length;
//
//     packet.matches.clear();
//     for _ in 0..30 {
//         let v5: u8 = max(
//             0,
//             (uncompressed_buffer_cursor.position() - v20 as u64) as u8,
//         );
//
//         remaining_length = min(
//             remaining_length,
//             (uncompressed_buffer_size - uncompressed_buffer_cursor.position() as usize) as u32,
//         );
//
//         if remaining_length <= 2 {
//             packet.total_length += 1;
//             packet.matches.push(PacketMatch {
//                 length: -1,
//                 data: uncompressed_buffer_cursor.read_u8().unwrap() as i32,
//             });
//             window_index += 1;
//         } else {
//             let mut pos: u32 = 0;
//
//             let mut match_length: i32 = 2;
//
//             let mut prev: *mut Match = g_window_buffer[window_index as usize].prev;
//             while !prev.is_null() && prev.as_ref().unwrap().pos >= v5 as i64 {
//                 if uncompressed_buffer[uncompressed_buffer_cursor.position()  as usize + 2] == uncompressed_buffer[prev.as_ref().unwrap().pos as usize + 2] {
//                     let mut j: i32 = 3;
//                     while uncompressed_buffer[prev.as_ref().unwrap().pos  as usize + j  as usize] == uncompressed_buffer[uncompressed_buffer_cursor.position()  as usize + j  as usize] && remaining_length as usize != j as usize {
//                         j += 1;
//                     }
//
//                     if match_length < j {
//                         if remaining_length as usize == j as usize {
//                             pos = prev.as_ref().unwrap().pos as u32;
//                             match_length = remaining_length as i32;
//                             break;
//                         }
//                         match_length = j;
//                         pos = prev.as_ref().unwrap().pos as u32;
//                     }
//                 }
//
//                 prev = prev.as_ref().unwrap().prev;
//             }
//
//             if match_length == 2 {
//                 packet.total_length += 1;
//                 packet.matches.push(PacketMatch {
//                     length: -1,
//                     data: uncompressed_buffer_cursor.read_u8().unwrap() as i32,
//                 });
//                 window_index += 1;
//             } else {
//                 packet.total_length += match_length;
//                 packet.matches.push(PacketMatch {
//                     length: match_length - 3,
//                     data: (uncompressed_buffer_cursor.position() as usize - pos as usize) as i32,
//                 });
//                 uncompressed_buffer_cursor.seek(SeekFrom::Current(match_length as i64)).unwrap();
//                 window_index += match_length as u32;
//             }
//         }
//
//         window_index = window_index % 0x8000;
//
//         if uncompressed_buffer_cursor.position() >= uncompressed_buffer_size as u64 {
//             return false;
//         }
//     }
//
//     return true;
// }
//
// unsafe fn lzss_compress(
//     uncompressed_buffer: &[u8],
//     uncompressed_buffer_size: usize,
//     compressed_buffer: &mut [u8],
//     _compressed_buffer_size: usize,
// ) -> Result<usize, io::Error> {
//     let mut g_window_buffer: Vec<Match> = std::iter::repeat(Match {
//         pos: -1,
//         prev: null_mut(),
//         next: null_mut()
//     }).take(0x8000).collect::<Vec<_>>();
//
//     let mut g_matches: Vec<Match> = std::iter::repeat(Match {
//         pos: -1,
//         prev: null_mut(),
//         next: null_mut()
//     }).take(0x10000).collect::<Vec<_>>();
//
//     let mut compressed_buffer_cursor = Cursor::new(compressed_buffer);
//     compressed_buffer_cursor.write_u32::<LittleEndian>(uncompressed_buffer_size as u32).unwrap();
//     compressed_buffer_cursor.seek(SeekFrom::Current(4)).unwrap();
//
//     let window_size: u32 = min(uncompressed_buffer_size as u32, 0x8000u32);
//
//     let mut packets: Vec<Packet> = std::iter::repeat(Packet {
//         match_length: 0,
//         total_length: 0,
//         matches: vec![]
//     }).take(4).collect::<Vec<_>>();
//     packets[0].match_length = 2;
//     packets[1].match_length = 3;
//     packets[2].match_length = 4;
//     packets[3].match_length = 5;
//
//     let mut window_index: u32 = 0;
//
//     for i in 0..window_size {
//         let pos: i64 = i as i64;
//         let match_index: u16 = u16::from_be_bytes(uncompressed_buffer[pos as usize..pos as usize + 2].try_into().unwrap());
//         let mut current: &mut Match = &mut g_window_buffer[i as usize];
//         let next: &mut Match = &mut g_matches[match_index as usize];
//         current.pos = pos;
//         next.push_back(current);
//     }
//
//     let mut uncompressed_buffer_cursor = Cursor::new(uncompressed_buffer);
//
//     let mut buffer_size_2 = 0x8000u32;
//     let mut k = 0x7000i32;
//
//     while uncompressed_buffer_cursor.position() < uncompressed_buffer_size as u64 {
//         let mut len: u8;
//
//         packets[3].total_length = 0;
//         packets[2].total_length = 0;
//         packets[1].total_length = 0;
//         packets[0].total_length = 0;
//
//         if encode_packet(
//             &uncompressed_buffer[uncompressed_buffer_cursor.position() as usize..],
//             &mut packets[3],
//             window_index,
//             &mut uncompressed_buffer_cursor,
//             uncompressed_buffer_size,
//             &mut g_window_buffer,
//         ) && packets[3].total_length <= 540
//         {
//             if encode_packet(
//                 &uncompressed_buffer[uncompressed_buffer_cursor.position() as usize..],
//                 &mut packets[2],
//                 window_index,
//                 &mut uncompressed_buffer_cursor,
//                 uncompressed_buffer_size,
//                 &mut g_window_buffer,
//             ) {
//                 len = if packets[2].total_length <= packets[3].total_length { 1 } else { 0 } + 2;
//                 if packets[len as usize].total_length <= 300 {
//                     if encode_packet(
//                         &uncompressed_buffer[uncompressed_buffer_cursor.position() as usize..],
//                         &mut packets[1],
//                         window_index,
//                         &mut uncompressed_buffer_cursor,
//                         uncompressed_buffer_size,
//                         &mut g_window_buffer,
//                     ) {
//                         if packets[1].total_length > packets[len as usize].total_length {
//                             len = 1;
//                         }
//
//                         if packets[len as usize].total_length <= 180 {
//                             encode_packet(
//                                 &uncompressed_buffer[uncompressed_buffer_cursor.position() as usize..],
//                                 &mut packets[0],
//                                 window_index,
//                                 &mut uncompressed_buffer_cursor,
//                                 uncompressed_buffer_size,
//                                 &mut g_window_buffer,
//                             );
//                             if packets[0].total_length >= packets[len as usize].total_length {
//                                 len = 0;
//                             }
//                         }
//                     } else {
//                         len = 1;
//                     }
//                 }
//             } else {
//                 len = 2;
//             }
//         } else {
//             len = 3;
//         }
//
//         let current_packet: &Packet = &packets[len as usize];
//
//         let mut flag: u32 = 0;
//         for i in 0..current_packet.matches.len() {
//             if current_packet.matches[i].length >= 0 {
//                 flag |= 0x80000000u32 >> i;
//             }
//         }
//
//         compressed_buffer_cursor.write_u32::<BigEndian>(flag | len as u32).unwrap();
//
//         for m in current_packet.matches.iter() {
//             if m.length == -1 {
//                 compressed_buffer_cursor.write_u8(m.data as u8).unwrap();
//             } else {
//                 compressed_buffer_cursor
//                     .write_u16::<BigEndian>((m.data + (m.length << (0xE - len)) - 1) as u16).unwrap();
//             }
//         }
//
//         uncompressed_buffer_cursor.seek(SeekFrom::Current(current_packet.total_length as i64)).unwrap();
//
//         window_index = (window_index + current_packet.total_length as u32) % 0x8000u32;
//
//         k -= current_packet.total_length;
//         if k < 0 {
//             let window_size_1: u32 = min(uncompressed_buffer_size as u32, buffer_size_2 + 0x1000u32);
//             for i in buffer_size_2..window_size_1 {
//                 let pos: i64 = i as i64;
//                 let match_index: u16 =
//                     u16::from_be_bytes(uncompressed_buffer[pos as usize..pos as usize + 2].try_into().unwrap());
//                 let mut current: &mut Match = &mut g_window_buffer.split_at_mut(i as usize % 0x8000usize).1[0];
//                 let next: &mut Match = &mut g_matches.split_at_mut(match_index as usize).1[0];
//                 current.next.as_mut().unwrap().orphan();
//                 current.pos = pos;
//                 next.push_back(current);
//             }
//             k += 0x1000i32;
//             buffer_size_2 = window_size_1;
//         }
//     }
//
//     let compressed_size = compressed_buffer_cursor.position() as u32;
//     compressed_buffer_cursor.seek(SeekFrom::Start(4)).unwrap();
//     compressed_buffer_cursor.write_u32::<LittleEndian>(compressed_size).unwrap();
//
//     return Ok(compressed_size as usize);
// }

#[derive(Clone)]
struct PacketMatch {
    length: i32,
    data: i32,
}

#[derive(Clone)]
struct Packet {
    match_length: i32,
    total_length: i32,
    matches: Vec<PacketMatch>,
}

#[derive(Clone)]
struct Match {
    pos: u64,
    prev: *mut Match,
    next: *mut Match,
}

impl Match {
    unsafe fn orphan(&mut self) {
        self.prev.as_mut().unwrap().next = null_mut();
        self.prev = null_mut();
    }

    unsafe fn push_back(&mut self, node: *mut Match) {
        node.as_mut().unwrap().prev = self.prev;
        if !node.as_mut().unwrap().prev.is_null() {
            node.as_mut().unwrap().prev.as_mut().unwrap().next = node;
        };
        node.as_mut().unwrap().next = self;
        node.as_mut().unwrap().next.as_mut().unwrap().prev = node;
    }
}

unsafe fn encode_packet(
    mut uncompressed_buffer_ptr: u64,
    packet: &mut Packet,
    mut window_index: u32,
    uncompressed_buffer: &[u8],
    uncompressed_buffer_size: usize,
    g_window_buffer: &Vec<Match>,
) -> bool {
    let mut remaining_length: u32 = (1 << packet.match_length) + 2;
    let v20: u32 = 0x10000 >> packet.match_length;

    packet.matches.clear();
    for _ in 0..30 {
        let v5: u64 = max(0 as i64, uncompressed_buffer_ptr as i64 - v20 as i64) as u64;

        remaining_length = min(
            remaining_length,
            (uncompressed_buffer_size - uncompressed_buffer_ptr as usize) as u32,
        );

        if remaining_length <= 2 {
            packet.total_length += 1;
            packet.matches.push(PacketMatch {
                length: -1,
                data: uncompressed_buffer[uncompressed_buffer_ptr as usize] as i32,
            });
            uncompressed_buffer_ptr += 1;
            window_index += 1;
        } else {
            let mut ptr: u64 = 0;

            let mut match_length: i32 = 2;
            let mut cur: *const Match = g_window_buffer[window_index as usize].prev;
            while !cur.is_null() && cur.as_ref().unwrap().pos >= v5 {
                if uncompressed_buffer[uncompressed_buffer_ptr as usize + 2]
                    == uncompressed_buffer[cur.as_ref().unwrap().pos as usize + 2]
                {
                    let mut j: i32 = 3;
                    while uncompressed_buffer[cur.as_ref().unwrap().pos as usize + j as usize]
                        == uncompressed_buffer[uncompressed_buffer_ptr as usize + j as usize]
                        && remaining_length != j as u32
                    {
                        j += 1;
                    }

                    if match_length < j {
                        if remaining_length == j as u32 {
                            ptr = cur.as_ref().unwrap().pos;
                            match_length = remaining_length as i32;
                            break;
                        }
                        match_length = j;
                        ptr = cur.as_ref().unwrap().pos;
                    }
                }
                cur = cur.as_ref().unwrap().prev;
            }

            if match_length == 2 {
                packet.total_length += 1;
                packet.matches.push(PacketMatch {
                    length: -1,
                    data: uncompressed_buffer[uncompressed_buffer_ptr as usize] as i32,
                });
                uncompressed_buffer_ptr += 1;
                window_index += 1;
            } else {
                packet.total_length += match_length;
                packet.matches.push(PacketMatch {
                    length: match_length - 3,
                    data: uncompressed_buffer_ptr as i32 - ptr as i32 ,
                });
                uncompressed_buffer_ptr += match_length as u64;
                window_index += match_length as u32;
            }
        }

        window_index = window_index % 0x8000;

        if uncompressed_buffer_ptr >= uncompressed_buffer_size as u64 {
            return false;
        }
    }

    return true;
}

pub(crate) unsafe fn lzss_compress(
    uncompressed_buffer: &[u8],
    uncompressed_buffer_size: usize,
    compressed_buffer: &mut [u8],
    _compressed_buffer_size: usize,
) -> Result<usize, io::Error> {
    assert_eq!(uncompressed_buffer.len(), uncompressed_buffer_size + 2);

    let mut g_window_buffer: Vec<Match> = std::iter::repeat(Match {
        pos: 0,
        prev: null_mut(),
        next: null_mut(),
    })
    .take(0x8000)
    .collect::<Vec<_>>();

    let mut short_lookup: Vec<Match> = std::iter::repeat(Match {
        pos: 0,
        prev: null_mut(),
        next: null_mut(),
    })
    .take(0x10000)
    .collect::<Vec<_>>();

    let mut compressed_buffer_cursor = Cursor::new(compressed_buffer);

    let window_size: u32 = min(uncompressed_buffer_size as u32, 0x8000 as u32);

    let mut packets: Vec<Packet> = std::iter::repeat(Packet {
        match_length: 0,
        total_length: 0,
        matches: vec![],
    })
    .take(4)
    .collect::<Vec<_>>();
    packets[0].match_length = 2;
    packets[1].match_length = 3;
    packets[2].match_length = 4;
    packets[3].match_length = 5;

    let mut window_index: u32 = 0;

    for i in 0..window_size {
        let ptr: u32 = i;
        let match_index: u16 = u16::from_be_bytes(
            uncompressed_buffer[ptr as usize..ptr as usize + 2]
                .try_into()
                .unwrap(),
        );
        let current: *mut Match = &mut g_window_buffer[i as usize];
        let next: *mut Match = &mut short_lookup[match_index as usize];
        current.as_mut().unwrap().pos = ptr as u64;
        next.as_mut().unwrap().push_back(current);
    }

    let mut uncompressed_buffer_cursor = Cursor::new(uncompressed_buffer);

    let mut buffer_size_2: u32 = 0x8000 as u32;
    let mut k: i32 = 0x7000;

    while uncompressed_buffer_cursor.position() < uncompressed_buffer_size as u64 {
        let mut len: u8;

        packets[3].total_length = 0;
        packets[2].total_length = 0;
        packets[1].total_length = 0;
        packets[0].total_length = 0;

        if encode_packet(
            uncompressed_buffer_cursor.position(),
            &mut packets[3],
            window_index,
            uncompressed_buffer,
            uncompressed_buffer_size,
            &g_window_buffer,
        ) && packets[3].total_length <= 540
        {
            if encode_packet(
                uncompressed_buffer_cursor.position(),
                &mut packets[2],
                window_index,
                uncompressed_buffer,
                uncompressed_buffer_size,
                &g_window_buffer,
            ) {
                len = if packets[2].total_length <= packets[3].total_length {
                    1
                } else {
                    0
                } + 2;
                if packets[len as usize].total_length <= 300 {
                    if encode_packet(
                        uncompressed_buffer_cursor.position(),
                        &mut packets[1],
                        window_index,
                        uncompressed_buffer,
                        uncompressed_buffer_size,
                        &g_window_buffer,
                    ) {
                        if packets[1].total_length > packets[len as usize].total_length {
                            len = 1;
                        }

                        if packets[len as usize].total_length <= 180 {
                            encode_packet(
                                uncompressed_buffer_cursor.position(),
                                &mut packets[0],
                                window_index,
                                uncompressed_buffer,
                                uncompressed_buffer_size,
                                &g_window_buffer,
                            );
                            if packets[0].total_length >= packets[len as usize].total_length {
                                len = 0;
                            }
                        }
                    } else {
                        len = 1;
                    }
                }
            } else {
                len = 2;
            }
        } else {
            len = 3;
        }

        let current_packet: &Packet = &packets[len as usize];

        let mut flag: u32 = 0;
        for i in 0..current_packet.matches.len() {
            if current_packet.matches[i].length >= 0 {
                flag |= 0x80000000u32 >> i;
            }
        }

        compressed_buffer_cursor.write_u32::<BigEndian>(flag | len as u32)?;

        for m in current_packet.matches.iter() {
            if m.length == -1 {
                compressed_buffer_cursor.write_u8(m.data as u8)?;
            } else {
                compressed_buffer_cursor
                    .write_u16::<BigEndian>((m.data + (m.length << (0xE - len)) - 1) as u16)?;
            }
        }

        uncompressed_buffer_cursor.seek(SeekFrom::Current(current_packet.total_length as i64))?;

        window_index = (window_index + current_packet.total_length as u32) % 0x8000 as u32;

        k -= current_packet.total_length;
        if k < 0 {
            let window_size_1: u32 =
                min(uncompressed_buffer_size as u32, buffer_size_2 + 0x1000u32);
            for i in buffer_size_2..window_size_1 {
                let ptr: u32 = i;
                let match_index: u16 = u16::from_be_bytes(
                    uncompressed_buffer[ptr as usize..ptr as usize + 2]
                        .try_into()
                        .unwrap(),
                );
                let current: *mut Match = &mut g_window_buffer[i as usize % 0x8000 as usize];
                let next: *mut Match = &mut short_lookup[match_index as usize];
                current.as_mut().unwrap().next.as_mut().unwrap().orphan();
                current.as_mut().unwrap().pos = ptr as u64;
                next.as_mut().unwrap().push_back(current);
            }
            k += 0x1000i32;
            buffer_size_2 = window_size_1;
        }
    }

    return Ok(compressed_buffer_cursor.position() as usize);
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

pub trait LZ {
    fn decompress_internal(
        self: &Self,
        compressed_buffer: &Vec<u8>,
        decompressed_buffer: &mut Vec<u8>,
    ) -> Result<(), io::Error>;
    fn compress_internal(
        self: &Self,
        decompressed_buffer: &mut Vec<u8>,
        compressed_buffer: &mut Vec<u8>,
    ) -> Result<(), io::Error>;
    fn decompress(
        self: &Self,
        compressed_path: &Path,
        decompressed_path: &Path,
    ) -> Result<(), io::Error> {
        let mut compressed_file = File::open(compressed_path)?;
        let mut decompressed_file = File::create(decompressed_path)?;

        let decompressed_len = compressed_file.read_u32::<LittleEndian>()? as usize;
        let compressed_len = compressed_file.read_u32::<LittleEndian>()? as usize - 8;

        let mut decompressed_buffer = vec![0; decompressed_len];
        let mut compressed_buffer = vec![0; compressed_len];

        compressed_file.read(&mut compressed_buffer)?;

        self.decompress_internal(&mut compressed_buffer, &mut decompressed_buffer)?;

        decompressed_file.write(&decompressed_buffer)?;

        Ok(())
    }

    fn compress(
        self: &Self,
        decompressed_path: &Path,
        compressed_path: &Path,
    ) -> Result<(), io::Error> {
        let mut decompressed_file = File::open(decompressed_path)?;
        let mut compressed_file = File::create(compressed_path)?;

        let mut decompressed_buffer = vec![];

        decompressed_file.read_to_end(&mut decompressed_buffer)?;

        let mut compressed_buffer = vec![0; decompressed_buffer.len() * 2];

        self.compress_internal(&mut decompressed_buffer, &mut compressed_buffer)?;

        compressed_file.write_u32::<LittleEndian>(decompressed_buffer.len() as u32)?;
        compressed_file.write_u32::<LittleEndian>(compressed_buffer.len() as u32 + 8)?;

        compressed_file.write(&compressed_buffer)?;

        Ok(())
    }
}

pub struct LZLZSS {}

impl LZ for LZLZSS {
    fn decompress_internal(
        self: &Self,
        compressed_buffer: &Vec<u8>,
        decompressed_buffer: &mut Vec<u8>,
    ) -> Result<(), io::Error> {
        let decompressed_buffer_len = decompressed_buffer.len();
        match lzss_decompress(
            &compressed_buffer[..],
            compressed_buffer.len(),
            &mut decompressed_buffer[..],
            decompressed_buffer_len,
            false,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn compress_internal(
        self: &Self,
        decompressed_buffer: &mut Vec<u8>,
        compressed_buffer: &mut Vec<u8>,
    ) -> Result<(), io::Error> {
        unsafe {
            let compressed_buffer_len = compressed_buffer.len();
            let decompressed_buffer_len = decompressed_buffer.len();
            decompressed_buffer.resize(decompressed_buffer_len + 2, 0);
            match lzss_compress(
                &decompressed_buffer[..],
                decompressed_buffer_len,
                &mut compressed_buffer[..],
                compressed_buffer_len,
            ) {
                Ok(len) => {
                    decompressed_buffer.resize(decompressed_buffer_len, 0);
                    compressed_buffer.resize(len, 0);
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
    }
}

pub struct LZLZ4 {}

impl LZ for LZLZ4 {
    fn decompress_internal(
        self: &Self,
        compressed_buffer: &Vec<u8>,
        decompressed_buffer: &mut Vec<u8>,
    ) -> Result<(), io::Error> {
        let compressed_buffer_cursor = Cursor::new(compressed_buffer);
        let mut decompressed_buffer_cursor = Cursor::new(decompressed_buffer);

        let mut decoder = Decoder::new(compressed_buffer_cursor)?;
        io::copy(&mut decoder, &mut decompressed_buffer_cursor)?;

        Ok(())
    }

    fn compress_internal(
        self: &Self,
        decompressed_buffer: &mut Vec<u8>,
        compressed_buffer: &mut Vec<u8>,
    ) -> Result<(), io::Error> {
        let mut decompressed_buffer_cursor = Cursor::new(decompressed_buffer);
        let compressed_buffer_cursor = Cursor::new(compressed_buffer);

        let mut encoder = EncoderBuilder::new()
            .level(4)
            .build(compressed_buffer_cursor)?;
        io::copy(&mut decompressed_buffer_cursor, &mut encoder)?;
        let (_output, result) = encoder.finish();
        result
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::PathBuf;

    use byteorder::{LittleEndian, ReadBytesExt};
    use checksums::hash_file;
    use checksums::Algorithm;
    use test_generator::test_resources;

    use crate::lz;

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

pub struct LZSubCommand<'a> {
    algorithms: HashMap<&'a str, &'a dyn LZ>,
}

impl LZSubCommand<'_> {
    pub fn new<'a>() -> LZSubCommand<'a> {
        let mut algorithms: HashMap<&str, &dyn LZ> = HashMap::new();

        algorithms.insert("lzss", &LZLZSS {});
        algorithms.insert("lz4", &LZLZ4 {});

        LZSubCommand { algorithms }
    }

    pub fn subcommand(self: &Self) -> App {
        SubCommand::with_name("lz")
            .about("Used to compress raw files")
            .arg(
                Arg::with_name("ALGORITHM")
                    .short("a")
                    .long("algorithm")
                    .takes_value(true)
                    .required(true)
                    .requires("INPUT")
                    .possible_values(
                        self.algorithms
                            .keys()
                            .map(|x| x.clone())
                            .collect_vec()
                            .as_slice(),
                    )
                    .help("The algorithm the raw file should be compatible with"),
            )
            .arg(
                Arg::with_name("COMPRESS")
                    .short("c")
                    .long("compress")
                    .requires("INPUT")
                    .conflicts_with("DECOMPRESS")
                    .help("compress the file"),
            )
            .arg(
                Arg::with_name("DECOMPRESS")
                    .short("d")
                    .long("decompress")
                    .requires("INPUT")
                    .conflicts_with("COMPRESS")
                    .help("decompress the file"),
            )
            .after_help("EXAMPLES:\n    lz -ac lzss -i raw.dat\n    lz -ad lz4 -i raw.dat")
            .settings(&[AppSettings::ArgRequiredElseHelp])
    }

    pub fn execute(
        self: &Self,
        matches: &ArgMatches,
        subcommand_matches: &ArgMatches,
    ) -> Result<(), io::Error> {
        let input_path_string = matches.value_of_os("INPUT").unwrap();
        let input_path = Path::new(input_path_string);

        let output_path = match subcommand_matches.value_of_os("OUTPUT") {
            Some(output_path_string) => PathBuf::from(output_path_string),
            None => input_path.with_extension(if subcommand_matches.is_present("COMPRESS") {
                "comp"
            } else {
                "uncomp"
            }),
        };

        match subcommand_matches.value_of("ALGORITHM") {
            None => panic!("Algorithm is required"),
            Some(algorithm) => {
                if let Some(lz_implementation) = self.algorithms.get(algorithm) {
                    if subcommand_matches.is_present("COMPRESS") {
                        lz_implementation.compress(&input_path, &output_path.as_path())?;
                    } else {
                        lz_implementation.decompress(&input_path, &output_path.as_path())?;
                    }
                } else {
                    panic!("bad algorithm")
                }
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod test_lz {
    use crate::lz::{LZ, LZLZSS};
    use checksums::{hash_file, Algorithm};
    use std::path::Path;
    use test_generator::test_resources;

    #[test_resources("D:/programming/widberg/dpc/data/8014325.Bitmap_Z.out")]
    fn test_fuel_lzrs(path: &str) {
        let uncompressed_path = Path::new(path);
        let compressed_path = uncompressed_path.with_extension("out.uncomp");

        let lzss = LZLZSS {};
        lzss.compress(uncompressed_path, compressed_path.as_path())
            .unwrap();

        assert_eq!(
            hash_file(
                uncompressed_path.with_extension("in").as_path(),
                Algorithm::MD5
            ),
            hash_file(compressed_path.as_path(), Algorithm::MD5)
        );
    }
}
