#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use config::Config;
use question::{Answer, Question};
use reqwest::blocking::Client;
use serde_json::json;
use spinners::{Spinner, Spinners};
use std::process::Command;

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the command to execute
    prompt: Vec<String>,

    /// Run the generated program without asking for confirmation
    #[clap(short = 'y', long)]
    force: bool,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new();

    let client = Client::new();
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Generating your command...".into());
    let api_addr = format!("{}/completions", config.api_base);
    let response = client
        .post(api_addr)
        .json(&json!({
            "top_p": 1,
            "stop": "```",
            "temperature": 0,
            "suffix": "\n```",
            "max_tokens": 1000,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "model": "gpt-3.5-turbo-instruct",
            "prompt": build_prompt(&cli.prompt.join(" ")),
        }))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .send()
        .unwrap();

    let status_code = response.status();
    if status_code.is_client_error() {
        let response_body = response.json::<serde_json::Value>().unwrap();
        let error_message = response_body["error"]["message"].as_str().unwrap();
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("API error: \"{error_message}\"").red().to_string(),
        );
        std::process::exit(1);
    } else if status_code.is_server_error() {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("OpenAI is currently experiencing problems. Status code: {status_code}")
                .red()
                .to_string(),
        );
        std::process::exit(1);
    }

    let code = response.json::<serde_json::Value>().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    spinner.stop_and_persist(
        "✔".green().to_string().as_str(),
        "Got some code!".green().to_string(),
    );

    PrettyPrinter::new()
        .input_from_bytes(code.as_bytes())
        .language("bash")
        .grid(true)
        .print()
        .unwrap();

    let should_run = if cli.force {
        true
    } else {
        Question::new(
            ">> Run the generated program? [Y/n]"
                .bright_black()
                .to_string()
                .as_str(),
        )
        .yes_no()
        .until_acceptable()
        .default(Answer::YES)
        .ask()
        .expect("Couldn't ask question.")
            == Answer::YES
    };

    if should_run {
        config.write_to_history(code.as_str());
        spinner = Spinner::new(Spinners::BouncingBar, "Executing...".into());

        let output = Command::new("bash")
            .arg("-c")
            .arg(code.as_str())
            .output()
            .unwrap_or_else(|_| {
                spinner.stop_and_persist(
                    "✖".red().to_string().as_str(),
                    "Failed to execute the generated program.".red().to_string(),
                );
                std::process::exit(1);
            });

        if !output.status.success() {
            spinner.stop_and_persist(
                "✖".red().to_string().as_str(),
                "The program threw an error.".red().to_string(),
            );
            println!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }

        spinner.stop_and_persist(
            "✔".green().to_string().as_str(),
            "Command ran successfully".green().to_string(),
        );

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn build_prompt(prompt: &str) -> String {
    let os_hint = if cfg!(target_os = "macos") {
        " (on macOS)"
    } else if cfg!(target_os = "linux") {
        " (on Linux)"
    } else {
        ""
    };

    format!("{prompt}{os_hint}:\n```bash\n#!/bin/bash\n")
}
