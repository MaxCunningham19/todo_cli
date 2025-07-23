use std::process::exit;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use todo::{read, store, Item, List, Progress, Status};

#[derive(Subcommand, Debug, PartialEq, Clone)]
enum UpdateCommand {
    /// Update the task's due date.
    Date {
        #[arg(help = "New due date in dd-mm-yyyy format",
        value_parser = parse_date)]
        date: NaiveDate,
    },
    /// Update the task description.
    Desc {
        #[arg(help = "New description")]
        desc: String,
    },
    /// Update the progress (0.0 to 1.0).
    Progress {
        #[arg(help = "New progress as a float between 0.0 and 1.0",
    value_parser = parse_progress)]
        progress: Progress,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
enum Command {
    /// Add a new item.
    Add {
        #[arg(help = "Task description")]
        task: String,

        #[arg(
            help = "Due date (dd-mm-yyyy)",
            value_parser = parse_date
        )]
        date: Option<NaiveDate>,

        #[arg(help = "Progress (0.0 to 1.0)",
    value_parser = parse_progress)]
        progress: Option<Progress>,
    },

    /// Remove specific tasks by index.
    Remove {
        #[arg(help = "One or more task indices to remove")]
        indices: Vec<usize>,
    },

    /// Mark one or more tasks as done.
    Done {
        #[arg(help = "One or more task indices to mark as done")]
        indices: Vec<usize>,
    },

    /// Mark one or more tasks as TODO.
    Undo {
       #[arg(help = "One or more task indices to mark as TODO")]
        indices: Vec<usize>, 
    },

    /// Update a field of a task.
    Update {
        #[arg(help = "Index of the task to update")]
        index: usize,

        #[command(subcommand)]
        value: UpdateCommand,
    },

    /// List all tasks.
    List,

    /// Clear all completed tasks.
    Clear,
}

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A simple and effective todo CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

const DATE_FORMAT: &str = "%d-%m-%Y";
const DEFAULT_DB_PATH: &str = "./db.json";

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, DATE_FORMAT).map_err(|e| format!("Invalid date format: {}", e))
}

fn parse_progress(v: &str) -> Result<Progress, String> {
    match v.parse::<f64>() {
        Ok(v) => Progress::new(v),
        Err(_) => Err("Value was not a valid decimal between 0.0 and 1.0".into()),
    }
}

fn format_list(list: &List) {
    for item in list.list() {
        println!("{:?} {:?} {:?}", item.deadline(), item.desc(), item.progress());
    }
}
fn main() {
    let cli = Cli::parse();

    let mut todo_list = read(DEFAULT_DB_PATH).unwrap_or_else(|err| {eprintln!("Could not read list from location. Err: {}",err); exit(1)});

    match &cli.command {
        Command::Add {
            task,
            date,
            progress,
        } => {
            println!("Adding task {}", task);
            let mut item = Item::new(task.to_string());
            if let Some(date) = date {
                item.set_deadline(*date);
            }
            if let Some(progress) = progress {
                item.set_progress(progress.clone());
            }
            todo_list.add(item);
        }

        Command::Remove { indices } => {
            for index in indices {
                if let Some(_) = todo_list.remove(index - 1) {
                    println!("Removing task {:?}", index);
                } else {
                    eprintln!(
                        "Failed to remove task {:?} due to invalid index location.",
                        index
                    )
                }
            }
        }

        Command::Done { indices } => {
            for index in indices {
                match todo_list.mutate_index(index-1, |item| {
                    item.set_progress(Progress::one());
                }) {
                    Ok(_) => println!("Marking task {:?} as complete", index),
                    Err(e) => eprintln!("{}",e),
                }
            }
        }
    
        Command::Undo { indices } => {
            for index in indices {
                match todo_list.mutate_index(index-1, |item| {
                    item.set_progress(Progress::zero());
                }) {
                    Ok(_) => println!("Marking task {:?} as complete", index),
                    Err(e) => eprintln!("{}",e),
                }
            }
        }


        Command::List => {
            println!("Printing List");
            format_list(&todo_list);
        }

        Command::Clear => {
            println!("Clearing completed tasks");
            todo_list.mut_list().retain(|item| item.status() != &Status::Complete)
        }

        Command::Update { index, value } => {
            let index = index - 1;
            match value {
                UpdateCommand::Date { date } => todo_list.mutate_index(index, |item| {
                    item.set_deadline(*date);
                }),
                UpdateCommand::Desc { desc } => todo_list.mutate_index(index, |item| {
                    item.set_desc(desc.to_string());
                }),
                UpdateCommand::Progress { progress } => todo_list.mutate_index(index, |item| {
                    item.set_progress(progress.clone());
                }),
            }
            .unwrap_or_else(|err| eprint!("{}", err));
        }
    };

    store(&todo_list, DEFAULT_DB_PATH).unwrap_or_else(|err| {eprintln!("Could not save list. Err: {}",err); exit(1)});
    if cli.command != Command::List {
        format_list(&todo_list);
    }
}
