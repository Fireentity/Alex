use anyhow::anyhow;
use vulkanalia::{Entry, Instance};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::vk::InstanceV1_0;
use winit::window::Window;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window;

#[derive(Clone, Debug)]
pub struct App {
    window: Window,
    entry: Entry,
    instance: Instance
}

impl App {

    pub unsafe fn new(window: Window) -> Self {

        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Alex\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(b"Age\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 0, 0));

        let extensions = window::get_required_instance_extensions(&window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        let info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions);

        let loader = LibloadingLoader::new(LIBRARY).expect("Unable to create loader");
        let entry = Entry::new(loader).expect("Error while creating Vulkan entry");
        let instance = entry.create_instance(&info, None).expect("Unable to create Vulkan instance");
        App {window, entry, instance }
    }

    pub(crate) unsafe fn render(&self, window: &Window) {
    }

    pub(crate) unsafe fn destroy(&self) {
        self.instance.destroy_instance(None);
    }
}