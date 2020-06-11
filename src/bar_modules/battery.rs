use crate::module::Module;
use std::fs;
use std::io;
use systemstat::platform::common::Platform;

pub type AnimationType = [&'static str; 5];

pub struct Battery {
    pub animation: AnimationType,
    pub animation_index: usize,
    pub format: &'static str,
}

impl Module for Battery {
    fn yield_next_value(&mut self) -> String {
        let symbol: &'static str;
        let power = battery_life().unwrap();
        if let Ok(charging) = crate::SYSTEM.on_ac_power() {
            if charging {
                //device is charging --> play animation
                symbol = self.animation[self.animation_index];
                self.animation_index += 1;
                self.animation_index %= 5;
            } else {
                //device is not charging --> show appropriate level
                if power <= 1.0 {
                    symbol = self.animation[0];
                } else {
                    let curr_index = ((power / 25.0) + (1.0 - ((power / 25.0) % 1.0))) as usize;
                    symbol = self.animation[curr_index];
                }
            }
            let content = format!("{:3.0}% {}", power, symbol);
            return self.format.replace(crate::PLACEHOLDER, &content);
        }
        String::new()
    }
}

impl Battery {
    pub fn init_with_animation(animation: AnimationType, format: &'static str) -> Self {
        Self {
            animation,
            animation_index: 0,
            format,
        }
    }
}

fn battery_life() -> io::Result<f32> {
    let input = "/sys/class/power_supply/BAT0";
    let charge_now = fs::read_to_string(input.to_string() + "/charge_now")?;
    let charge_full = fs::read_to_string(input.to_string() + "/charge_full")?;

    let charge_now: i32 = charge_now
        .trim()
        .parse()
        .and_then(|n| Ok(n))
        .or_else(|_| Err(io::Error::new(io::ErrorKind::Other, format!("error"))))?;

    let charge_full: i32 = charge_full
        .trim()
        .parse()
        .and_then(|n| Ok(n))
        .or_else(|_| Err(io::Error::new(io::ErrorKind::Other, format!("error"))))?;

    let res = charge_now as f32 / charge_full as f32 * 100.;

    Ok(res)
}
