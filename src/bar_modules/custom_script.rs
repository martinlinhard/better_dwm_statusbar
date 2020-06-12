use crate::module::Module;
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub struct CustomScript {
    pub format: &'static str,
    /// This has to be the content of a script! (or just a normal command)
    pub command: &'static str,
}

impl CustomScript {
    pub fn new(format: &'static str, command: &'static str) -> Self {
        Self { format, command }
    }
}

impl Module for CustomScript {
    fn yield_next_value(&mut self) -> String {
        let content = execute_in_bash(self.command).unwrap_or(String::new());
        self.format.replace(crate::PLACEHOLDER, content.trim())
    }
}

pub fn execute_in_bash(value: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("bash").arg(value).output()?;
    Ok(String::from_utf8(output.stdout)?)
}
