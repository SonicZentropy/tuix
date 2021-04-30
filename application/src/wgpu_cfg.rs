pub use tuix_wgpu::window::Window;
pub use tuix_winit::EventLoop;
use femtovg::renderer::WGPU;

pub type RenderCanvas = femtovg::Canvas<WGPU>;
