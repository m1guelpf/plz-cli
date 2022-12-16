# Copilot, for your terminal

A CLI tool that generates shell scripts from a human readable description.

## Installation

You can install `plz` by running the following command in your terminal.

```
curl -fsSL https://raw.githubusercontent.com/m1guelpf/plz-cli/main/install.sh | sh -
```

You may need to close and reopen your terminal after installation. Alternatively, you can download the binary corresponding to your OS from the [latest release](https://github.com/m1guelpf/plz-cli/releases/latest).

## Usage

`plz` uses [GPT-3](https://beta.openai.com/). To use it, you'll need to grab an API key from [your dashboard](https://beta.openai.com/), and save it to `OPENAI_API_KEY` as follows (you can also save it in your bash/zsh profile for persistance between sessions).

```bash
export OPENAI_API_KEY='sk-XXXXXXXX'
```

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
