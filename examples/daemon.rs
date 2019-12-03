use pimoroni_fanshim::Fanshim;

use std::fs;

fn main() {
    let rest_temp: u32 = 54;
    let blow_temp: u32 = 55;

    loop {
        let thermal_data = read_to_string("/sys/devices/virtual/thermal/thermal_zone0/temp");
        let temp = thermal_data?.parse::<u32>();
    }
}
