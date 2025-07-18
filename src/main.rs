use clap::Subcommand;
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

const TASKS_FILE: &str = "tasks.json";

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    name: String,
    description: Option<String>,
    done: bool,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    #[command(alias = "a")]
    Add {
        name: String,
        description: Option<String>,
    },
    /// List all tasks
    #[command(alias = "l")]
    List,
    /// Remove a task by its id
    #[command(alias = "r")]
    Remove {
        id: usize,
    },
    /// Show details of a task by its id
    #[command(alias = "d")]
    Describe {
        id: usize,
    },
    /// Mark a task as done by its id
    #[command(alias = "do")]
    Done {
        id: usize,
    },
    /// Mark a task as undone by its id
    #[command(alias = "u")]
    Undo {
        id: usize,
    },
    /// Print all tasks as JSON
    #[command(alias = "j")]
    Json,
}

fn load_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();
    if let Ok(mut file) = File::open(TASKS_FILE) {
        let mut data = String::new();
        if file.read_to_string(&mut data).is_ok() {
            tasks = serde_json::from_str(&data).unwrap_or_default();
        }
    }
    tasks
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TASKS_FILE)?;
    file.write_all(data.as_bytes())
}

fn main() {
    let mut tasks = load_tasks();

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, description } => {
            let task = Task {
                id: tasks.len() + 1,
                name,
                description,
                done: false,
            };
            tasks.push(task);
            save_tasks(&tasks).unwrap();
            println!("Task added: {}", tasks.last().unwrap().name);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks available.");
            } else {
                for (index, task) in tasks.iter().enumerate() {
                    let status_icon = if task.done { "✅" } else { "🔳" };
                    println!("{}: {} {}", index + 1, status_icon, task.name);
                }
            }
        }
        Commands::Remove { id } => {
            if id > 0 && id <= tasks.len() {
                tasks.remove(id - 1);
                save_tasks(&tasks).unwrap();
                println!("Task {} removed.", id);
            } else {
                println!("Invalid task number. Please enter a number between 1 and {}.", tasks.len());
            }
        }
        Commands::Describe { id } => {
            if id > 0 && id <= tasks.len() {
                let task = &tasks[id - 1];
                println!("Task {}: {}", task.id, task.name);
                match &task.description {
                    Some(desc) => println!("Description: {}", desc),
                    None => println!("Description: (none)"),
                }
                println!("Done: {}", if task.done { "✅" } else { "🔳" });
            } else {
                println!("Invalid task number. Please enter a number between 1 and {}.", tasks.len());
            }
        }
        Commands::Done { id } => {
            if id > 0 && id <= tasks.len() {
                tasks[id - 1].done = true;
                save_tasks(&tasks).unwrap();
                println!("Task {} marked as done.", id);
            } else {
                println!("Invalid task number. Please enter a number between 1 and {}.", tasks.len());
            }
        }
        Commands::Undo { id } => {
            if id > 0 && id <= tasks.len() {
                tasks[id - 1].done = false;
                save_tasks(&tasks).unwrap();
                println!("Task {} marked as undone.", id);
            } else {
                println!("Invalid task number. Please enter a number between 1 and {}.", tasks.len());
            }
        }
        Commands::Json => {
            let json = serde_json::to_string_pretty(&tasks).unwrap();
            println!("{}", json);
        }
    }
}