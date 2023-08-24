#![feature(allocator_api)]

mod child_main;
mod controls;
mod utils;
mod rainbow;

use controls::App;
use rainbow::rainbow;

pub fn main() { 
    let mut app: App<iced::keyboard::KeyCode, iced_widget::renderer::Renderer<iced::Theme>>  = App::new();
    let widget = rainbow();
    app.add_window(widget);
    loop {
        app.draw(None::<Vec<_>>)
    }
}
