use cursive::views::{Dialog, TextView, EditView};
use cursive::Cursive;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
mod db; 
use cursive::view::Resizable; 
use cursive::CursiveExt; 

fn main() {
    let mut siv = Cursive::new();  


    let conn = Arc::new(Mutex::new(
        db::init_db().expect("Не удалось открыть базу данных"),
    ));


    siv.add_layer(
        Dialog::new()
            .title("Меню заметок")
            .content(TextView::new("Выберите действие:"))
            .button("Добавить заметку", {
                let conn = Arc::clone(&conn); 
                move |siv| {
                    siv.add_layer(
                        Dialog::new()
                            .title("Новая заметка")
                            .content(
                                EditView::new()
                                    .on_submit({
                                        let conn = Arc::clone(&conn);  
                                        move |siv, text| {
                              
                                            let parts: Vec<&str> = text.splitn(2, ',').collect();
                                            if parts.len() == 2 {
                                                let title = parts[0].trim().to_string();
                                                let content = parts[1].trim().to_string();

                                                db::add_note(&conn.lock().unwrap(), &title, &content)
                                                    .expect("Не удалось добавить заметку");

                                                show_notes(siv, Arc::clone(&conn));
                                            } else {
                                                siv.add_layer(Dialog::new().content(TextView::new("Неверный формат! Используйте запятую для разделения заголовка и содержания")).button("ОК", |siv| { siv.quit(); }));
                                            }
                                        }
                                    })
                                    .max_width(30) 
                            )
                            .button("Отмена", |siv| { siv.quit(); })
                    );
                }
            })
            .button("Показать заметки", {
                let conn = Arc::clone(&conn);  
                move |siv| {
                    show_notes(siv, Arc::clone(&conn)); 
                }
            })
            .button("Выход", |siv| {
                siv.quit();
            })
    );


    siv.run();
}


fn show_notes(siv: &mut Cursive, conn: Arc<Mutex<Connection>>) {
    let conn = conn.lock().unwrap();
    db::get_notes(&conn).expect("Не удалось получить заметки");


    let mut content = String::new();
    let mut binding = conn.prepare("SELECT title, content FROM notes").unwrap(); 
    let notes_iter = binding.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?
        ))
    }).unwrap();

    let mut is_empty = true;
    for note in notes_iter {
        let (title, content_text) = note.unwrap();
        content.push_str(&format!("Title: {}\nContent: {}\n\n", title, content_text));
        is_empty = false;
    }

    if is_empty {
        content = "Нет заметок.".to_string();
    }

    siv.add_layer(
        Dialog::new()
            .title("Список заметок")
            .content(TextView::new(content))
            .button("Закрыть", |siv| { siv.quit(); })
    );
}


