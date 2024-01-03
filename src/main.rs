use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// A simple program that injects a user prompt into an llm prompt template.
#[derive(Parser, Debug)]
struct Args {
    /// The user input prompt to inject into the prompt template.
    prompt: String,

    /// Path to a prompt template that inclues a placeholder string, "{PROMPT}."
    #[clap(short, long, default_value = "template.txt")]
    template: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Get the content of the prompt template file into a variable
    let template = match read_file(&args.template) {
        Ok(content) => content,
        Err(_) => {
            println!("Error: Unable to read prompt template {:?}.", args.template);
            return;
        }
    };

    // Replace {PROMPT} with the provided string
    let updated_text = replace_prompt(&template, &args.prompt);

    // Print the updated text as command line output
    print!("{}", updated_text);
}

fn read_file(path: &PathBuf) -> Result<String, io::Error> {
    let mut content = String::new();
    fs::File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}

fn replace_prompt(template: &str, prompt: &str) -> String {
    format!("{}", template.replace("{PROMPT}", prompt))
}
