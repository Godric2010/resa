use num_format::Locale::se;
use crate::system::ini::{WindowIniData, WindowMode};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{WindowBuilder, Window}};
use winit::dpi::{LogicalSize};
use crate::system::rendering::IRenderer;
use crate::system::rendering::mesh::mesh::Mesh;
use crate::system::rendering::mesh::vertex::Vertex;
use crate::system::rendering::vulkan::renderer::VkRenderer;

pub struct ResaWindow {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub mode: WindowMode,
    renderer_loop: Box<dyn IRenderer>,
    window: Window,
    event_loop: EventLoop<()>,
    os: String,
}

impl ResaWindow {

    pub fn init(ini_data: &WindowIniData, os_name: &str) -> ResaWindow {
        let event_loop = EventLoop::new();

        let window_size = LogicalSize::new(ini_data.window_width, ini_data.window_height);
        let window_mode = match ini_data.window_mode{
            WindowMode::Windowed => { None }
            WindowMode::Fullscreen => { None }
        };

        let window = WindowBuilder::new()
            .with_title(&ini_data.window_title)
            .with_inner_size(window_size)
            .with_fullscreen(window_mode)
            .with_resizable(true)
            .with_always_on_top(true)
            .with_transparent(false)
            .build(&event_loop).unwrap();

        // if self.os == "Darwin" {
        //     println!("Init metal rs here!");
        // } else {
        let renderer_loop = Box::new(VkRenderer::new(&window));
        // }


        let instance = ResaWindow {
            width: ini_data.window_width,
            height: ini_data.window_height,
            title: ini_data.window_title.to_string(),
            mode: ini_data.window_mode,
            renderer_loop,
            window,
            event_loop,
            os: os_name.to_string(),
        };

        instance
    }

    pub fn get_gpu_name(&self) -> String {
        let name = self.renderer_loop.get_gpu_name().to_string();
        name
    }

    pub fn run_window_loop(self) {
        let win = self.window;
        let mut renderer = self.renderer_loop;


        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested, ..
                } => {
                    *control_flow = ControlFlow::Exit;
                    renderer.dispose();
                }
                Event::MainEventsCleared => {
                    win.request_redraw();
                }
                Event::RedrawRequested(_) => {}
                _ => ()
            }

            let meshes = [Mesh { vertices: Box::new([]), indices: Box::new([]), faces: Box::new([]) }];
            renderer.render(&meshes);
        });
    }
}