use crate::module::Module;
use chrono::Local;

const TIME_FORMAT: &'static str = "%d.%m.%Y, %H:%M:%S";

pub struct Time {
    pub format: &'static str,
}

impl Time {
    pub fn new(format: &'static str) -> Self {
        Self { format }
    }
}

impl Module for Time {
    fn yield_next_value(&mut self) -> String {
        self.format.replace(
            crate::PLACEHOLDER,
            &Local::now().format(TIME_FORMAT).to_string(),
        )
    }
}
