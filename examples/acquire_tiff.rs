extern crate tiff;

use std::fs;
use std::io::Result;
use std::time::{Duration,SystemTime,UNIX_EPOCH};
use eva::driver::EvaDriver;

use std::io::{Cursor};
use tiff::encoder::{colortype, TiffEncoder};

const IMGX: u32 = 1201;

fn main() {
    match EvaDriver::init_device(Duration::from_secs(5)) {
        Err(err) => panic!("{}", err.to_string()),
        Ok(mut driver) => {
            println!("initialised device");
            loop {
                driver.wait_for_acquisition();
                println!("starting image acquisition...");
                match save_tiff(driver.acquire_image()) {
                    Err(err) => eprintln!("{:?}", err),
                    Ok(filename) => println!("saved image as {}", filename),
                }
            }
        }
    }
}

fn save_tiff(buf: Vec<u8>) -> Result<String> {
    let filename = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string() + ".tiff";

    let data = buf.chunks_exact(2).map(|bytes| !u16::from(u16::from(bytes[0]) ^ (u16::from(bytes[1]) << 8)) << 4).collect::<Vec<u16>>();
    
    let mut file = Cursor::new(Vec::new());
    let mut tiff = TiffEncoder::new_big(&mut file).unwrap();
    let mut image = tiff.new_image::<colortype::Gray16>(IMGX, (data.len()/(IMGX as usize)) as u32).unwrap();

    image
        .encoder();

    image.write_data(&data).unwrap();

    match fs::write(filename.clone(), file.into_inner()) {
        Err(err) => Err(err),
        Ok(_) => Ok(filename),
    }
}