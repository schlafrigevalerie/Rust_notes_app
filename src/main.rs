use cursive::CursiveExt;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, TextView};
use cursive::Cursive;
use cursive::theme::{Color, Palette, PaletteColor, Theme};
use std::sync::{Arc, Mutex};
use rusqlite::{Connection};

mod db;

fn main() {
    let mut siv = Cursive::new();
    
    // Настройка темы с цветом фона
    siv.set_theme(Theme {
        shadow: false,
        palette: {
            let mut palette = Palette::default();
            // Устанавливаем бледно-розовый фон, используя PaletteColor::Background
            palette[PaletteColor::Background] = Color::Rgb(255, 182, 193); // Бледно-розовый
            palette
        },
        ..Default::default() // Используем стандартные значения для других параметров
    });

    let conn = Arc::new(Mutex::new(db::init_db().unwrap()));

    siv.add_layer(
        Dialog::new()
            .title("Главное меню")
            .content(TextView::new("Добро пожаловать в приложение для заметок!"))
            .button("Создать заметку", {
                let conn = conn.clone();
                move |siv| {
                    add_note_screen(siv, conn.clone());
                }
            })
            .button("Посмотреть заметки", {
                let conn = conn.clone(); 
                move |siv| {
                    show_notes_screen(siv, conn.clone());
                }
            })
            .button("Выход", |siv| siv.quit()),
    );

    siv.run();
}

fn add_note_screen(siv: &mut Cursive, conn: Arc<Mutex<Connection>>) {
    siv.add_layer(
        Dialog::new()
            .title("Введите заголовок заметки")
            .content(EditView::new().on_submit({
                let conn = conn.clone(); 
                move |siv, title| {
                    let title = title.to_string(); 
                    siv.add_layer(
                        Dialog::new()
                            .title("Введите текст заметки")
                            .content(EditView::new().on_submit({
                                let conn = conn.clone(); 
                                move |siv, content| {
                                    let content = content.to_string(); 
                                    
                                    let conn = conn.lock().unwrap();
                                    db::add_note(&conn, &title, &content).expect("Не удалось сохранить заметку");

                                    siv.add_layer(
                                        Dialog::new()
                                            .title("Заметка сохранена")
                                            .content(TextView::new("Заметка была успешно сохранена!"))
                                            .button("Ок", |siv| {
                                                siv.quit(); 
                                            }),
                                    );
                                }
                            }))
                            .button("Отмена", |siv| {
                                siv.quit(); 
                            }),
                    );
                }
            }))
            .button("Отмена", |siv| {
                siv.quit(); 
            }),
    );
}

fn show_notes_screen(siv: &mut Cursive, conn: Arc<Mutex<Connection>>) {
    let conn = conn.lock().unwrap();
    let mut notes = Vec::new();

    let mut stmt = conn.prepare("SELECT id, title, content FROM notes").unwrap();
    let notes_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?, 
            row.get::<_, String>(1)?,  
            row.get::<_, String>(2)?,  
        ))
    }).unwrap();

    for note in notes_iter {
        let (id, title, content) = note.unwrap();
        notes.push(format!("#{} [{}] - {}\n{}", id, title, content, content)); 
    }

    let content = if notes.is_empty() {
        "Нет заметок".to_string()
    } else {
        notes.join("\n")
    };

    siv.add_layer(
        Dialog::new()
            .title("Список заметок")
            .content(TextView::new(content))
            .button("Назад", |siv| {
                siv.quit(); 
            }),
    );
}
