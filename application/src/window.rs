use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};
use tuix_core::WindowDescription;
use winit::dpi::PhysicalSize;


pub struct Window(winit::window::Window);

impl Window {
	pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
		let inner = &window_description.inner_size;
		//TODO: This should use LogicalSize instead, but it breaks our swapchain atm
		//example for fix https://github.com/ebkalderon/renderdoc-rs/blob/master/examples/triangle.rs
		let size = winit::dpi::PhysicalSize::new(inner.width, inner.height);
		let winit_window = winit::window::WindowBuilder::new()
			.with_inner_size(size)
			.with_min_inner_size(size)
			.with_title("tuix wgpu demo")
			.with_window_icon(if let Some(icon) = &window_description.icon {
				Some(
					winit::window::Icon::from_rgba(
						icon.clone(),
						window_description.icon_width,
						window_description.icon_height,
					)
						.unwrap(),
				)
			} else {
				None
			})
			.build(events_loop)
			.expect("Couldn't build winit window");

		Self(winit_window)
	}
}
