# RustyTasks

A simple, fast, and efficient command-line task manager built with Rust. This project demonstrates core Rust concepts in a practical, real-world application, making it an ideal piece for a developer's portfolio.

## Project Overview

- Developed a cross-platform task manager in Rust using `clap`, `serde`, and file-based JSON storage.
- Implemented features for adding, listing, and deleting tasks directly from the terminal.
- Demonstrated understanding of borrowing, traits, match expressions, and error handling in a real-world CLI scenario.

## Features

- **Add new tasks**: Quickly add tasks to your to-do list.
- **List all tasks**: View all your pending tasks with their IDs.
- **Delete tasks**: Remove tasks by their ID once you've completed them.
- **Data Persistence**: Tasks are saved to a `tasks.json` file, so your data is safe between sessions.

## Tech Stack

- **[Rust](https://www.rust-lang.org/)**: The core programming language.
- **[clap](https://crates.io/crates/clap)**: A powerful library for parsing command-line arguments.
- **[serde](https://crates.io/crates/serde)**: A framework for serializing and deserializing Rust data structures efficiently, used here for JSON handling.

## Getting Started

Follow these instructions to get a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Make sure you have Rust and its package manager, Cargo, installed on your system. You can install them by following the official instructions at [rustup.rs](https://rustup.rs/).

### Building the Project

1.  Clone the repository to your local machine:
    ```sh
    git clone https://github.com/your-username/rustytasks.git
    cd rustytasks
    ```
    *(Note: Remember to replace `your-username` with your actual GitHub username once you've pushed the repository.)*

2.  Build the project in release mode for an optimized executable:
    ```sh
    cargo build --release
    ```
    The executable will be available at `target/release/rustytasks.exe` on Windows or `target/release/rustytasks` on Linux/macOS.

## How to Test the Application

You can test the application by running the following commands from your terminal within the project directory.

*(On Windows, use `.\\target\\release\\rustytasks.exe`. On Linux/macOS, use `./target/release/rustytasks`)*

### 1. Check the Help Message

Get an overview of all available commands and options.

```sh
.\\target\\release\\rustytasks.exe --help
```

### 2. Add a Task

Add a new task to your list. The description should be enclosed in quotes if it contains spaces.

```sh
.\\target\\release\\rustytasks.exe add "Learn more about Rust"
```

### 3. List Tasks

View all the tasks you have added.

```sh
.\\target\\release\\rustytasks.exe list
```
*Expected Output:*
```
[ ] 1 - Learn more about Rust
```

### 4. Add Another Task

```sh
.\\target\\release\\rustytasks.exe add "Build another cool project"
```

### 5. List Tasks Again

```sh
.\\target\\release\\rustytasks.exe list
```
*Expected Output:*
```
[ ] 1 - Learn more about Rust
[ ] 2 - Build another cool project
```

### 6. Delete a Task

Delete a task by providing its unique ID.

```sh
.\\target\\release\\rustytasks.exe delete 1
```

### 7. Confirm Deletion

List the tasks one last time to confirm the task was successfully deleted.

```sh
.\\target\\release\\rustytasks.exe list
```
*Expected Output:*
```
[ ] 2 - Build another cool project
``` 