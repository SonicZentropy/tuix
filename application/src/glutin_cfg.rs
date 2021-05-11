pub use tuix_wgpu::window::Window;
pub use tuix_winit::EventLoop;
use femtovg::renderer::OpenGl;

pub type RenderCanvas = femtovg::Canvas<OpenGl>;

pub type WinitEvent<'a, T> = glutin::event::Event<'a, T>;


pub struct GlutinRenderingThing {
	pub canvas: RenderCanvas,
}


type RenderingThing = GlutinRenderingThing;

pub fn create_renderer() -> RenderingThing {

}

