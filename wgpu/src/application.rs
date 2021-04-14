use tuix_core::{State, EventManager, Entity, Units, BoundingBox, WindowWidget};
use tuix_core::Fonts as TuixFonts;
use tuix_core::Size as TuixSize;
use winit::dpi::Size as Size;

use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
};
use crate::window::Window;
use femtovg::renderer::{WGPUInstance, WGPUContext, WGPUSwapChain, WGPU};
use femtovg::{Canvas, FontId, ImageFlags, Color, Align, Baseline, ImageId, Paint, Path, Renderer};
use femtovg::Size as FemtoSize;

use resource::resource;

use tuix_core::event::Event as TuixEvent;
use tuix_core::events::WindowEvent as TuixWindowEvent;
use std::time::Instant;

struct Fonts {
	regular: FontId,
	bold: FontId,
	icons: FontId,
}

pub struct Application {
	pub window: Window,
	pub state: State,
	event_loop: EventLoop<()>,
	pub event_manager: EventManager,
}

impl Application {
	pub fn new<F: FnOnce(&mut State, &mut tuix_core::WindowBuilder)> (
		app:F,
	) -> Self {
		let event_loop = EventLoop::new();
		let mut state = State::new();

		let event_manager = EventManager::new();

		let root = Entity::root();
		state.hierarchy.add(Entity::root(), None);

		let mut tuix_window_builder = tuix_core::WindowBuilder::new(root);
		app(&mut state, &mut tuix_window_builder);
		let window_description = tuix_window_builder.get_window_description();
		let inner = &tuix_window_builder.get_window_description().inner_size;

		let size = winit::dpi::LogicalSize::new(inner.width, inner.height);
		let window = Window::new(&event_loop, window_description);

		let regular_font = include_bytes!("../../resources/Roboto-Regular.ttf");
		let bold_font = include_bytes!("../../resources/Roboto-Bold.ttf");
		let icon_font = include_bytes!("../../resources/entypo.ttf");
		let emoji_font = include_bytes!("../../resources/OpenSansEmoji.ttf");

		/*let fonts = Fonts {  //TODO: Fix this
			regular: Some(
				window
					.canvas
					.add_font_mem(regular_font)
					.expect("Cannot add font"),
			),
			bold: Some(
				window
					.canvas
					.add_font_mem(bold_font)
					.expect("Cannot add font"),
			),
			icons: Some(
				window
					.canvas
					.add_font_mem(icon_font)
					.expect("Cannot add font"),
			),
			emoji: Some(
				window
					.canvas
					.add_font_mem(emoji_font)
					.expect("Cannot add font"),
			),
		};

		state.fonts = fonts;*/
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



	pub async fn run(self) {
		let mut state = self.state;
		let mut event_manager = self.event_manager;
		let mut window = self.window;
		let mut should_quit = false;

		state.insert_event(TuixEvent::new(TuixWindowEvent::Restyle).target(Entity::root()));
		state.insert_event(TuixEvent::new(TuixWindowEvent::Relayout).target(Entity::root()));
		state.needs_redraw = true;

		let size = window.winit_window.inner_size();

		let instance = WGPUInstance::from_window(&window.winit_window).await.unwrap();
		let ctx = WGPUContext::new(instance).await.unwrap();
		let size = FemtoSize::new(size.width as _, size.height as _);
		let mut swap_chain = WGPUSwapChain::new(&ctx, size);
		let renderer = WGPU::new(&ctx, size, swap_chain.format());
		let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

		let fonts = Fonts {
			regular: canvas
				.add_font_mem(&resource!("../examples/assets/Roboto-Regular.ttf"))
				.expect("Cannot add font"),
			bold: canvas
				.add_font_mem(&resource!("../examples/assets/Roboto-Light.ttf"))
				.expect("Cannot add font"),
			icons: canvas
				.add_font_mem(&resource!("../examples/assets/entypo.ttf"))
				.expect("Cannot add font"),
		};

		let image = canvas
			.load_image_mem(&resource!("../examples/assets/images/image4.jpg"), ImageFlags::empty())
			.unwrap();

		let mut screenshot_image_id = None;

		let start = Instant::now();
		let mut prevt = start;

		let mut mousex = 0.0;
		let mut mousey = 0.0;
		let mut dragging = false;

		//gfx hasn't implemented this for vulkan yet
		//ctx.device().start_capture();

		// let mut perf = PerfGraph::new();

		let mut frame_count = 0;

		let mut event_loop = self.event_loop;

		event_loop.run(move |event, _, control_flow| {
			// #[cfg(not(target_arch = "wasm32"))]
			// let window = windowed_context.window();

			*control_flow = ControlFlow::Poll;

			match event {
				Event::LoopDestroyed => return,
				Event::WindowEvent { ref event, .. } => match event {
					#[cfg(not(target_arch = "wasm32"))]
					WindowEvent::Resized(new_size) => {
						let new_size = FemtoSize::new(new_size.width as _, new_size.height as _);
						canvas.set_size(new_size.w as _, new_size.h as _, 1.0);
						swap_chain.resize(new_size);
						// todo!("resize");
					}
					WindowEvent::CursorMoved {
						device_id: _, position, ..
					} => {
						// if dragging {
						//     let p0 = canvas.transform().inversed().transform_point(mousex, mousey);
						//     let p1 = canvas
						//         .transform()
						//         .inversed()
						//         .transform_point(position.x as f32, position.y as f32);

						//     canvas.translate(p1.0 - p0.0, p1.1 - p0.1);
						// }

						// mousex = position.x as f32;
						// mousey = position.y as f32;
					}
					WindowEvent::MouseWheel {
						device_id: _, delta, ..
					} => { //match delta {
						// winit::event::MouseScrollDelta::LineDelta(_, y) => {
						//     let pt = canvas.transform().inversed().transform_point(mousex, mousey);
						//     canvas.translate(pt.0, pt.1);
						//     canvas.scale(1.0 + (y / 10.0), 1.0 + (y / 10.0));
						//     canvas.translate(-pt.0, -pt.1);
						// }
						// _ => (),
					}
					WindowEvent::MouseInput {
						button: MouseButton::Left,
						state,
						..
					} => match state {
						ElementState::Pressed => dragging = true,
						ElementState::Released => dragging = false,
					},
					WindowEvent::KeyboardInput {
						input:
						KeyboardInput {
							virtual_keycode: Some(VirtualKeyCode::S),
							state: ElementState::Pressed,
							..
						},
						..
					} => {
						if let Some(screenshot_image_id) = screenshot_image_id {
							canvas.delete_image(screenshot_image_id);
						}

						// if let Ok(image) = canvas.screenshot() {
						//     screenshot_image_id = Some(canvas.create_image(image.as_ref(), ImageFlags::empty()).unwrap());
						// }
					}
					WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
					_ => (),
				},
				Event::RedrawRequested(_) => {
					let now = Instant::now();
					let dt = (now - prevt).as_secs_f32();
					prevt = now;

					// perf.update(dt);

					let dpi_factor = window.winit_window.scale_factor();
					//let size = window.inner_size();
					let size = window.winit_window.inner_size();

					// let t = start.elapsed().as_secs_f32();

					canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
					let bg_color = Color::rgbf(0.3, 0.3, 0.3);
					canvas.clear_rect(0, 0, size.width as u32, size.height as u32, bg_color);


					draw_text(&mut canvas, &fonts, "qwe", 200.0, 0.0, 100.0, 300.0);
					draw_image(&mut canvas, 30.0, 30.0, &[image]);

					let height = 600.0;
					let width = 800.0;

					let t = 0.0;

					let frame = swap_chain.get_current_frame().unwrap();
					let target = &frame.output.view;
					canvas.flush(Some(target));

					frame_count += 1;
				}
				Event::MainEventsCleared => {
					//scroll = 1.0;
					window.winit_window.request_redraw()
				}
				_ => (),
			}
		});

	}
}

fn draw_text<T: Renderer>(canvas: &mut Canvas<T>, fonts: &Fonts, title: &str, x: f32, y: f32, w: f32, h: f32) {
	canvas.save();
	let mut text_paint = Paint::color(Color::rgba(255, 0, 0, 255));
	text_paint.set_font_size(80.0);
	text_paint.set_font(&[fonts.regular]);
	text_paint.set_text_align(Align::Left);
	text_paint.set_text_baseline(Baseline::Middle);
	let _ = canvas.fill_text(x + h, y + h * 0.5, title, text_paint);
	canvas.restore();
}

fn draw_image(canvas: &mut Canvas<impl Renderer>, x: f32, y: f32, images: &[ImageId]) {
	let (w, h) = canvas.image_size(images[0]).unwrap();

	let paint = Paint::image(images[0], x, y, w as f32, h as f32, 0.0, 1.0);
	let mut path = Path::new();
	path.rect(x, y, w as _, h as _);
	canvas.fill_path(&mut path, paint);
}














































