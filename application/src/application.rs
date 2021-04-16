//use tuix_core::{State, EventManager, Entity, Units, BoundingBox, WindowWidget,
//                apply_clipping, Fonts, Propagation, MouseButtonState, PropSet, Visibility, Display, apply_hover, MouseButton};
//use winit::dpi::Size;
//use winit::{
//	event_loop::{ControlFlow, EventLoop},
//};
//
//use winit::event::{Event as WinitEvent, WindowEvent, ElementState, VirtualKeyCode, KeyboardInput};
//
//use crate::window::Window;
//use femtovg::renderer::{WGPUInstance, WGPUContext, WGPUSwapChain, WGPU};
//use femtovg::{Canvas, FontId, ImageFlags, Color, Align, Baseline, ImageId, Paint, Path, Renderer};
//use femtovg::Size as FemtoSize;
//
//use resource::resource;
//
//use tuix_core::event::Event;
//use tuix_core::events::WindowEvent as TuixWindowEvent;
//use std::time::Instant;
//use crate::keyboard::{vcode_to_code, scan_to_code, vk_to_key};
//use tuix_application::Window;

use tuix_core::{State, EventManager, Entity, Units, BoundingBox, WindowWidget};
use winit::event_loop::EventLoop;
use crate::window::Window;

pub struct Application {
	pub window: Window,
	pub state: State,
	event_loop: EventLoop<()>,
	pub event_manager: EventManager,
}

pub trait TuixRenderer {
	fn run(self, window: Window, event_loop: EventLoop<()>, event_manager: EventManager, state: State);
}

impl Application {
	pub fn new<F: FnOnce(&mut State, &mut tuix_core::WindowBuilder)>(
		app: F,
	) -> Self {
		let event_loop = EventLoop::new();
		let mut state = State::new();

		let event_manager = EventManager::new();

		let root = Entity::root();
		state.hierarchy.add(Entity::root(), None);

		let mut tuix_window_builder = tuix_core::WindowBuilder::new(root);
		app(&mut state, &mut tuix_window_builder);
		let window_description = tuix_window_builder.get_window_description();
		let window = Window::new(&event_loop, window_description);

		state.style.width.insert(
			Entity::root(),
			Units::Pixels(window_description.inner_size.width as f32),
		);
		state.style.height.insert(
			Entity::root(),
			Units::Pixels(window_description.inner_size.height as f32),
		);

		state
			.data
			.set_width(Entity::root(), window_description.inner_size.width as f32);
		state
			.data
			.set_height(Entity::root(), window_description.inner_size.height as f32);
		state.data.set_opacity(Entity::root(), 1.0);

		let mut bounding_box = BoundingBox::default();
		bounding_box.w = window_description.inner_size.width as f32;
		bounding_box.h = window_description.inner_size.height as f32;

		state.data.set_clip_region(Entity::root(), bounding_box);

		WindowWidget::new().build_window(&mut state);

		Application {
			window,
			event_loop,
			event_manager,
			state,
		}
	}

	pub fn run(self) {
		//pollster::block_on(self.run_internal())
		let window = self.window;
		let event_loop = self.event_loop;
		let event_manager = self.event_manager;
		let state = self.state;

		#[cfg(feature = "wgpu")]
		let renderer = WgpuRenderer;
		renderer.run(self);
	}
}
