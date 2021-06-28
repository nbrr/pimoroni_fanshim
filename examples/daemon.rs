use pimoroni_fanshim::Fanshim;
use std::error::Error;

use std::fs;
use std::path::Path;

use std::thread;
use std::time::Duration;

use log::debug;
use env_logger;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Params {
    #[structopt(default_value="31", help="brightness, from 0 to 31")]
    brightness: u8,
    #[structopt(default_value="255")]
    intensity: u8,
    #[structopt(default_value="53", help="temperature under which the fan will stop")]
    rest_temp: u32,
    #[structopt(default_value="55", help="temperature over which the fan will start")]
    blow_temp: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let params = Params::from_args();

    let path = Path::new("/sys/devices/virtual/thermal/thermal_zone0/temp");
    let mut fanshim = Fanshim::default_config()?;
    fanshim.fan_on();
    debug!("fanshim intialized");

    loop {
        let thermal_data = fs::read_to_string(&path);
        let temp = thermal_data?.trim().parse::<u32>()?;
        debug!("temperature read: {}", temp);

        if temp <= params.rest_temp*1000 {
            fanshim.fan_off();
            fanshim.color(params.brightness, 0, params.intensity, 0);
        } else if temp >= params.blow_temp*1000 {
            fanshim.fan_on();
            fanshim.color(params.brightness, params.intensity, 0, 0);
        } else {
            fanshim.color(params.brightness, params.intensity, params.intensity, 0);
        }

        thread::sleep(Duration::from_millis(1000));
    }
}
