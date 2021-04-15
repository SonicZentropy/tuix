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
		//TODO: This should use LogicalSize instead, but it breaks our swapchain atm
		//example for fix https://github.com/ebkalderon/renderdoc-rs/blob/master/examples/triangle.rs
		let size = winit::dpi::PhysicalSize::new(inner.width, inner.height);
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
