use rusqlite::{params, Connection, Result};

pub fn init_db() -> Result<Connection> {
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

pub fn add_note(conn: &Connection, title: &str, content: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        params![title, content],
    )?;
    println!("Заметка добавлена!");
    Ok(())
}

pub fn get_notes(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, title, content, created_at FROM notes")?;
    let notes_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,  // id
            row.get::<_, String>(1)?,  // title
            row.get::<_, String>(2)?,  // content
            row.get::<_, String>(3)?,  // created_at
        ))
    })?;

    for note in notes_iter {
        let (id, title, content, created_at) = note?;
        println!("#{} [{}] - {}\n{}\n", id, created_at, title, content);
    }
    Ok(())
}
