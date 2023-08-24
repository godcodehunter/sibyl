#![feature(allocator_api)]

mod child_main;
mod controls;
mod utils;
mod rainbow;

use controls::App;
use iced_graphics::core::Widget;
use rainbow::rainbow;
use utils::SharedMemoryAllocator;
use std::sync::{Mutex, Arc};

pub fn main() { 
    let mut app: App<iced::keyboard::KeyCode, iced_widget::renderer::Renderer<iced::Theme>>  = App::new();
    let widget = rainbow(); 
    let alloc: Box<dyn Widget<_, _>, SharedMemoryAllocator> = Box::new_in(widget, SharedMemoryAllocator);
    let mw = Arc::new(Mutex::new(alloc));
    app.add_window(mw);
    let widget = rainbow(); 
    let alloc: Box<dyn Widget<_, _>, SharedMemoryAllocator> = Box::new_in(widget, SharedMemoryAllocator);
    let mw = Arc::new(Mutex::new(alloc));
    app.add_window(mw);
    loop {
        app.draw(None::<Vec<_>>)
    }
}
