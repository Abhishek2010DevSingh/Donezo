use donezo::command::get_command;

fn main() -> anyhow::Result<()> {
    let command = get_command();
    println!("{:?}", command.get_matches());
    Ok(())
}
