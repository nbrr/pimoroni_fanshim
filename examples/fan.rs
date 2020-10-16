use pimoroni_fanshim::Fanshim;
use std::error::Error;

use structopt::StructOpt;

use env_logger;
use log::debug;

#[derive(StructOpt, Debug)]
#[structopt(name = "Status of the fan")]
enum Switch {
    Off,
    On,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Switch::from_args();

    let mut fanshim = Fanshim::default_config()?;
    debug!("fanshim intialized");

    match args {
        Switch::Off => {
            fanshim.fan_off();
            debug!("fanshim fan off")
        }
        Switch::On => {
            fanshim.fan_on();
            debug!("fanshim fan on")
        }
    }
    Ok(())
}
