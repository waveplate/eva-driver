use std::fs;
use std::io::Result;
use std::time::{Duration,SystemTime,UNIX_EPOCH};
use eva::driver::EvaDriver;

fn main() {
    match EvaDriver::init_device(Duration::from_secs(5)) {
        Err(err) => panic!("{}", err.to_string()),
        Ok(mut driver) => {
            println!("initialised device");
            loop {
                driver.wait_for_acquisition();
                println!("starting image acquisition...");
                match save_raw(driver.acquire_image()) {
                    Err(err) => eprintln!("{:?}", err),
                    Ok(filename) => println!("saved image as {}", filename),
                }
            }
        }
    }
}

fn save_raw(image: Vec<u8>) -> Result<String> {
    let filename = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string() + ".raw";
    match fs::write(
        filename.clone(),
        image
    ) {
        Err(err) => Err(err),
        Ok(_) => Ok(filename),
    }
}