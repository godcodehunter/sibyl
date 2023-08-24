// use iced_wgpu::core::Widget;
// use iced_wgpu::Renderer;
// use iced_widget::{slider, text_input, Column, Row, Text};
// use iced_winit::core::{Alignment, Color, Element, Length};
// use iced_winit::runtime::{Command, Program};
// use iced_winit::style::Theme;

// pub trait StyleSheet {
//     type Style: Default;
// }

// enum Selection {
//     Vertical {
//         start: usize,
//         end: usize,
//     },
//     Horizontal {
//         start: f32,
//         end: f32,
//     },
//     Zone {
//         v_s: usize,
//         v_e: usize,
//         h_s: f32,
//         h_e: f32,
//     },
// }

// // pub struct Chart<Renderer>
// // where
// //     Renderer: iced_wgpu::core::Renderer,
// //     Renderer::Theme: StyleSheet,
// // {
// //     smooth_filling: bool,
// //     grid: bool,
// //     scrolling: bool,
// //     scaling: bool,
// //     minimap: bool,
// //     magnifier: bool,
// //     selections: Vec<Selection>,
// //     data: Vec<f32>,
// //     // размер x, y
// //     style: <Renderer::Theme as StyleSheet>::Style,
// // }

// // impl<Message, Renderer> Widget<Message, Renderer> for Chart<Renderer>
// // where
// //     Renderer: iced_wgpu::core::Renderer,
// // {
// //     fn width(&self) -> Length {
// //         todo!()
// //     }

// //     fn height(&self) -> Length {
// //         todo!()
// //     }

// //     fn layout(
// //         &self,
// //         renderer: &Renderer,
// //         limits: &iced_wgpu::core::layout::Limits,
// //     ) -> iced_wgpu::core::layout::Node {
// //         todo!()
// //     }

// //     fn draw(
// //         &self,
// //         state: &iced_wgpu::core::widget::Tree,
// //         renderer: &mut Renderer,
// //         theme: &Renderer::Theme,
// //         style: &iced_wgpu::core::renderer::Style,
// //         layout: iced_wgpu::core::Layout<'_>,
// //         cursor_position: iced_wgpu::core::Point,
// //         viewport: &iced_wgpu::core::Rectangle,
// //     ) {
// //         renderer.fill_quad(quad, background)
// //     }
// // }

use std::collections::HashMap;

use iced_wgpu::core::{window, Element, Widget};

use iced_wgpu::{wgpu, Backend, Renderer, Settings};
use iced_winit::core::Size;
use iced_winit::runtime::program;
use iced_winit::runtime::Debug;
use iced_winit::{conversion, futures, winit, Clipboard};
use std::sync::Mutex;
use crate::utils::SharedMemoryAllocator;

use crate::utils;

struct Screen;

pub type Window<M, R> = Mutex<Box<dyn Widget<M, R>, SharedMemoryAllocator>>;

pub struct App<M, R> {
    // TODO: ...
    screens: Vec<Screen>,
    windows: HashMap<libc::pid_t, Window<M, R>>,
}

enum Action {
    // TODO: ...
    Eyes,
}

impl<M, R> App<M, R> 
where R: iced_wgpu::core::Renderer 
{
    pub fn new() -> Self {
        Self { screens: Default::default(), windows: Default::default() }
    }

    pub fn add_window<'a>(&mut self, widget: impl Widget<M, R> + 'a) {
        let alloc: Box<dyn Widget<M, R>, SharedMemoryAllocator> = Box::new_in(widget, SharedMemoryAllocator);
        let mw = Mutex::new(alloc);
        let tag = utils::fork().expect("Can't create process fork");
        
        match tag {
            utils::ProcessTag::ParentProcess(pid) => {
                self.windows.insert(pid, mw);
            }
            utils::ProcessTag::ChildProcess => {
                crate::child_main::child_main(mw);
            }
        }
    }

    pub fn draw(&mut self, action: Option<impl IntoIterator<Item = Action>>) {
        
    }
}
