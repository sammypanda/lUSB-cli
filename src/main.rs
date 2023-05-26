extern crate libusb;
use std::time::Duration;

fn main() {
    let context = libusb::Context::new().unwrap(); // instantiate a libusb context

    //
    // translate devices from <Result> and iterate over
    //
    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap(); // translate the device description from <Result>
        let device_handle; // allows us to access extra deets

        // open device for handle with consideration that it might be empty
        match device.open() { 
            Ok(result) => device_handle = result,
            Err(_error) => {
                println!("Bus {}, Device {} - unused: no further work :)",
                    device.bus_number(),
                    device.address()
                );
                continue;
            }
        }
        
        let device_languages = device_handle.read_languages( // dependency for reading string descriptors
            Duration::new(30, 0) // timeout after 30 seconds, 0 nanoseconds 
        ).unwrap(); 

        for each in &device_languages {
            println!("{}", each.lang_id());
        }

        println!("Bus {} Device {} ID {}:{} (n/a)",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
            // device_handle.read_product_string(
            //     lang_id,
            //     device_desc
            // )
        );
    }
}