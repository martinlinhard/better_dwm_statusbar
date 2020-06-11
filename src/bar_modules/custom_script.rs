use crate::module::Module;
use std::process::Command;

pub struct CustomScript {
    pub format: &'static str,
    pub command: Command,
}

impl CustomScript {
    pub fn new(format: &'static str, command: Command) -> Self {
        Self { format, command }
    }
}

impl Module for CustomScript {
    fn yield_next_value(&mut self) -> String {
        // If the output is okay, we want to convert it to a string
        let output = self
            .command
            .output()
            .map(|output| String::from_utf8(output.stdout).unwrap());

        // If the output was an error, we want to convert it to an empty string
        let content = output.unwrap_or(String::new());

        self.format.replace(crate::PLACEHOLDER, content.trim())
    }
}
