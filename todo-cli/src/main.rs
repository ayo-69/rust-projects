use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const DATA_FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(value_name = "DESCRIPTION")]
        description: String,
    },
    List,
    Done {
        #[arg(value_name = "ID")]
        id: usize,
    },
    Delete {
        #[arg(value_name = "ID")]
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            let id = tasks.len() + 1;
            let task = Task {
                id,
                description,
                done: false,
            };
            tasks.push(task.clone());
            save_tasks(&tasks);
            println!("Task added: {} (ID: {})", task.description, task.id);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks found.");
                return;
            }
            println!("ID | Done | Task");
            println!("---|------|-----");
            for task in &tasks {
                let status = if task.done { "✓" } else { " " };
                println!("{} | {} | {}", task.id, status, task.description);
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_tasks(&tasks);
                println!("Task {} marked as done", id);
            } else {
                eprintln!("Task with ID {} not found.", id);
            }
        }
        Commands::Delete { id } => {
            if tasks.iter().any(|t| t.id == id) {
                tasks.retain(|t| t.id != id);
                // Re-index IDs
                for (index, task) in tasks.iter_mut().enumerate() {
                    task.id = index + 1;
                }
                save_tasks(&tasks);
                println!("Task {} deleted.", id);
            } else {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }
}

fn load_tasks() -> Vec<Task> {
    if !Path::new(DATA_FILE).exists() {
        return Vec::new();
    }

    let mut file = match File::open(DATA_FILE) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return Vec::new();
    }

    serde_json::from_str(&contents).unwrap_or_default()
}

fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DATA_FILE)
        .expect("Failed to open tasks file");

    file.write_all(json.as_bytes())
        .expect("Failed to write tasks");
}
