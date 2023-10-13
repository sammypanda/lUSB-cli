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
                    interface_loop(&mut device_handle, true)
                }
                "disable" => {
                    interface_loop(&mut device_handle, false)
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

fn interface_loop<T: UsbContext>(device_handle: &mut DeviceHandle<T>, enable: bool) {
    device_handle.device().active_config_descriptor().unwrap().interfaces().enumerate().for_each(|(index, _interface)| {
        let index = index as u8;

        if enable {
            device_handle.attach_kernel_driver(index).unwrap_or_else(|error| println!("{error}"));
        } else {
            device_handle.detach_kernel_driver(index).unwrap_or_else(|error| println!("{error}"));
        }
    });
}