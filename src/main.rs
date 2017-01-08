extern crate byteorder;

use std::io::Cursor;
use std::io::prelude::*;
use std::fs::File;
use std::char;

use byteorder::{ReadBytesExt, LittleEndian, BigEndian};

struct Rect {

}

enum Compression {
    Uncompressed,
    Zlib,
    Lzma,
}

impl Default for Compression {
    fn default() -> Compression {
        Compression::Uncompressed
    }
}

#[derive(Default)]
struct Header {
    compression: Compression,
    version: u8,
    file_length: u32,
//    frame_size: Rect,
//    frame_rate: u16,
//    frame_count: u16,
}

impl Header {
    fn read(&mut self, cursor: &mut Cursor<Vec<u8>>) {
        let signature = cursor.read_u32::<BigEndian>().unwrap();

        match char::from_u32(signature >> 24) {
            Some(val) => match val {
                'F' => self.compression = Compression::Uncompressed,
                'C' => self.compression = Compression::Zlib,
                'Z' => self.compression = Compression::Lzma,
                _ => println!("Unknown signature."),
            },
            None => println!("Unknown signature."),
        }

        self.version = (signature & 0xFF) as u8;

        self.file_length = cursor.read_u32::<LittleEndian>().unwrap();
    }
}

#[derive(Default)]
struct SWF {
    header: Header,
}

impl SWF {
    fn read(&mut self, cursor: &mut Cursor<Vec<u8>>) {
        self.header.read(cursor);
    }
}

fn main() {
    let mut f = File::open("/home/sekhmet/main.swf")
        .expect("Couldn't open file");

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .expect("Failed to read file");

    let mut swf = SWF { ..Default::default() };
    let mut cursor = Cursor::new(buffer);
    swf.read(&mut cursor);
}