extern crate filetime;
extern crate glob;
use std::fs;
use self::filetime::FileTime;
use std::path::Path;
use self::glob::glob;


#[derive(Clone)]
pub struct SyncConfig {
    pub path : String
}

fn check_file<P: AsRef<Path>>(file_path : P, file_stamp : FileTime)->Option<FileTime> {
    let md = match fs::metadata(file_path) {
        Ok(d) => d,
        Err(_) => return None
    };

    let mod_time = FileTime::from_last_modification_time(&md);
    if mod_time > file_stamp {
        return Some(mod_time);
    }
    None
}

pub fn check_files(files : Vec<(String,FileTime)>) -> Vec<(String,FileTime,bool)>{
    let mut res = vec!();
    for &(ref path,time) in files.iter(){
        match check_file(path,time){
            Some(mod_time) =>
                res.push((path.to_owned(),mod_time,true)),
            None =>
                res.push((path.to_owned(),time,false)),
        }
    }
    res
}

pub fn init_files_list(path_pattern : &Path) -> Vec<(String,FileTime,bool)>{
    let mut res = vec!();
    for entry in glob(path_pattern.to_str().unwrap()).unwrap(){
        match entry {
            Ok(path) => {
                let metadata = fs::metadata(&path.to_str().unwrap()).unwrap();

                if metadata.is_dir(){
                    let mod_time = check_file(&path,FileTime::zero()).unwrap();
                    res.push((path.to_str().unwrap().to_owned(),mod_time,true));
                }
            }
            Err(e) => println!("{:?}",e),
        }
    }
    res
}


#[test]
fn test_get_time() {
    let md1 = check_file("Cargo.toml",FileTime::zero());
    assert!(md1.is_some());
    let md2 = check_file("Cargo.toml",md1.unwrap());
    assert!(md2.is_none());
}

// fn get_all_files()

#[test]
fn test_glob() {
    for entry in glob("../redirect-keyboard/**/*").unwrap(){
        match entry {
            Ok(path) => println!("{:?}",path.display()),
            Err(e) => println!("{:?}",e),
        }
    }
}
#[test]
fn test_init() {
    let files = init_files_list(Path::new("src/*"));
    for &(ref path,mod_time,sync) in files.iter(){
        assert!(sync);
        println!("{}: {} - {}",path,mod_time, if sync {"sync"}else {"no-sync"} );
    }
    let new_files = check_files(files.iter().map(|&(ref p,t,_)| (p.to_owned(),t)).collect::<Vec<_>>());
    for &(ref path,mod_time,sync) in new_files.iter(){
        assert!(!sync);
        println!("{}: {} - {}",path,mod_time, if sync {"sync"}else {"no-sync"} );
    }
}

#[test]
fn test_init_on_file() {
    let files = init_files_list(Path::new("src/main.rs"));
    let mut once = false;
    for &(ref path,mod_time,sync) in files.iter(){
        assert!(!once);
        once = true;
        assert!(sync);
        println!("{}: {} - {}",path,mod_time, if sync {"sync"}else {"no-sync"} );
    }
    let new_files = check_files(files.iter().map(|&(ref p,t,_)| (p.to_owned(),t)).collect::<Vec<_>>());
    for &(ref path,mod_time,sync) in new_files.iter(){
        assert!(!sync);
        println!("{}: {} - {}",path,mod_time, if sync {"sync"}else {"no-sync"} );
    }
}

#[test]
fn test_metadata() {
    let metadata = fs::metadata("src/main.rs").unwrap();
    println!("main.rs: {} b", metadata.len());

}
