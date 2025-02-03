use anyhow::Ok;
use anyhow::Result;
use clap::ArgMatches;
use rusqlite::params;
use rusqlite::Connection;

pub struct CommandHandler(Connection);

impl CommandHandler {
    pub fn new(conn: Connection) -> Self {
        CommandHandler(conn)
    }

    pub fn add(&self, matches: &ArgMatches) -> Result<()> {
        let task = matches
            .get_one::<String>("TASK")
            .expect("TASK argument is required");
        let due_date = matches.get_one::<String>("due");

        match due_date {
            Some(due) => {
                self.0.execute(
                    "INSERT INTO tasks (title, due_time) VALUES (?1, ?2)",
                    params![task, due],
                )?;
            }
            None => {
                self.0
                    .execute("INSERT INTO tasks (title) VALUES (?1)", params![task])?;
            }
        }

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
