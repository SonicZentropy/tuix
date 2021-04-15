extern crate tuix;
use tuix::*;

use tuix::button::Button;
#[cfg(feature = "wgpu")]
use tuix_wgpu::application::Application;
#[cfg(not(feature = "wgpu"))]
use tuix::Application;

static THEME: &'static str = include_str!("themes/counter_theme.css");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
    label: Entity,
}

impl Counter {
    pub fn set_initial_value(mut self, val: i32) -> Self {
        self.value = val;
        self
    }
}

impl Widget for Counter {
    type Ret = Entity;

    // Build
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        Button::with_label("increment")
            .on_press(Event::new(CounterMessage::Increment))
            .build(state, entity, |builder| builder.class("increment"));

        Button::with_label("decrement")
            .on_press(Event::new(CounterMessage::Decrement))
            .build(state, entity, |builder| builder.class("decrement"));

        self.label = Label::new(&self.value.to_string()).build(state, entity, |builder| builder);

        entity.set_element(state, "counter")
    }

    // Events
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast::<CounterMessage>() {
            match counter_event {
                CounterMessage::Increment => {
                    self.value += 1;
                    self.label.set_text(state, &self.value.to_string());
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                    self.label.set_text(state, &self.value.to_string());
                }
            }
        }
    }
}

fn main() {
    // Create the app
    let app = Application::new(|state, window| {
        state.add_theme(THEME);

        // Set the window title and size
        window.set_title("Counter").set_inner_size(400, 100);

        Counter::default()
            // Set local state
            .set_initial_value(50)
            // Build the widget
            .build(state, window.entity(), |builder| builder);
    });

    app.run();
}
