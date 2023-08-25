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

pub fn handle_verb(verb: &str, device: &Device) {
    match device.get_device_handle() {
        Ok(mut device_handle) => {
            println!("Device: {:?}", device.get_index().unwrap());
            println!(
                "device_handle: {}",
                device_handle
                .device()
                .device_descriptor()
                .unwrap()
                .product_id()
            );
            
            match verb {
                "enable" => {
                    // Handle "enable" verb
                }
                "disable" => {
                    // TODO: actually get the correct interface instead of "0", might have to
                        // -- according to my research most USB devices will just have the one, so 0 is correct, but... 
                        //
                        // get device()
                        // get device config_descriptor()
                        // unpack result/err
                        // get configdescriptor interfaces()
                        // .Iter interfaces so it -> interface
                        // interface num
                        // .. uncommented code cont, replace 0 with var for iter ..
                        //
                        // something like: device_handle.device().active_config_descriptor().unwrap().interfaces.count()
                        // (but with an iter instead of the count thing)
                        //
                    device_handle.claim_interface(0);
                    device_handle.detach_kernel_driver(0).unwrap_or_else(|error| println!("{error}"));
                }
                _ => {
                    panic!("Invalid verb, use disable or enable");
                }
            }
        },
        Err(error) => {
            eprintln!("{error}");
        }
    };
}