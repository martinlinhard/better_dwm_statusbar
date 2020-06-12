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
    let process = Command::new("bash")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    process
        .stdin
        .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Fail"))?
        .write_all(value.as_bytes())?;

    let mut s = String::new();

    process
        .stdout
        .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Fail"))?
        .read_to_string(&mut s)?;

    Ok(s)
}
