use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::ControlFlow;
use anyhow::anyhow;
use vulkanalia::{Entry, Instance, vk};
use vulkanalia::vk::{Display, HasBuilder};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use crate::app::App;
use crate::event_listener::EventListener;
use crate::window_config::WindowConfig;


pub struct Axel {
    destroying: bool,
    app: App,
    windows_config: WindowConfig,
    event_loop: EventLoop<()>,
    event_listener: EventListener,
}

impl Axel {
    pub const CONFIG_RELATIVE_PATH: & 'static str = &"./config.json";

    pub fn windows_config(&self) -> &WindowConfig {
        &self.windows_config
    }

    pub fn destroying(&self) -> bool {
        self.destroying
    }

    pub fn destroying_mut(&mut self) -> &mut bool {
        &mut self.destroying
    }

    pub unsafe fn new() -> Axel {
        pretty_env_logger::init();

        let &mut data: String = String::new();
        File::open("arp.txt")
            .expect("File not found {}")
            .read_to_string(data)
            .expect("Cannot read file");

        let window_config: WindowConfig = serde_json::from_str(data)
            .expect("Cannot parse window config");

        let window = window_config.to_window_config();

        let event_loop = EventLoop::new();

        let mut app = App::new(window);

        let mut destroying = false;

        Axel{app, destroying,windows_config,event_loop,event_listener}
    }

    pub fn start(&mut self) {

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            self.event_listener.dispatch(&event);
            match event {
                // Render a frame if our Vulkan app is not being destroyed.
                Event::MainEventsCleared if !destroying => unsafe { self.app.render(&window) }.unwrap(),
                // Destroy our Vulkan app.
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    self.destroying = true;
                    *control_flow = ControlFlow::Exit;
                    unsafe { app.destroy(); }
                }
                _ => {}
            }
        });
    }
}