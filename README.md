# Prompt-Former

A dead simple way to chat with your your LLM over the command line.

This is a minimalist Rust program designed to make prompting ollama models easy. It allows you to switch between custom prompt templates for different tasks and to different prompt formats for different models.

For example, Dolphin Mistral uses the ChatML format. An example "Hello World!" prompt might look something like this:

```bash
> ollama run dolphin """
<|im_start|>system
You are Dolphin, a helpful AI assistant.<|im_end|>
<|im_start|>user
Hello World! <|im_end|>
<|im_start|>assistant
"""
```

Instead, configure Prompt-Former to use your desired ChatML-formatted template and you can skip the boilerplate:

```bash
> pf "Hello World!" | ollama run dolphin
```

Plenty of libraries, eg, langchain, support prompt templates. This has minimal dependencies, can be run as a standalone app on the command line, and is trivial to extend to new templates.

Give your command line the power of AI:

```bash
> pf "$(cat notes.md) /n/n Summarize the notes above." -t templates/summarize.txt | dolphin

> pf "$(tree -L 2 project_folder) /n/n Based on this directory structure, can you remind me what this project is about?" | dolphin

> pf "$(dmesg --level=err,warn | tail) /n/n Do you know what this error is about in the output of dmesg? Should I be concerned?" | dolphin

> pf "$(cat main.py) /n/n Create a README.md file for this program." | dolphin > README.md
```

See also:

- [Ollama](https://ollama.ai/) installation instructions.
- [Rust](https://www.rust-lang.org/tools/install) installation instructions to install rust + cargo toolchains to build this repo.

## Getting Started

To use `prompt-former`, follow these steps:

1. Clone this repository to your local machine using Git:
   ```bash
   git clone https://github.com/alexansari101/prompt-former.git
   ```
2. Navigate to the cloned directory:
   ```bash
   cd prompt-former
   ```
3. Build the project using Cargo (Rust's build tool):
   ```bash
   cargo build --release
   ```
4. Alias as pf and add binary and folder path by appending to your .bashrc

   ```bash
   PROMPT_FORMER_FOLDER=<YOUR_PATH_HERE>
   PROMPT_FORMER_BIN=$PROMPT_FORMER_FOLDER/target/release/prompt-former

   pf() {
      local current_dir="$PWD"
      cd $PROMPT_FORMER_FOLDER && $PROMPT_FORMER_BIN "$@"
      cd "$current_dir"
   }
   ```

5. Run the binary with your desired arguments, replacing `<prompt>` and `<template_path>` with your input:
   ```bash
   pf <prompt>
   pf -t <template_path> <prompt>
   ```

## Usage

The program takes two required arguments:

- `<prompt>`, a string argument containing the user input prompt to inject into the prompt template.
- `-t, --template`, a path to a prompt template file that includes a placeholder string, "{PROMPT}".

For example:

```
pf "Write a poem about cats." -t templates/poem_template.txt
```

Add your own custom tempates to the templates folder and have fun!

## Contributing

Pull requests are welcome!

Please make sure to update tests as needed.

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.
