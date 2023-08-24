use std::sync::Mutex;

use iced_wgpu::core::{Element, Widget};
use iced_winit::settings::Window;
use iced_winit::winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Backend, Renderer, Settings};
use iced_widget::runtime::{self, program, Command, Debug};
use iced_winit::core::Size;
use iced_winit::{futures, winit};

use crate::controls::SharedWidget;
use crate::utils::SharedMemoryAllocator;

pub struct Program<M, R> {
    widget: SharedWidget<M, R>,
}

impl<M, R> Program<M, R> {
    pub fn new(widget: SharedWidget<M, R>) -> Self {
        Self { widget }
    }
}

impl<M, R> runtime::Program for Program<M, R> {
    type Renderer = Renderer<()>;
    type Message = ();

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Self::Renderer> {
        todo!()
    }
}

// TODO: с одной стороны фреймверк елм подобный с другой, так как это другой поток я поставил
// для надежности мьютекс, не помешает разобраться потом 
pub fn child_main<M, R>(widget: SharedWidget<M, R>) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).expect("Can't create winit window");

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let surface = unsafe { instance.create_surface(&window) }.expect("Can't create surface");

    let (format, (device, queue)) = futures::futures::executor::block_on(async {
        let adapter = wgpu::util::initialize_adapter_from_env_or_default(
            &instance,
            wgpu::Backends::PRIMARY,
            Some(&surface),
        )
        .await
        .expect("Create adapter");

        let adapter_features = adapter.features();
        let needed_limits = wgpu::Limits::default();
        let capabilities = surface.get_capabilities(&adapter);

        (
            capabilities
                .formats
                .iter()
                .copied()
                .find(wgpu::TextureFormat::is_srgb)
                .or_else(|| capabilities.formats.first().copied())
                .expect("Get preferred format"),
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        features: adapter_features & wgpu::Features::default(),
                        limits: needed_limits,
                    },
                    None,
                )
                .await
                .expect("Request device"),
        )
    });

    let physical_size = window.inner_size();
    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        },
    );

    // Initialize iced
    let mut debug = Debug::new();
    let mut renderer: iced_graphics::Renderer<iced_wgpu::Backend, iced::Theme> = Renderer::new(Backend::new(&device, &queue, Settings::default(), format));

    let mut viewport = Viewport::with_physical_size(
        Size::new(physical_size.width, physical_size.height),
        window.scale_factor(),
    );

    let scene: Program<M, R> = Program::new(widget);
    // let mut state = program::State::new(scene, viewport.logical_size(), &mut renderer, &mut debug);

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { window_id, event } => {
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                   _ => {},
                }
            },
            _ => {},
        }
    });
}
