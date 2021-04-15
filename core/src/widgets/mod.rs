pub mod element;
pub use element::Element;

pub mod buttons;
pub use buttons::*;

pub mod inputs;
pub use inputs::*;

pub mod popups;
pub use popups::*;

pub mod menus;
pub use menus::*;

pub mod scrollbar;
pub use scrollbar::*;

pub mod progress;
pub use progress::*;


pub mod tab;
pub use tab::*;


pub mod dropdown;
pub use dropdown::*;

pub mod scroll_container;
pub use scroll_container::*;

pub mod value_slider;
pub use value_slider::ValueSlider;

pub mod length_box;
pub use length_box::LengthBox;

pub mod panel;
pub use panel::*;

pub mod label;
pub use label::*;

pub mod containers;
pub use containers::*;

pub mod vector_edit;
pub use vector_edit::*;

pub mod window;
pub use window::WindowWidget;

pub mod tooltip;
pub use tooltip::*;

// Audio Widgets
pub mod audio_widgets;
pub use audio_widgets::*;

pub mod debug_container;
pub use debug_container::*;

pub use crate::entity::Entity;
pub use crate::events::{Event, Propagation, Widget, WindowEvent};
pub use crate::mouse::*;
pub use crate::state::State;
pub use crate::{Code, Key};
pub use crate::{PropGet, PropSet, Animation, AnimationState};
use femtovg::renderer::{OpenGl, WGPU};

#[derive(Default)]
pub struct BaseWidget {
    on_hover: Option<Event>,
}

impl BaseWidget {
    pub fn on_hover(&mut self, event: Event) -> &mut Self {
        self.on_hover = Some(event);

        self
    }
}

pub trait BasicWidget: Sized {
    fn get_base_widget(&mut self) -> &mut BaseWidget;

    fn on_hover(mut self, event: Event) -> Self
    {
        self.get_base_widget().on_hover(event);

        self
    }

    fn on_active(mut self, event: Event) -> Self
    {
        self
    }
}

#[cfg(feature = "wgpu")]
pub type RenderCanvas = femtovg::Canvas<WGPU>;
#[cfg(not(feature = "wgpu"))]
pub type RenderCanvas = femtovg::Canvas<OpenGl>;
