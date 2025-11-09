use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "File counters")]
#[command(about = "Counts the number of lines, words and characters within that file.")]
struct Arg {
    #[arg(value_name = "path")]
    path: String,
}

fn main() {
    let args = Arg::parse();
    let path = Path::new(&args.path);

    // Check if file exists
    if let Ok(false) = fs::exists(&path) {
        println!("file does not exist");
    }
    if let Err(e) = fs::exists(&path) {
        println!("error opening file");
    }

    // Read file contents
    let content = match fs::read_to_string(args.path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error reading file");
            "".to_string()
        }
    };

    // Calculate word count
    let word_count = count_words(&content);
    let line_count = count_lines(&content);
    let character_count = count_characters(&content);

    println!("Word count: {}", word_count);
    println!("Line count: {}", line_count);
    println!("Character count: {}", character_count);
}

// Count
fn count_words(string: &String) -> usize {
    let words: Vec<&str> = string.split_whitespace().collect();
    words.len()
}
fn count_lines(string: &String) -> usize {
    string.lines().count()
}
fn count_characters(string: &String) -> usize {
    string.chars().count()
}
