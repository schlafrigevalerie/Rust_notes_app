use rusqlite::{params, Connection, Result};

fn init_db() -> Result<Connection> {
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}

fn main() -> Result<()> {
    let conn = init_db()?;
    println!("База данных инициализирована!");
    Ok(())
}


