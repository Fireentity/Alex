use std::error::Error;
use std::ops::ControlFlow;
use vulkanalia::{Entry, Instance, vk};
use vulkanalia::vk::{Display, HasBuilder};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct Axel;

impl Axel {
    pub const CONFIG_RELATIVE_PATH: & 'static str = "./config.json";

    pub fn new() -> Result<(), dyn Error> {
        pretty_env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Vulkan Tutorial (Rust)")
            .with_inner_size(LogicalSize::new(1024, 768))
            .build(&event_loop)?;

        let mut app = unsafe { App::create(&window)? };
        let mut destroying = false;
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                // Render a frame if our Vulkan app is not being destroyed.
                Event::MainEventsCleared if !destroying => unsafe { app.render(&window) }.unwrap(),
                // Destroy our Vulkan app.
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    destroying = true;
                    *control_flow = ControlFlow::Exit;
                    unsafe { app.destroy(); }
                }
                _ => {}
            }
        });
    }

    pub unsafe fn create_instance(window: &Window, entry: &Entry) -> Instance {

        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Alex\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(b"Age\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 0, 0));

        let extensions = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        let info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions);

        entry.create_instance(&info, None)?
    }
}