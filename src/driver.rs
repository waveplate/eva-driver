use std::thread;
use std::time::{Duration, SystemTime};
use rusb::{Result, Error, GlobalContext, DeviceHandle, DeviceList};

use crate::handshake::{EvaHandshake, consts};

pub const VID: u16 = 0x0c2a;
pub const PID: [u16; 2] = [0x4000, 0x4001];

pub struct EvaDriver {
    pub handle: DeviceHandle<GlobalContext>,
}

impl EvaDriver {
    pub fn init_device(timeout: Duration) -> Result<EvaDriver> {
        if EvaDriver::get_current_pid(VID)? == PID[0] {
            EvaDriver::open_device_handle(VID, PID[0])?.switch_device();
        }

        let start = SystemTime::now();

        loop {
            let elapsed = SystemTime::now().duration_since(start);

            if elapsed.is_err() {
                return Err(Error::Other);
            }

            if elapsed.unwrap().as_millis() > timeout.as_millis() {
                return Err(Error::Timeout);
            }

            match EvaDriver::get_current_pid(VID).ok() {
                None => continue,
                Some(pid) => {
                    if pid == PID[1] {
                        break;
                    }
                },
            }

            thread::sleep(Duration::from_millis(100));
        }

        let mut driver = EvaDriver::open_device_handle(VID, PID[1])?;
        driver.handle.claim_interface(0)?;
        driver.handshake();

        return Ok(driver);
    }


    pub fn wait_for_acquisition(&mut self) {
        let mut count = 0;
        loop {
            self.bulk_transfer_write(consts::URB_BULK_OUT_1, consts::CHK.to_vec());
            match self.bulk_transfer_read(consts::URB_BULK_IN_1) {
                None => {},
                Some(data) => {
                    if data.len() == 4 && data[3] == 0x00 {
                        count+=1;
                        if count == 2 {
                            return;
                        }
                    }
                },
            }
        }
    }

    pub fn acquire_image (&mut self) -> Vec<u8> {
        let mut idx = consts::IDX.to_vec();
        let mut image: Vec<u8> = Vec::new();

        loop {
            idx = self.update_acquisition_idx(idx);
            image.append(&mut self.read_acquisition_data());

            if idx[7] == 0x80 && idx[8] == 0x1f {
                self.end_acquisition();
                break;
            }
        }

        return image
    }

    fn open_device_handle(vid: u16, pid: u16) -> Result<EvaDriver> {
        for device in DeviceList::new()?.iter() {
            let descriptor = device.device_descriptor()?;
            if descriptor.vendor_id() == vid && descriptor.product_id() == pid {
                let handle = device.open()?;
                return Ok(EvaDriver {
                    handle: handle
                });
            }
        }
        return Err(Error::NoDevice);
    }

    fn get_current_pid(vid: u16) -> Result<u16> {
        for device in DeviceList::new()?.iter() {
            let descriptor = device.device_descriptor()?;
            if descriptor.vendor_id() == vid {
                return Ok(descriptor.product_id())
            }
        }
        return Err(Error::NoDevice);
    }

    fn switch_device(&mut self) -> Option<Vec<u8>> {
        let mut buf = [0; 1];
        match self.handle.write_control(0x40, 0xa0, 0x7f92, 0x00, &mut buf, Duration::from_millis(0)) {
            Err(err) => panic!("{}", err.to_string()),
            Ok(nbytes) => Some(buf[0..nbytes].to_vec()),
        }
    }

    fn handshake(&mut self){
        for step in EvaHandshake::sequence().transfers {
            if step.data.is_none() {
                self.bulk_transfer_read(step.endpoint);
            } else {
                self.bulk_transfer_write(step.endpoint, step.data.unwrap());
            }
        }
    }

    fn bulk_transfer_read(&mut self, endpoint: u8) -> Option<Vec<u8>> {
        let mut buf: [u8; 32768] = [0; 32768];
        return match self.handle.read_bulk(endpoint, &mut buf, Duration::from_millis(0)) {
            Err(_) => None,
            Ok(nbytes) => Some(buf[0..nbytes].to_vec()),
        }
    }

    fn bulk_transfer_write(&mut self, endpoint: u8, data: Vec<u8>) -> Option<usize> {
        return match self.handle.write_bulk(endpoint, data.as_slice(), Duration::from_millis(0)) {
            Err(_) => return None,
            Ok(nbytes) => Some(nbytes),
        }
    }

    fn update_acquisition_idx(&mut self, mut idx: Vec<u8>) -> Vec<u8> {
        self.bulk_transfer_write(consts::URB_BULK_OUT_1, idx.clone());
        self.bulk_transfer_read(consts::URB_BULK_IN_1);

        match idx[7] {
            0x00 => {
                idx[7] = 0x80;
                return idx;
            },
            _ => {
                idx[7] = 0x00;
                idx[8] += 1;
                return idx;
            }
        }
    }

    fn read_acquisition_data(&mut self) -> Vec<u8> {
        match self.bulk_transfer_read(consts::URB_BULK_IN_2) {
            None => Vec::new(),
            Some(buf) => {
                self.bulk_transfer_write(consts::URB_BULK_OUT_2, consts::ACK.to_vec());
                return buf;
            }
        }
    }

    fn end_acquisition(&mut self) {
        self.bulk_transfer_write(consts::URB_BULK_OUT_1, consts::SETUP_5.to_vec());
        self.bulk_transfer_read(consts::URB_BULK_IN_1);
    }

}