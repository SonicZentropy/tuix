use winit::{
	event_loop::{ EventLoop },
};
use tuix_core::WindowDescription;

#[cfg(target_os="windows")]
use winit::platform::windows::WindowBuilderExtWindows;


pub struct Window {
	pub winit_window: winit::window::Window,
}

impl Window {
	pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
		let inner = &window_description.inner_size;
		//TODO: This should use LogicalSize instead, but it breaks our swapchain atm
		//example for fix https://github.com/ebkalderon/renderdoc-rs/blob/master/examples/triangle.rs
		let size = winit::dpi::PhysicalSize::new(inner.width, inner.height);

		#[cfg(target_os="windows")]
			let builder = winit::window::WindowBuilder::new().with_drag_and_drop(false);

		#[cfg(not(target_os="windows"))]
			let builder = winit::window::WindowBuilder::new();

		let winit_window = builder
			.with_inner_size(size)
			.with_title("tuix wgpu demo")
			// This is forced off because it breaks Windows
			.build(events_loop)
			.expect("Couldn't build winit window");

		Self {
			winit_window
		}
	}
}
