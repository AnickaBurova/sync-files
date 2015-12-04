extern crate byteorder;
extern crate encoding;

use self::byteorder::{ WriteBytesExt,  LittleEndian};
use std::fs;
use std::io::{Read,Write,Result};
use std::path::Path;
use std::time::Duration;
use std::thread;

use sync::{SyncConfig,init_files_list,check_files};
use self::encoding::{Encoding, EncoderTrap};
use self::encoding::all::ISO_8859_1;




pub fn run_master<T : Write>(stream : &mut T, cfg : SyncConfig) -> Result<()>{
    let mut files = init_files_list(Path::new(&cfg.path));
    loop {
        for &(ref path, _, sync) in files.iter(){
            if sync {
                let mut file = try!(fs::File::open(&path));
                let mut buffer = Vec::new();
                try!(file.read_to_end(&mut buffer));
                let name_data = match ISO_8859_1.encode(&path[..],EncoderTrap::Strict){
                    Ok(d) => d,
                    Err(e) => {
                        println!("{}", e);vec!()}
                };

                try!(stream.write_u16::<LittleEndian>(name_data.len() as u16));
                try!(stream.write(&name_data[..]));

                try!(stream.write_u32::<LittleEndian>(buffer.len() as u32));
                try!(stream.write(&buffer[..]));
            }
        }
        thread::sleep(Duration::new(2,0));
        files = check_files(files.iter().map(|&(ref p,t,_)| (p.to_owned(),t)).collect::<Vec<_>>());
    }
}
