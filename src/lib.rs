use std::error::Error;

use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand, ValueEnum};
use redis::{Client, Commands};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Command {
    #[clap(subcommand)]
    action: ActionType,
}

pub fn run(cmd: Command) -> Result<(), Box<dyn Error>> {
    match cmd.action {
        ActionType::AddItem(add_item) => save_new_item(add_item),
        _ => {
            println!("Not Implemented yet");
            Ok(())
        }
    }
}

fn save_new_item(add_item: AddItemCommand) -> Result<(), Box<dyn Error>> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_connection()?;

    let id: u32 = conn.incr("task:counter", 1)?;

    let item = ToDoItem {
        id,
        title: add_item.title,
        description: add_item.description,
        priority: add_item.priority,
        status: Status::ToDo,
    };

    add_item_to_redis(conn, item)?;

    Ok(())
}

fn add_item_to_redis(mut conn: redis::Connection, item: ToDoItem) -> Result<(), Box<dyn Error>> {
    let _: () = conn.hset(format!("task:{}", item.id), "id", item.id)?;
    let _: () = conn.hset(format!("task:{}", item.id), "title", item.title)?;
    let _: () = conn.hset(format!("task:{}", item.id), "description", item.description)?;
    let priority: &'static str = item.priority.into();
    let _: () = conn.hset(format!("task:{}", item.id), "priority", priority)?;
    let status: &'static str = item.status.into();
    let _: () = conn.hset(format!("task:{}", item.id), "status", status)?;
    Ok(())
}

#[derive(Debug, Subcommand)]
enum ActionType {
    /// Adds a new todo item to the task manager.
    AddItem(AddItemCommand),
    /// Changes the status of a task by its ID.
    ChangeStatus(ChangeStatusCommand),
    /// Lists todo items, optionally filtering by priority or keyword.
    List(ListCommand),
    /// Displays detailed information about a single task by its ID.
    Show(ShowCommand),
    /// Deletes a task from the task manager by its ID.
    DeleteItem(DeleteItemCommand),
    /// Displays a summary of tasks grouped by status and priority.
    Summary(SummaryCommand),
}

#[derive(Debug, Args)]
struct AddItemCommand {
    /// The title of the todo item. (Required)
    title: String,
    /// A short description of the task. (Required)
    description: String,
    /// The priority level of the task. (Optional)
    #[clap(value_enum, default_value_t=Priority::Low)]
    priority: Priority,
}

#[derive(Debug, Args)]
struct ChangeStatusCommand {
    /// The unique ID of the task to update. (Required)
    id: u32,
    /// The new status to assign to the task. (Required)
    status: Status,
}

#[derive(Debug, Args)]
struct ListCommand {
    /// Filter tasks by priority.
    #[arg(short, long)]
    priority: Option<Priority>,
    /// Filter tasks containing specific keywords in title or description.
    #[arg(short, long)]
    keywords: Option<String>,
    /// Show tasks created on or after this date (format: YYYY-MM-DD).
    #[arg(long)]
    from: Option<NaiveDate>,
    /// Show tasks created on or before this date (format: YYYY-MM-DD).
    #[arg(long)]
    to: Option<NaiveDate>,
}

#[derive(Debug, Args)]
struct ShowCommand {
    /// The unique ID of the task. (Required)
    id: u32,
}

#[derive(Debug, Args)]
struct DeleteItemCommand {
    /// The unique ID of the task to delete. (Required)
    id: u32,
}

#[derive(Debug, Args)]
struct SummaryCommand {
    /// Output summary in JSON format.
    #[arg(long)]
    json: bool,
}

struct ToDoItem {
    id: u32,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
}

#[derive(ValueEnum, Clone, Debug, strum::IntoStaticStr)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, ValueEnum, Clone, strum::IntoStaticStr)]
enum Status {
    ToDo,
    InProgress,
    Done,
}
