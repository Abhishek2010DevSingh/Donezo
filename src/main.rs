use donezo::{command::get_command, command_handler::CommandHandler, db::conn};

fn main() -> anyhow::Result<()> {
    let command_handler = CommandHandler::new(conn()?)?;
    command_handler.run(&get_command().get_matches())?;
    Ok(())
}
