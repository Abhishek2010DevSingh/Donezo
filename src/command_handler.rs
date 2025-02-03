use anyhow::{Context, Result};
use clap::ArgMatches;
use rusqlite::{named_params, params, Connection};
use tabled::Table;

use crate::{db::migration, model::Task};

pub struct CommandHandler(Connection);

impl CommandHandler {
    pub fn new(conn: Connection) -> Result<Self> {
        migration(&conn)?;
        Ok(CommandHandler(conn))
    }

    pub fn add(&self, matches: &ArgMatches) -> Result<()> {
        let task = matches
            .get_one::<String>("TASK")
            .context("TASK argument is required")?;
        let due_date = matches.get_one::<String>("due");

        let mut stmt = self
            .0
            .prepare("SELECT id FROM tasks WHERE title = ?1")
            .context("Failed to prepare statement for checking existing task")?;

        let exists = stmt
            .exists(params![task])
            .context("Failed to check if task exists")?;

        if exists {
            anyhow::bail!("A task with the title '{}' already exists.", task);
        }

        let query = match due_date {
            Some(_) => "INSERT INTO tasks (title, due_time) VALUES (:title, :due_time)",
            None => "INSERT INTO tasks (title) VALUES (:title)",
        };

        let mut stmt = self
            .0
            .prepare(query)
            .context("Failed to prepare task insertion statement")?;

        match due_date {
            Some(due) => stmt
                .execute(named_params! {":title": task, ":due_time": due})
                .context("Failed to insert task with due date")?,
            None => stmt
                .execute(named_params! {":title": task})
                .context("Failed to insert task")?,
        };

        let mut stmt = self
            .0
            .prepare("SELECT id, title, due_time FROM tasks WHERE title = ?1")
            .context("Failed to prepare statement to retrieve inserted task")?;

        let task: Task = stmt
            .query_row(params![task], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    due_time: crate::model::DisplayOption(row.get(2)?),
                    done: false,
                    created_at: "Just now".to_string(),
                })
            })
            .context("Failed to retrieve inserted task")?;

        println!("{}", Table::new(vec![task]));

        Ok(())
    }

    pub fn run(&self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("add", sub_matches)) => self.add(sub_matches)?,
            //Some(("list", _)) => self.list()?,
            //Some(("complete", sub_matches)) => self.complete(sub_matches)?,
            //Some(("delete", sub_matches)) => self.delete(sub_matches)?,
            _ => eprintln!("Unknown command"),
        }
        Ok(())
    }
}
