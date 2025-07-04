use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const TASKS_FILE: &str = "tasks.json";

/// Represents a single task in the to-do list.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    /// A unique identifier for the task.
    id: u32,
    /// A description of the task.
    description: String,
    /// The completion status of the task.
    completed: bool,
}

/// Defines the command-line interface for the task manager.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The subcommand to be executed.
    #[clap(subcommand)]
    command: Commands,
}

/// Defines the available subcommands for the CLI.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add {
        /// The description of the task
        description: String,
    },
    /// List all tasks
    List,
    /// Delete a task
    Delete {
        /// The id of the task to delete
        id: u32,
    },
}

/// Reads the list of tasks from the tasks file.
/// If the file does not exist, it will be created.
/// Returns an empty vector if the file is empty or new.
fn read_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TASKS_FILE)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.is_empty() {
        Ok(Vec::new())
    } else {
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }
}

/// Writes the list of tasks to the tasks file.
/// This will overwrite the existing file content.
fn write_tasks(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(TASKS_FILE)?;
    let json = serde_json::to_string_pretty(tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments using clap
    let cli = Cli::parse();

    // Load existing tasks from the JSON file
    let mut tasks = read_tasks()?;

    // Handle different CLI subcommands
    match &cli.command {
        Commands::Add { description } => {
            // Generate a new unique ID by finding the maximum existing ID and adding 1
            let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

            // Create a new task with the provided description
            let new_task = Task {
                id: new_id,
                description: description.clone(),
                completed: false,
            };

            // Add the new task to the list and save to file
            tasks.push(new_task);
            write_tasks(&tasks)?;
            println!("Added task with id {}", new_id);
        }
        Commands::List => {
            // Display all tasks with their completion status
            if tasks.is_empty() {
                println!("No tasks yet.");
            } else {
                for task in tasks {
                    println!(
                        "[{}] {} - {}",
                        if task.completed { "x" } else { " " },
                        task.id,
                        task.description
                    );
                }
            }
        }
        Commands::Delete { id } => {
            // Store the initial length to check if a task was actually deleted
            let initial_len = tasks.len();

            // Remove the task with the specified ID from the list
            tasks.retain(|task| task.id != *id);

            // Check if a task was actually found and deleted
            if tasks.len() < initial_len {
                // Save the updated task list to file
                write_tasks(&tasks)?;
                println!("Deleted task with id {}", id);
            } else {
                println!("Task with id {} not found.", id);
            }
        }
    }

    Ok(())
}
