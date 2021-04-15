use tuix_core::{State, EventManager, Entity, Units, BoundingBox, WindowWidget,
                apply_clipping, Fonts, Propagation, MouseButtonState, PropSet, Visibility, Display, apply_hover, MouseButton};
use tuix_core::Size as TuixSize;
use winit::dpi::Size as Size;

use winit::{
	//event::*,
	event_loop::{ControlFlow, EventLoop},
};

use winit::event::{Event as WinitEvent, WindowEvent, ElementState, VirtualKeyCode, KeyboardInput};

use crate::window::Window;
use femtovg::renderer::{WGPUInstance, WGPUContext, WGPUSwapChain, WGPU};
use femtovg::{Canvas, FontId, ImageFlags, Color, Align, Baseline, ImageId, Paint, Path, Renderer};
use femtovg::Size as FemtoSize;

use resource::resource;

use tuix_core::event::Event;
use tuix_core::events::WindowEvent as TuixWindowEvent;
use std::time::Instant;
use crate::keyboard::{vcode_to_code, scan_to_code, vk_to_key};

pub struct Application {
	pub window: Window,
	pub state: State,
	event_loop: EventLoop<()>,
	pub event_manager: EventManager,
}


type RenderCanvas = femtovg::Canvas<WGPU>;

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
		pollster::block_on(self.run_internal())
	}

	async fn run_internal(self) {
		let mut state = self.state;
		let mut event_manager = self.event_manager;
		let mut window = self.window;

		let mut should_quit = false;

		state.insert_event(Event::new(TuixWindowEvent::Restyle).target(Entity::root()));
		state.insert_event(Event::new(TuixWindowEvent::Relayout).target(Entity::root()));

		let size = window.winit_window.inner_size();

		let instance = WGPUInstance::from_window(&window.winit_window).await.unwrap();
		let ctx = WGPUContext::new(instance).await.unwrap();
		let size = FemtoSize::new(size.width as _, size.height as _);
		let mut swap_chain = WGPUSwapChain::new(&ctx, size);
		let renderer = WGPU::new(&ctx, size, swap_chain.format());
		let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

		let fonts = Fonts {
			regular: Some(canvas
				.add_font_mem(&resource!("../examples/assets/Roboto-Regular.ttf"))
				.expect("Cannot add font")
			),
			bold: Some(canvas
				.add_font_mem(&resource!("../examples/assets/Roboto-Light.ttf"))
				.expect("Cannot add font")
			),
			icons: Some(canvas
				.add_font_mem(&resource!("../examples/assets/entypo.ttf"))
				.expect("Cannot add font")
			),
			emoji: None,
		};

		state.fonts = fonts;

		let image = canvas
			.load_image_mem(&resource!("../examples/assets/images/image4.jpg"), ImageFlags::empty())
			.unwrap();

		//let mut screenshot_image_id = None;

		let start = Instant::now();
		let mut prevt = start;

		let mut mousex = 0.0;
		let mut mousey = 0.0;
		let mut dragging = false;

		//gfx hasn't implemented this for vulkan yet
		//TODO Fix gfx to implement this because it's awesome
		//ctx.device().start_capture();

		// let mut perf = PerfGraph::new();

		let mut frame_count = 0;

		let mut event_loop_proxy = self.event_loop.create_proxy();
		state.needs_redraw = true;

		self.event_loop.run(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;

			match event {
				WinitEvent::LoopDestroyed => return,

				WinitEvent::UserEvent(_) => {
					window.winit_window.request_redraw();
				}
				WinitEvent::RedrawRequested(_) => {
					let now = Instant::now();
					let dt = (now - prevt).as_secs_f32();
					prevt = now;

					let dpi_factor = window.winit_window.scale_factor();
					let size = window.winit_window.inner_size();

					let frame = swap_chain.get_current_frame().unwrap();
					let target = &frame.output.view;

					let hierarchy = state.hierarchy.clone();
					event_manager.draw(&mut state, &hierarchy, &mut canvas);

					canvas.flush(Some(target));
					frame_count += 1;
				}

				WinitEvent::WindowEvent { event, .. } => match event {
					//////////////////
					// Close Window //
					//////////////////
					WindowEvent::CloseRequested => {
						state.insert_event(Event::new(TuixWindowEvent::WindowClose));
						should_quit = true;
					}

					// TODO: there's a todo in glutin/applications.rs of this exact same code so
					///////////////////////
					// Modifiers Changed //
					///////////////////////
					WindowEvent::ModifiersChanged(modifiers_state) => {
						state.modifiers.shift = modifiers_state.shift();
						state.modifiers.ctrl = modifiers_state.ctrl();
						state.modifiers.alt = modifiers_state.alt();
						state.modifiers.logo = modifiers_state.logo();
					}

					////////////////////
					// Focused Window //
					////////////////////
					WindowEvent::Focused(_) => {
						state.insert_event(Event::new(TuixWindowEvent::Restyle).target(Entity::root()));
						state.insert_event(Event::new(TuixWindowEvent::Relayout).target(Entity::root()));
						state.insert_event(Event::new(TuixWindowEvent::Redraw).target(Entity::root()));
					}

					////////////////////
					// Focused Window //
					////////////////////
					WindowEvent::ReceivedCharacter(input) => {
						state.insert_event(
							// theglutin  ver takes event btw, while we take ref event. what are u talking about sry
							Event::new(TuixWindowEvent::CharInput(input))
								.target(state.focused)
								.propagate(Propagation::Down),
						);
					}

					#[cfg(not(target_arch = "wasm32"))]
					WindowEvent::Resized(physical_size) => {
						let new_size = FemtoSize::new(physical_size.width as _, physical_size.height as _);
						canvas.set_size(new_size.w as _, new_size.h as _, 1.0);
						swap_chain.resize(new_size);

						state
							.style
							.width
							.insert(Entity::root(), Units::Pixels(new_size.w as f32));
						state
							.style
							.height
							.insert(Entity::root(), Units::Pixels(new_size.h as f32));

						state
							.data
							.set_width(Entity::root(), new_size.w as f32);
						state
							.data
							.set_height(Entity::root(), new_size.h as f32);

						let mut bounding_box = BoundingBox::default();
						bounding_box.w = new_size.w as f32;
						bounding_box.h = new_size.h as f32;

						state.data.set_clip_region(Entity::root(), bounding_box);

						state.insert_event(Event::new(TuixWindowEvent::Restyle).target(Entity::root()));
						state.insert_event(Event::new(TuixWindowEvent::Relayout).target(Entity::root()));
						state.insert_event(Event::new(TuixWindowEvent::Redraw).target(Entity::root()));
					}
					WindowEvent::CursorMoved {
						device_id: _, position, ..
					} => {
						let cursorx = (position.x) as f32;
						let cursory = (position.y) as f32;

						state.mouse.cursorx = cursorx as f32;
						state.mouse.cursory = cursory as f32;

						apply_hover(&mut state);

						if state.captured != Entity::null() {
							state.insert_event(
								Event::new(TuixWindowEvent::MouseMove(cursorx, cursory))
									.target(state.captured)
									.propagate(Propagation::Direct),
							);
						} else if state.hovered != Entity::root() {
							state.insert_event(
								Event::new(TuixWindowEvent::MouseMove(cursorx, cursory))
									.target(state.hovered),
							);
						}
					}
					WindowEvent::MouseWheel {
						device_id: _, delta, ..
					} => {
						let (x, y) = match delta {
							winit::event::MouseScrollDelta::LineDelta(xx, yy) => (xx, yy),
							_ => (0.0, 0.0),
						};

						if state.captured != Entity::null() {
							state.insert_event(
								Event::new(TuixWindowEvent::MouseScroll(x, y))
									.target(state.captured)
									.propagate(Propagation::Direct),
							);
						} else {
							state.insert_event(
								Event::new(TuixWindowEvent::MouseScroll(x, y))
									.target(state.hovered),
							);
						}
					}
					WindowEvent::MouseInput {
						button,
						state: s,
						..
					} => {
						let s = match s {
							winit::event::ElementState::Pressed => MouseButtonState::Pressed,
							winit::event::ElementState::Released => MouseButtonState::Released,
						};

						let b = match button {
							winit::event::MouseButton::Left => MouseButton::Left,
							winit::event::MouseButton::Right => MouseButton::Right,
							winit::event::MouseButton::Middle => MouseButton::Middle,
							winit::event::MouseButton::Other(id) => MouseButton::Other(id),
						};

						match b {
							MouseButton::Left => {
								state.mouse.left.state = s;
							}

							MouseButton::Right => {
								state.mouse.right.state = s;
							}

							MouseButton::Middle => {
								state.mouse.middle.state = s;
							}

							_ => {}
						}

						match s {
							MouseButtonState::Pressed => {
								if state.hovered != Entity::null()
									&& state.active != state.hovered
								{
									state.active = state.hovered;
									state.insert_event(Event::new(TuixWindowEvent::Restyle).target(Entity::root()));
									state.needs_restyle = true;
								}

								if state.captured != Entity::null() {
									state.insert_event(
										Event::new(TuixWindowEvent::MouseDown(b))
											.target(state.captured)
											.propagate(Propagation::Direct),
									);
								} else {
									state.insert_event(
										Event::new(TuixWindowEvent::MouseDown(b))
											.target(state.hovered),
									);
								}

								match b {
									MouseButton::Left => {
										state.mouse.left.pos_down =
											(state.mouse.cursorx, state.mouse.cursory);
										state.mouse.left.pressed = state.hovered;
									}

									MouseButton::Middle => {
										state.mouse.middle.pos_down =
											(state.mouse.cursorx, state.mouse.cursory);
										state.mouse.left.pressed = state.hovered;
									}

									MouseButton::Right => {
										state.mouse.right.pos_down =
											(state.mouse.cursorx, state.mouse.cursory);
										state.mouse.left.pressed = state.hovered;
									}

									_ => {}
								}
							}

							MouseButtonState::Released => {
								state.active = Entity::null();
								//state.insert_event(Event::new(WindowEvent::Restyle));
								state.needs_restyle = true;

								if state.captured != Entity::null() {
									state.insert_event(
										Event::new(TuixWindowEvent::MouseUp(b))
											.target(state.captured)
											.propagate(Propagation::Direct),
									);
								} else {
									state.insert_event(
										Event::new(TuixWindowEvent::MouseUp(b))
											.target(state.hovered),
									);
								}

								match b {
									MouseButton::Left => {
										state.mouse.left.pos_up =
											(state.mouse.cursorx, state.mouse.cursory);
										state.mouse.left.released = state.hovered;
									}

									MouseButton::Middle => {
										state.mouse.middle.pos_up =
											(state.mouse.cursorx, state.mouse.cursory);
										state.mouse.left.released = state.hovered;
									}

									MouseButton::Right => {
										state.mouse.right.pos_up =
											(state.mouse.cursorx, state.mouse.cursory);
										state.mouse.left.released = state.hovered;
									}

									_ => {}
								}
							}
						}
					}

						//wtf happened to this
					WindowEvent::KeyboardInput {
						device_id: _,
						input,
						is_synthetic: _,
					} => {
						let s = match input.state {
							winit::event::ElementState::Pressed => MouseButtonState::Pressed,
							winit::event::ElementState::Released => MouseButtonState::Released,
						};

						// Prefer virtual keycodes to scancodes, as scancodes aren't uniform between platforms
						let code = if let Some(vkey) = input.virtual_keycode {
							vcode_to_code(vkey)
						} else {
							scan_to_code(input.scancode)
						};

						let key = vk_to_key(
							input.virtual_keycode.unwrap_or(VirtualKeyCode::NoConvert),
						);

						if let Some(virtual_keycode) = input.virtual_keycode {
							if virtual_keycode == VirtualKeyCode::F5
								&& s == MouseButtonState::Pressed
							{
								state.reload_styles().unwrap();
							}

							if virtual_keycode == VirtualKeyCode::H && s == MouseButtonState::Pressed {
								println!("Hierarchy");
								for entity in state.hierarchy.into_iter() {
									//println!("Entity: {}  Parent: {:?} FC: {:?} NS: {:?}", entity, state.hierarchy.get_parent(entity), state.hierarchy.get_first_child(entity), state.hierarchy.get_next_sibling(entity));
									println!("Entity: {} posx: {} posy: {} width: {} height: {} visibility: {:?}", entity, state.data.get_posx(entity), state.data.get_posy(entity), state.data.get_width(entity), state.data.get_height(entity), state.data.get_visibility(entity));
								}
							}

							if virtual_keycode == VirtualKeyCode::Tab
								&& s == MouseButtonState::Pressed
							{
								let next_focus = state
									.style
									.focus_order
									.get(state.focused)
									.cloned()
									.unwrap_or_default()
									.next;
								let prev_focus = state
									.style
									.focus_order
									.get(state.focused)
									.cloned()
									.unwrap_or_default()
									.prev;

								if state.modifiers.shift {
									if prev_focus != Entity::null() {
										state.focused.set_focus(&mut state, false);
										state.focused = prev_focus;
										state.focused.set_focus(&mut state, true);
									} else {
										// TODO impliment reverse iterator for hierarchy
										// state.focused = match state.focused.into_iter(&state.hierarchy).next() {
										//     Some(val) => val,
										//     None => Entity::root(),
										// };
									}
								} else {
									let hierarchy = state.hierarchy.clone();


									//let next = iter.next();

									println!("Focused: {}", state.focused);


									if next_focus != Entity::null() {
										state.focused.set_focus(&mut state, false);
										state.focused = next_focus;
										state.focused.set_focus(&mut state, true);
									} else {
										state.focused.set_focus(&mut state, false);

										use tuix_core::IntoHierarchyIterator;
										let mut iter = state.focused.into_iter(&hierarchy);
										iter.next();


										state.focused = if let Some(mut temp) = iter.next() {
											while !state.data.get_focusability(temp)
												|| state.data.get_visibility(temp) == Visibility::Invisible
												|| state.data.get_opacity(temp) == 0.0
												|| state.style.display.get(temp) == Some(&Display::None)
											{
												temp = match iter.next() {
													Some(e) => e,
													None => {
														break;
													}
												}
											}

											temp
										} else {
											Entity::root()
										};

										state.focused.set_focus(&mut state, true);
									}
								}


								state.insert_event(
									Event::new(TuixWindowEvent::Restyle)
										.target(Entity::root())
										.origin(Entity::root()),
								);
							}
						}

						match s {
							MouseButtonState::Pressed => {
								if state.focused != Entity::null() {
									state.insert_event(
										Event::new(TuixWindowEvent::KeyDown(code, key))
											.target(state.focused)
											.propagate(Propagation::DownUp),
									);
								} else {
									state.insert_event(
										Event::new(TuixWindowEvent::KeyDown(code, key))
											.target(state.hovered)
											.propagate(Propagation::DownUp),
									);
								}
							}

							MouseButtonState::Released => {
								if state.focused != Entity::null() {
									state.insert_event(
										Event::new(TuixWindowEvent::KeyUp(code, key))
											.target(state.focused)
											.propagate(Propagation::DownUp),
									);
								} else {
									state.insert_event(
										Event::new(TuixWindowEvent::KeyUp(code, key))
											.target(state.hovered)
											.propagate(Propagation::DownUp),
									);
								}
							}
						}
					}

					_ => (),
				},

				WinitEvent::MainEventsCleared => { //done
					while !state.event_queue.is_empty() {
						event_manager.flush_events(&mut state);
					}

					if state.apply_animations() {
						*control_flow = ControlFlow::Poll;

						state.insert_event(Event::new(TuixWindowEvent::Relayout)
							.target(Entity::root()));

						//This triggers Event::UserEvent to switch event loop from wait to poll
						event_loop_proxy.send_event(()).unwrap();
						window.winit_window.request_redraw();
					} else {
						*control_flow = ControlFlow::Wait;
					}

					let hierarchy = state.hierarchy.clone();

					if state.needs_redraw {
						apply_clipping(&mut state, &hierarchy);
						window.winit_window.request_redraw();
						state.needs_redraw = false;
					}
				}
				_ => (),
			}

			if should_quit {
				*control_flow = ControlFlow::Exit;
			}
		});
	}
}

//fn draw_text<T: Renderer>(canvas: &mut Canvas<T>, fonts: &Fonts, title: &str, x: f32, y: f32, w: f32, h: f32) {
//	canvas.save();
//	let mut text_paint = Paint::color(Color::rgba(255, 0, 0, 255));
//	text_paint.set_font_size(80.0);
//	text_paint.set_font(&[fonts.regular]);
//	text_paint.set_text_align(Align::Left);
//	text_paint.set_text_baseline(Baseline::Middle);
//	let _ = canvas.fill_text(x + h, y + h * 0.5, title, text_paint);
//	canvas.restore();
//}
//
//fn draw_image(canvas: &mut Canvas<impl Renderer>, x: f32, y: f32, images: &[ImageId]) {
//	let (w, h) = canvas.image_size(images[0]).unwrap();
//
//	let paint = Paint::image(images[0], x, y, w as f32, h as f32, 0.0, 1.0);
//	let mut path = Path::new();
//	path.rect(x, y, w as _, h as _);
//	canvas.fill_path(&mut path, paint);
//}
//
//

