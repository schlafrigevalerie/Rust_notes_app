mod db;

use db::{init_db, add_note, get_notes};
use std::io::{self, Write};
use rusqlite::Result;

fn main() -> Result<()> {
    let conn = init_db()?;

    loop {
        println!("Выберите действие: (1) Добавить заметку, (2) Показать заметки, (0) Выйти");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Название: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                print!("Содержание: ");
                io::stdout().flush().unwrap();
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();

                add_note(&conn, title.trim(), content.trim())?;
            }
            "2" => {
                get_notes(&conn)?;
            }
            "0" => {
                println!("Выход...");
                break;
            }
            _ => {
                println!("Неверный ввод, попробуйте снова!");
            }
        }
    }

    Ok(())
}
