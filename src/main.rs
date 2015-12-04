extern crate argparse;

use std::net::{TcpListener,TcpStream};
use std::io::{ Result  };
use std::thread;
use argparse::{ArgumentParser, Store,StoreTrue};

mod sync;
mod master;
mod slave;

use sync::{SyncConfig};
use master::{run_master};
use slave::{run_slave};



#[derive(Clone)]
struct Config {
    skip_client : bool,
    port : u16,
    localip : String,
    outsideip : String,
    master : bool
}

fn run(stream : &mut TcpStream, config : Config, sync_config : SyncConfig) -> Result<()>{

    if config.master {
        try!(run_master(stream,sync_config))
    }
    else
    {
        try!(run_slave(stream,sync_config))
    }
    Ok(())
}

fn try_run_client(config : Config, sync_config : SyncConfig) -> Result<()>{
    println!("Trying to connect to {}:{}",config.outsideip, config.port );
    let mut stream = try!(TcpStream::connect((&config.outsideip as &str,config.port)));
    run(&mut stream,config,sync_config)
}


fn run_server(config : Config, sync_config : SyncConfig) -> Result<()> {
    println!("Creating server on {}:{}" , config.localip,config.port );
    let listener = try!(TcpListener::bind((&config.localip as &str,config.port)));
    println!("Waiting for new connections");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                let cfg = config.clone();
                let scfg = sync_config.clone();
                thread::spawn(move||{
                    println!("connected");
                    run(&mut stream.try_clone().unwrap(),cfg,scfg)
                });
            }
            Err(e) => {
                println!("Connection failed {}",e );
            }
        }
    }

    drop(listener);
    Ok(())
}

fn main() {
    let mut config = Config{
        skip_client : false,
        port: 61822,
        localip : "127.0.0.1".to_owned(),
        outsideip : "127.0.0.1".to_owned(),
        master : false
    };
    let mut sync_config = SyncConfig{
        path : "".to_owned()
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Synchronise files content between two computers.");
        ap.refer(&mut config.master)
            .add_option(&["-m","--master"],StoreTrue,"Act as a master, this machine will redirect keyboard to the other one.");
        ap.refer(&mut config.skip_client)
            .add_option(&["-s","--skip_client"],StoreTrue,"Skip connecting to client and create server right away.");
        ap.refer(&mut config.port)
            .add_option(&["-p","--port"],Store,"Port address");
        ap.refer(&mut config.localip)
            .add_option(&["-l","--local"],Store,"Local ip address");
        ap.refer(&mut config.outsideip)
            .add_option(&["-o","--outside"],Store,"Outside ip address");
        ap.refer(&mut sync_config.path).required()
            .add_option(&["-t","--path"],Store,"Path to sync folder or file. To use folder, use wildcard to specify a pattern to use.");

        ap.parse_args_or_exit();
    }
    println!("local: {}, outside: {}, port: {}",config.localip, config.outsideip, config.port );
    let _ = if config.skip_client{
        println!("Skiping connecting to client");
        run_server(config,sync_config)
    }else {match try_run_client(config.clone(),sync_config.clone()){
        Err(_) => {
            println!("Could not connect to server, creating own.");
            run_server(config,sync_config)},
        _ => Ok(())
    }};
}
