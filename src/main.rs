use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Command {
    /// Add a new item.
    Add {
        /// The task description.
        task: String,
    },
    /// Remove a specific task.
    Remove {
        /// The index of the task.
        index: Vec<usize>,
    },
    /// Mark a task as done.
    Done {
        /// The index of the task.
        index: Vec<usize>,
    },
    /// List all tasks.
    List,
    /// Clear all done tasks.
    Clear,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A todo CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add { task } => {
            println!("Adding task {}", task);
        }
        Command::Remove { index } => {
            println!("Removing {:?}th task", index);
        }
        Command::Done { index } => {
            println!("Marking {:?}th task as complete", index);
        }
        Command::List => {
            println!("Printing List");
        }
        Command::Clear => {
            println!("Clearing completed tasks");
        }
    }
}
