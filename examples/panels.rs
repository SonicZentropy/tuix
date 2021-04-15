use tuix::*;

#[cfg(feature = "wgpu")]
use tuix_wgpu::application::Application;
#[cfg(not(feature = "wgpu"))]
use tuix::Application;

const STYLE: &str = r#"

    *:focus {
        border-width: 1px;
        border-color: black;
    }


    panel>.header {
        background-color: #ff5e1a;
    }

    panel .container1 {
        background-color: white;
    }

    panel .container2 {
        padding: 10px;
        align-items: center;
    }

    button {
        background-color: #ff5e1a;
    }

    button:hover {
        background-color: #ff7033;
    }

    panel.one {
        margin: 10px;
        width: 300px;
        flex-direction: column;
    }

    panel.one>.header {
        flex-basis: 30px;
        flex-direction: row;
    }

    panel.two {
        margin: 10px;
        height: 100px;
        flex-direction: row;
    }

    panel.two>.header {
        flex-basis: 80px;
        flex-direction: column;
    }

    panel.two>.header>label {
        text-align: start;
        text-justify: center;
    }

"#;

fn main() {
    let app = Application::new(|state, window| {
        state.add_theme(STYLE);

        window.set_title("Panels");

        let panel = Panel::new("Panel 1").build(state, window.entity(), |builder| builder.class("one"));

        Button::with_label("Button").build(state, panel, |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
                .set_text_justify(Justify::Center)
        });

        let panel = Panel::new("Panel 2").build(state, window.entity(), |builder| builder.class("two"));

        Button::with_label("Button").build(state, panel, |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
                .set_text_justify(Justify::Center)
        });


    });

    app.run();
}
