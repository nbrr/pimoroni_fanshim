use pimoroni_fanshim::Fanshim;
use std::error::Error;

use std::fs;
use std::path::Path;

use log::debug;
use simple_logger;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init().unwrap();
    const REST_TEMP: u32 = 53;
    const BLOW_TEMP: u32 = 55;

    let path = Path::new("/sys/devices/virtual/thermal/thermal_zone0/temp");
    let mut fanshim = Fanshim::default_config()?;
    fanshim.fan_on();
    debug!("fanshim intialized");
    fanshim.color(1., 255, 255, 255);

    loop {
        let thermal_data = fs::read_to_string(&path);
        let temp = thermal_data?.parse::<u32>()?;
        debug!("temperature read: {}", temp);

        if temp <= REST_TEMP {
            fanshim.fan_off();
            fanshim.color(0.5, 0, 255, 0);
        } else if temp >= BLOW_TEMP {
            fanshim.fan_off();
            fanshim.color(0.5, 255, 0, 0);
        } else {
            fanshim.color(0.5, 255, 255, 0);
        }
    }
}
