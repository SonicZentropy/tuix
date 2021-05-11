pub use tuix_wgpu::window::Window;
pub use tuix_winit::EventLoop;
pub use tuix_wgpu::application::WGPURenderer;
pub use tuix_wgpu::{resource, pollster} ;

pub use tuix_winit::WinitEvent;
use femtovg::Canvas;
use femtovg::renderer::{WGPU};

pub type RenderCanvas = femtovg::Canvas<WGPU>;
pub type BackendRenderer = WGPURenderer;

pub async fn create_renderer(window: &Window) -> BackendRenderer {
	WGPURenderer::create_renderer(&window.winit_window).await
}
