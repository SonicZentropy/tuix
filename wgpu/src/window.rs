use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};
use tuix_core::WindowDescription;
use winit::dpi::PhysicalSize;


pub struct Window {
	pub winit_window: winit::window::Window,
}

impl Window {
	pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
		let inner = &window_description.inner_size;
		let size = winit::dpi::LogicalSize::new(inner.width, inner.height);
		let winit_window = winit::window::WindowBuilder::new()
			.with_inner_size(size)
			.with_title("tuix wgpu demo")
			.build(events_loop)
			.expect("Couldn't build winit window");

		Self {
			winit_window
		}
	}
}
