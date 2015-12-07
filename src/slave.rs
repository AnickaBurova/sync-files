extern crate byteorder;
extern crate encoding;

use self::byteorder::{ ReadBytesExt,  LittleEndian};
use std::fs::{File};
use std::io::{Write,Read,Result};
use std::path::Path;

use sync::{SyncConfig};
use self::encoding::{Encoding, DecoderTrap};
use self::encoding::all::ISO_8859_1;


pub fn run_slave<T : Read>(stream : &mut T, cfg : SyncConfig) -> Result<()>{
    loop {
        println!("Waiting for a data");
        let path_length : u16 = try!(stream.read_u16::<LittleEndian>());
        if path_length == 0{
            // ignore empty
            continue;
        }
        println!("Got the file name size: {}", path_length);
        let mut path_data = vec![0u8;0];
        try!(stream.take(path_length as u64).read_to_end(&mut path_data));
        let path = match ISO_8859_1.decode(&path_data[..],DecoderTrap::Strict){
            Ok(s) => s,
            Err(_) => "Error decoding".to_owned()
        };
        let data_length : u32 = try!(stream.read_u32::<LittleEndian>());
        let mut data = vec![0u8;0];
        try!(stream.take(data_length as u64).read_to_end(&mut data));
        println!("Receiving file: {} of size: {}", path, data.len() );
        // let full_path = Path::new(&cfg.path).join(&path);
        println!("Writing data to {}",path);
        let mut file = try!(File::create(path));
        try!(file.write_all(&data[..]));
    }
}
