use crate::{State, Entity};
use crate::widgets::*;
pub trait Inspectable: Default {
    fn widget(&self, state: &mut State, parent: Entity, name: &str) -> Entity;
}

impl Inspectable for String {
    fn widget(&self, state: &mut State, parent: Entity, name: &str) -> Entity {
        let row = HBox::new().build(state, parent, |builder| builder);
        let label = Label::new(name).build(state, row, |builder| builder);
        Textbox::new(self).build(state, row, |builder| builder.set_flex_grow(1.0))
    }
}

impl Inspectable for bool {
    fn widget(&self, state: &mut State, parent: Entity, name: &str) -> Entity {
        let row = HBox::new().build(state, parent, |builder| builder);
        let label = Label::new(name).build(state, row, |builder| builder);
        Checkbox::new(*self).build(state, row, |builder| builder)
    }
}

impl Inspectable for i32 {
    fn widget(&self, state: &mut State, parent: Entity, name: &str) -> Entity {
        let row = HBox::new().build(state, parent, |builder| builder);
        let label = Label::new(name).build(state, row, |builder| builder);
        Spinbox::new(*self)
             .with_min(95)
             .with_max(105)
             //.on_min(Event::new(CheckboxEvent::Uncheck).target(switch))
             //.on_max(Event::new(CheckboxEvent::Check).target(switch))
             .build(state, row, |builder| builder.set_flex_grow(1.0))
    }
}