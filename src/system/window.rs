use num_format::Locale::{fa, se};
use crate::system::ini::{WindowIniData, WindowMode};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{WindowBuilder, Window}};
use winit::dpi::{LogicalSize, Size};
use crate::system::ini::WindowMode::Fullscreen;
use crate::system::vk_renderer;

pub struct ResaWindow {
    width: u32,
    height: u32,
    title: String,
    mode: WindowMode,
    window: Option<Window>,
    event_loop: EventLoop<()>,
}

impl ResaWindow {
    pub fn init(ini_data: &WindowIniData) -> ResaWindow {
        let instance = ResaWindow {
            width: ini_data.window_width,
            height: ini_data.window_height,
            title: ini_data.window_title.to_string(),
            mode: ini_data.window_mode,
            window: None,
            event_loop: EventLoop::new(),
        };

        instance
    }

    pub fn build_window(&mut self) {
        let windowSize = LogicalSize::new(self.width, self.height);
        let windowMode = match self.mode {
            WindowMode::Windowed => { None }
            WindowMode::Fullscreen => { None }
        };

        self.window = Some(WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(windowSize)
            .with_fullscreen(windowMode)
            .with_resizable(true)
            .with_always_on_top(true)
            .with_transparent(false)
            .build(&self.event_loop).unwrap());

        vk_renderer::init();
    }

    pub fn run_window_loop(self) {
        let win = self.window.unwrap();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested, ..
                } => {
                    *control_flow = ControlFlow::Exit
                }
                Event::MainEventsCleared => {
                    win.request_redraw();
                }
                Event::RedrawRequested(_) => {}
                _ => ()
            }
        });
    }
}