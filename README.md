# Copilot, for your terminal

A CLI tool that generates shell scripts from a human readable description.

## Installation

You can install `plz` by running the following command in your terminal.

```
curl -fsSL https://raw.githubusercontent.com/m1guelpf/plz-cli/main/install.sh | sh -
```

### Homebrew

You can also install `plz` using [Homebrew](https://brew.sh/).

```sh
$ brew install plz-cli
```

You may need to close and reopen your terminal after installation. Alternatively, you can download the binary corresponding to your OS from the [latest release](https://github.com/m1guelpf/plz-cli/releases/latest).

## Usage

`plz` uses [GPT-6 Luna](https://developers.openai.com/api/docs/models/gpt-6-luna) through the Chat Completions API. The legacy Completions API, including `gpt-3.5-turbo-instruct`, is shut down. Grab an API key from [your dashboard](https://platform.openai.com/api-keys), and save it to `OPENAI_API_KEY` (you can also save it in your bash/zsh profile so it persists between sessions).

```bash
export OPENAI_API_KEY='sk-XXXXXXXX'
```

The model defaults to `gpt-6-luna`. Override it with `OPENAI_API_MODEL`. Override the API host with `OPENAI_API_BASE` (default `https://api.openai.com/v1`).

```bash
export OPENAI_API_MODEL='gpt-6-luna'
export OPENAI_API_BASE='https://api.openai.com/v1'
```

GPT-6 Luna only accepts function calls on Chat Completions when `reasoning_effort` is `none`. `plz` always sends that setting and asks the model to return the command through the `run_shell` tool.

Once you have configured your environment, run `plz` followed by whatever it is that you want to do (`plz show me all options for the plz cli`).

To get a full overview of all available options, run `plz --help`

```sh
$ plz --help
Generates bash scripts from the command line

Usage: plz [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  Description of the command to execute

Options:
  -y, --force    Run the generated program without asking for confirmation
  -h, --help     Print help information
  -V, --version  Print version information
```

## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
