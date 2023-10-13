extern crate rusb;
use std::time::Duration;

const ID_ENGLISH: u16 = 1033; // the language code for US English

//
// translate devices from <Result> and iterate over
//
pub fn demo() {
    let mut index = 0;

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap(); // translate the device description from <Result>
        let device_handle; // allows us to access extra deets
        let device_string;

        // open device for handle with consideration that it might be empty
        match device.open() {
            Ok(result) => device_handle = result,
            Err(error) => {
                println!("\u{274C} - Bus {}, Device {}: {}", // 2A2F is a ⨯ (cross product) unicode symbol
                    device.bus_number(),
                    device.address(),
                    error
                );
                continue;
            }
        }

        let device_languages = device_handle.read_languages( // dependency for reading string descriptors
            Duration::new(30, 0) // timeout after 30 seconds, 0 nanoseconds
        ).unwrap();

        device_string = device_languages.iter() // convert Vec to Iter to check if our language exists in
        .find(|language| language.lang_id() == ID_ENGLISH) // pass `language` parameter to represent `Some` value and get the `Language` id
        .and_then(|language| { // operation to be performed on `Some` value returned by `find()`
            device_handle.read_product_string(language.clone(), &device_desc, Duration::new(30, 0)) // get the product string
            .map_err(|error| error.to_string()) // if there is an error
            .ok() // convert `Result` into `Option<String>`
        })
        .unwrap_or_else(|| { // unwrapping `Option<String>` but with an `else` for...
            // handling the case when the desired language is not found
            String::from("Language not found")
        });

        println!("{}  - Bus {} Device {} ID {:04x}:{:04x} ({})",
            index,
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
            device_string
        );

        index += 1;
    }
}
