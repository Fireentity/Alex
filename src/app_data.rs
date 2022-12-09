use vulkanalia::{Entry, Instance, vk};
use vulkanalia::vk::{HasBuilder, PhysicalDevice};
use winit::window::Window;

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {
    physical_device: PhysicalDevice,
}