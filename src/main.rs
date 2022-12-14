use bat::PrettyPrinter;
use clap::Parser;
use question::{Answer, Question};
use reqwest::blocking::Client;
use serde_json::json;
use spinners::{Spinner, Spinners};
use std::{env, fs, io::Write, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the command to execute
    prompt: String,

    /// Run the generated program without asking for confirmation
    #[clap(short = 'y', long)]
    force: bool,
}

fn main() {
    let cli = Cli::parse();
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("This program requires an OpenAI API key to run. Please set the OPENAI_API_KEY environment variable.");
        std::process::exit(1);
    });

    let mut spinner = Spinner::new(Spinners::BouncingBar, "Generating your command...".into());

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .json(&json!({
            "top_p": 1,
            "temperature": 0,
            "suffix": "\n```",
            "max_tokens": 1000,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "model": "text-davinci-003",
            "prompt": format!("{}:\n```bash\n#!/bin/bash\n", cli.prompt),
        }))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap_or_else(|_| {
            spinner.stop_and_persist(
                "✖",
                "Failed to get a response. Have you set the OPENAI_API_KEY variable?".into(),
            );
            std::process::exit(1);
        });

    let text = response.json::<serde_json::Value>().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();

    spinner.stop_and_persist("✔", "Got some code!".into());

    PrettyPrinter::new()
        .input_from_bytes(text.trim().as_bytes())
        .language("bash")
        .grid(true)
        .print()
        .unwrap();

    let mut file = fs::File::create(".tmp.sh").unwrap();
    file.write_all(text.as_bytes()).unwrap();

    let mut should_run = true;
    if !cli.force {
        should_run = Question::new("\x1b[90m>> Run the generated program? [Y/n]\x1b[0m")
            .yes_no()
            .until_acceptable()
            .default(Answer::YES)
            .ask()
            .expect("Couldn't ask question.")
            == Answer::YES;
    }

    if should_run {
        spinner = Spinner::new(Spinners::BouncingBar, "Executing...".into());

        let output = Command::new("bash")
            .arg(".tmp.sh")
            .output()
            .unwrap_or_else(|_| {
                spinner.stop_and_persist("✖", "Failed to execute the generated program.".into());
                std::process::exit(1);
            });

        spinner.stop_and_persist("✔", "Command ran successfully".into());

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    fs::remove_file(".tmp.sh").unwrap();
}
