use clap::Parser;
use serde_json;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// A simple program that injects a user prompt into an llm prompt template.
#[derive(Parser, Debug)]
struct Args {
    /// The user input prompt to inject into the prompt template.
    prompt: String,

    /// Path to a prompt template that inclues a placeholder "{PROMPT}."
    #[clap(short, long)]
    template: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let template_path = match args.template {
        Some(template) => template,
        None => {
            let config = load_config();
            PathBuf::from(config["prompt-template"].as_str().unwrap())
        }
    };

    // Get the content of the prompt template file into a variable
    let template = match read_file(&template_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Error: Unable to read prompt template {:?}.", template_path);
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

fn load_config() -> serde_json::Value {
    let mut file = fs::File::open("config.json").expect("Unable to open config.json");
    let mut config = String::new();
    file.read_to_string(&mut config)
        .expect("Something went wrong reading the config file");
    let json: serde_json::Value =
        serde_json::from_str(&config).expect("Unable to parse config JSON");
    json
}
