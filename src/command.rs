use clap::{arg, Command};

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

pub fn get_command() -> Command {
    Command::new("Denezo")
        .bin_name("denezo")
        .subcommand_required(true)
        .author("Abhishek2010DevSingh")
        .about("Denezo is a simple command-line to-do list manager built with Rust. It allows users to manage tasks by adding, listing, completing, and deleting tasks from a SQLite database.")
        .styles(CLAP_STYLING)
        .subcommand(
            Command::new("add")
                .about("Add a new to-do item")
                .arg(arg!(<TASK> "The task description").required(true))
                .arg(
                    arg!(
                        -d --due <DATE> "Sets a due date for the task in YYYY-MM-DD format"
                    )
                    .required(false),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List all to-do items")
                .arg(arg!(-a --all "Show all tasks, including completed ones").required(false)),
        )
        .subcommand(
            Command::new("complete")
                .about("Mark a to-do item as completed")
                .arg(arg!(<ID> "The ID of the task to mark as complete").required(true)),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a to-do item")
                .arg(arg!(<ID> "The ID of the task to delete").required(true)),
        )
}
