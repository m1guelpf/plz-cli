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
    let code = generate_command(&config, &cli.prompt.join(" "));

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
        execute_command(&config, &code);
    }
}

fn generate_command(config: &Config, prompt: &str) -> String {
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Generating your command...".into());
    let body = match request_completion(config, prompt) {
        Ok(body) => body,
        Err(error_message) => fail(&mut spinner, &error_message),
    };
    let code = match command_from_body(&body) {
        Ok(command) => command,
        Err(error_message) => fail(&mut spinner, &error_message),
    };

    spinner.stop_and_persist(
        "✔".green().to_string().as_str(),
        "Got some code!".green().to_string(),
    );
    code
}

fn request_completion(config: &Config, prompt: &str) -> Result<serde_json::Value, String> {
    let response = Client::new()
        .post(format!("{}/chat/completions", config.api_base))
        .json(&completion_body(config, prompt))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .send()
        .map_err(|err| err.to_string())?;

    let status_code = response.status();
    let body = response
        .json::<serde_json::Value>()
        .unwrap_or(serde_json::Value::Null);
    if status_code.is_client_error() {
        let error_message = body["error"]["message"]
            .as_str()
            .unwrap_or("unknown client error");
        return Err(error_message.to_string());
    }
    if status_code.is_server_error() {
        return Err(format!(
            "OpenAI is currently experiencing problems. Status code: {status_code}"
        ));
    }
    Ok(body)
}

fn completion_body(config: &Config, prompt: &str) -> serde_json::Value {
    json!({
        "model": config.api_model,
        "reasoning_effort": "none",
        "max_completion_tokens": 1024,
        "messages": [
            {
                "role": "system",
                "content": "Return the shell command by calling run_shell. The command must be valid for the user's operating system."
            },
            {
                "role": "user",
                "content": build_prompt(prompt)
            }
        ],
        "tools": [
            {
                "type": "function",
                "function": {
                    "name": "run_shell",
                    "description": "The shell command that fulfills the user's request.",
                    "strict": true,
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "command": {
                                "type": "string",
                                "description": "A bash command or script to execute."
                            }
                        },
                        "required": ["command"],
                        "additionalProperties": false
                    }
                }
            }
        ],
        "tool_choice": {
            "type": "function",
            "function": {
                "name": "run_shell"
            }
        }
    })
}

fn execute_command(config: &Config, code: &str) {
    config.write_to_history(code);
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Executing...".into());

    let output = Command::new("bash")
        .arg("-c")
        .arg(code)
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

fn fail(spinner: &mut Spinner, message: &str) -> ! {
    spinner.stop_and_persist(
        "✖".red().to_string().as_str(),
        format!("API error: \"{message}\"").red().to_string(),
    );
    std::process::exit(1);
}

fn command_from_body(body: &serde_json::Value) -> Result<String, String> {
    let arguments = &body["choices"][0]["message"]["tool_calls"][0]["function"]["arguments"];
    let parsed = if let Some(text) = arguments.as_str() {
        serde_json::from_str::<serde_json::Value>(text)
            .map_err(|err| format!("model returned invalid tool arguments: {err}"))?
    } else if arguments.is_object() {
        arguments.clone()
    } else {
        return Err("model did not return a run_shell tool call".to_string());
    };

    parsed["command"]
        .as_str()
        .map(str::trim)
        .filter(|command| !command.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| "model returned an empty command".to_string())
}

fn build_prompt(prompt: &str) -> String {
    let os_hint = if cfg!(target_os = "macos") {
        " (on macOS)"
    } else if cfg!(target_os = "linux") {
        " (on Linux)"
    } else {
        ""
    };

    format!("{prompt}{os_hint}")
}
