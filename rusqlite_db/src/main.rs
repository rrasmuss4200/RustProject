use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("obc.db")?;

    conn.execute(
        "create table if not exists power_status (
            name text primary key,
            id integer
        )",
        [],
    )?;

    conn.execute(
        "create table if not exists txrx_status (
            name text primary key,
            id integer
        )",
        [],
    )?;

    Ok(())
}