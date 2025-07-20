use clap::{Parser, Subcommand};
use todo::TodoList;

const DEFAULT_DB_PATH: &str = "./db.json";

#[derive(Subcommand, Debug, PartialEq, Eq)]
enum Command {
    /// Add a new item.
    Add {
        /// The task description.
        tasks: Vec<String>,
    },
    /// Remove a specific task.
    Remove {
        /// The index of the task.
        indexs: Vec<usize>,
    },
    /// Mark a task as done.
    Done {
        /// The index of the task.
        indexs: Vec<usize>,
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

fn format_list(todo_list: &TodoList) -> String {
    let mut res = String::new();
    for (idx, item) in todo_list.items().iter().enumerate() {
        res.push_str(&format!(
            "{}. - {}  [{}]\n",
            idx + 1,
            item.desc,
            if item.is_done() { "x" } else { " " }
        ));
    }
    return res;
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list = TodoList::new();
    todo_list.load(DEFAULT_DB_PATH);
    match &cli.command {
        Command::Add { tasks } => {
            for task in tasks {
                println!("Adding task {}", task);
                todo_list.add(task.to_string());
            }
        }
        Command::Remove { indexs } => {
            for index in indexs {
                if let Ok(_) = todo_list.remove(index - 1) {
                    println!("Removing task {:?}", index);
                } else {
                    println!("Failed to remove task {:?} due to invalid index location.", index)
                }
            }
        }
        Command::Done { indexs } => {
            for index in indexs {
                todo_list.mark_as_done(index - 1);
                println!("Marking task {:?} as complete", index);
            }
        }
        Command::List => {
            println!("Printing List");
            print!("{}", format_list(&todo_list));
        }
        Command::Clear => {
            todo_list.clear_done_tasks();
            println!("Clearing completed tasks");
        }
    };
    todo_list.save(DEFAULT_DB_PATH);
    if cli.command != Command::List {
        print!("{}", format_list(&todo_list));
    }
}
