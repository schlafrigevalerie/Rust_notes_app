use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView};
use cursive::traits::*;
use cursive::traits::Nameable;
use cursive::style::{Color, ColorStyle};
use cursive::view::Resizable;
use cursive::views::Canvas;
use cursive::Printer;
use cursive::views::Checkbox;



fn main() {

    
    let mut siv = cursive::default();
    siv.load_theme_file("theme.toml").unwrap();
    let checkbox = Checkbox::new().checked().with_name("check");


    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5));
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add task", add_name))
        .child(Button::new("Delete rask", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title("To-Do"));



    siv.run();
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str, checked: bool) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            let task_status = if checked { "[X] " } else { "[ ] " };
            view.add_item_str(format!("{}{}", task_status, name));
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(EditView::new()
                .on_submit(move |s, name| {
                    let checked = s.call_on_name("checkbox", |view: &mut Checkbox| view.is_checked()).unwrap();
                    ok(s, &name, checked);
                })
                .with_name("name")
                .fixed_width(20))
            .child(Checkbox::new()
                .with_name("checkbox"))
    )
    .title("Enter a task")
    .button("Ok", |s| {
        let name = s.call_on_name("name", |view: &mut EditView| {
            view.get_content()
        }).unwrap();
        let checked = s.call_on_name("checkbox", |view: &mut Checkbox| view.is_checked()).unwrap();
        ok(s, &name, checked);
    })
    .button("Cancel", |s| {
        s.pop_layer();
    }));
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit));
}


