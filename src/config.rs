use std::{env, error::Error, io::Write, process::exit};

use colored::Colorize;

pub struct Config {
    pub api_key: String,
    pub shell: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
            println!("{}", "This program requires an OpenAI API key to run. Please set the OPENAI_API_KEY environment variable. https://github.com/m1guelpf/plz-cli#usage".red());
            exit(1);
        });

        let shell = env::var("SHELL").unwrap_or_else(|_| "".to_string());

        Ok(Self { api_key, shell })
    }

    pub fn write_to_history(&self, code: &str) -> std::io::Result<()> {
        let history_file = match self.shell.as_str() {
            "/bin/bash" => std::env::var("HOME").unwrap() + "/.bash_history",
            "/bin/zsh" => std::env::var("HOME").unwrap() + "/.zsh_history",
            _ => return Ok(()),
        };

        match std::fs::OpenOptions::new().append(true).open(history_file) {
            Ok(mut file) => match file.write_all(format!("{}\n", code).as_bytes()) {
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };
    }
}
