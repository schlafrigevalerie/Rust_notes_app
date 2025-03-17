use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView};
use cursive::traits::*;
use cursive::traits::Nameable;
use cursive::style::{Color, ColorStyle};
use cursive::view::Resizable;
use cursive::views::Canvas;
use cursive::Printer;

fn main() {
    let mut siv = cursive::default();
    siv.load_theme_file("theme.toml").unwrap();


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
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("name")
            .fixed_width(10))
        .title("Enter a task")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
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



// Gradient for the front color
fn front_color(x: u8, y: u8, x_max: u8, y_max: u8) -> Color {
    // We return a full 24-bits RGB color, but some backends
    // will project it to a 256-colors palette.
    Color::Rgb(
        x * (255 / x_max),
        y * (255 / y_max),
        (x + 2 * y) * (255 / (x_max + 2 * y_max)),
    )
}

// Gradient for the background color
fn back_color(x: u8, y: u8, x_max: u8, y_max: u8) -> Color {
    // Let's try to have a gradient in a different direction than the front color.
    Color::Rgb(
        128 + (2 * y_max + x - 2 * y) * (128 / (x_max + 2 * y_max)),
        255 - y * (255 / y_max),
        255 - x * (255 / x_max),
    )
}

fn draw(_: &(), p: &Printer) {
    // We use the view size to calibrate the color
    let x_max = p.size.x as u8;
    let y_max = p.size.y as u8;

    // Print each cell individually
    for x in 0..x_max {
        for y in 0..y_max {
            // We'll use a different style for each cell
            let style = ColorStyle::new(
                front_color(x, y, x_max, y_max),
                back_color(x, y, x_max, y_max),
            );

            p.with_color(style, |printer| {
                printer.print((x, y), "+");
            });
        }
    }
}