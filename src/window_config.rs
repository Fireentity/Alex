use winit::dpi::LogicalSize;
use winit::window::{Window, WindowBuilder};
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowConfig {
    width: i32,
    height: i32,
    title: str,
    resizable: bool,
}

impl WindowConfig {

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn resizable(&self) -> bool {
        self.resizable
    }

    pub fn to_window_config(&self) -> Window {
        WindowBuilder::new()
            .with_title(self.title())
            .with_inner_size(LogicalSize::new(self.width(),self.height()))
            .with_resizable(self.resizable())
            .build(&event_loop)
            .expect("Unable to create window from WindowBuilder::new()")
    }
}