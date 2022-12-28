extern crate core;

mod app;
mod app_data;
mod axel;
mod window_config;
mod event_listener;

use std::error::Error;
use anyhow::{anyhow, ensure, Result};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::axel::Axel;

#[rustfmt::skip]
fn main() {
    let result = Axel::new();
    match result {
        Err(error) => panic!("{}",error),
        Ok(()) => {}
    }
}