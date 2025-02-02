use donezo::db::conn;

fn main() -> anyhow::Result<()> {
    let conn = conn()?;
    let value = conn.execute("Select 1 + 1;", ())?;
    println!("{}", value);
    Ok(())
}
