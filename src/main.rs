use cursive::Cursive;
use cursive::views::{Dialog, EditView, TextView};
use cursive::view::Resizable;  // Добавляем импорт для Resizable
use cursive::CursiveExt;  // Для вызова run

fn main() {
    let mut siv = Cursive::new();

    // Экран для ввода текста
    siv.add_layer(
        Dialog::new()
            .title("Введите заметку")
            .content(
                EditView::new()
                    .on_submit(|siv, text| {
                        // Когда нажимаем Enter, переходим к экрану с заметкой
                        siv.add_layer(
                            Dialog::new()
                                .title("Ваша заметка")
                                .content(TextView::new(text))
                                .button("Назад", |siv| {
                                    siv.pop_layer();  // Возвращаемся к экрану ввода
                                }),
                        );
                    })
                    .max_width(20),  // Можно использовать max_width вместо fixed_width
            )
            .button("Выход", |siv| {
                siv.quit();
            }),
    );

    // Запускаем приложение
    siv.run();
}
