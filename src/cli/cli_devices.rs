extern crate rusb;
use super::cli_devices_list;
use std::io::{Error, ErrorKind};
use rusb::{Context, UsbContext, DeviceHandle};

pub struct Device {
    index: u8
}

impl Device {
    pub fn new(index: u8) -> Self {
        Self {
            index
        }
    }

    pub fn get_index(&self) -> Result<u8, Error> {
        Ok(self.index)
    }

    pub fn get_device_handle(&self) -> Result<DeviceHandle<Context>, Error> {
        let index = self.index;
        let context = Context::new().unwrap();
        let devices = context.devices().unwrap();

        let device = match devices.iter().nth(index as usize) { // handle <Option>
            Some(device) => device, // pass on the variable
            None => { // error case
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("USB with the '{}' identifier was not found", index)
                ));
            }
        };

        let device_handle = match device.open() { // handle <Result>
            Ok(device_handle) => device_handle, // pass on the variable
            Err(_) => { // error case
                return Err(Error::new(
                    ErrorKind::Unsupported,
                    format!("USB device not openable: identifier '{}'", index)
                ));
            }
        };

        // return device_handle as Result
        Ok(device_handle)
    }
}

pub fn list() {
    cli_devices_list::demo();
}