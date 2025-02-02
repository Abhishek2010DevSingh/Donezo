use donezo::{
    command::get_command,
    db::{conn, migration},
};

fn main() -> anyhow::Result<()> {
    let conn = conn()?;
    migration(&conn)?;

    let command = get_command();
    println!("{:?}", command.get_matches());
    Ok(())
}
