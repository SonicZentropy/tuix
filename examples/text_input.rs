use tuix::*;

#[cfg(feature = "wgpu")]
use tuix_wgpu::application::Application;
#[cfg(not(feature = "wgpu"))]
use tuix::Application;

fn main() {
    let app = Application::new(|state, window| {

        window.set_title("Hello GUI");

        Textbox::new("ä")
            .build(state, window.entity(), |builder| {
                builder
                    .set_width(Units::Pixels(100.0))
                    .set_height(Units::Pixels(30.0))
                    .set_background_color(Color::from("#202020"))
                    .set_text_justify(Justify::Center)
        });
    });
    app.run();
}
