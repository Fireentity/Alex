use anyhow::anyhow;
use vulkanalia::{Entry, Instance};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::vk::InstanceV1_0;
use winit::window::Window;
use crate::create_instance;

#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance
}

impl App {
    unsafe fn create(window: &Window) -> Self {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|error| anyhow!("{}",error))?;
        let instance = create_instance(window, &entr)?;
        Self {entry, instance }
    }

    unsafe fn render(&self, window: &Window) {
    }

    unsafe fn destroy(&self) {
        self.instance.destroy_instance(None);
    }
}