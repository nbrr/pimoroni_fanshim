use pimoroni_fanshim::Fanshim;
use std::error::Error;

use std::path::PathBuf;
use structopt::StructOpt;

use env_logger;
use log::debug;

#[derive(StructOpt, Debug)]
#[structopt(name = "Status of the led")]
enum Switch {
    Off,
    On {
        #[structopt(short, help = "brightness, from 0 to 31")]
        br: u8,
        #[structopt(short, help = "red, from 0 to 255")]
        r: u8,
        #[structopt(short, help = "green, from 0 to 255")]
        g: u8,
        #[structopt(short, help = "blue, from 0 to 255")]
        b: u8,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Switch::from_args();

    let mut fanshim = Fanshim::default_config()?;
    fanshim.led_off();
    fanshim.color(1., 255, 255, 255);
    debug!("fanshim intialized");

    match args {
        Switch::Off => {
            fanshim.led_off();
            debug!("fanshim led off")
        }
        Switch::On { br, r, g, b } => {
            fanshim.color2(br, r, g, b);
            debug!("fanshim led color {} {} {} {}", br, r, g, b)
        }
    }
    Ok(())
}
