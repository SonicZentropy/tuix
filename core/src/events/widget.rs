use crate::{builder::Builder, EventHandler, RenderCanvas};
use crate::{Entity, Hierarchy, State, AsEntity};
use femtovg::{
    renderer::OpenGl, Align, Baseline, FillRule, FontId, ImageFlags, ImageId, LineCap, LineJoin,
    Paint, Path, Renderer, Solidity,
};

use crate::style::{Direction, Justify, Units, Visibility};
use crate::{Event, EventManager, Message};

pub type Canvas = femtovg::Canvas<OpenGl>;

use std::any::Any;
pub trait Widget: std::marker::Sized + 'static {
    type Ret;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret;

    /// Adds the widget into state and returns the associated type Ret - an entity id or a tuple of entity ids
    fn build<F>(mut self, state: &mut State, parent: Entity, mut builder: F) -> Self::Ret
    where
        F: FnMut(Builder) -> Builder,
        Self: std::marker::Sized + 'static,
    {
        // Create a new entity
        let entity = state.add(parent);

        // Call the on_build function of the widget
        let ret = self.on_build(state, entity);

        // Call the builder closure
        builder(Builder::new(state, entity)).build(self);

        // Return the entity or entities returned by the on_build method
        ret
    }

    // Called when events are flushed
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}

    fn on_test(&self) {}

    // Called when a redraw occurs
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut RenderCanvas) {
        // Skip window
        if entity == Entity::root() {
            return;
        }

        // Skip invisible widgets
        if state.data.get_visibility(entity) == Visibility::Invisible {
            //println!("Invisible: {}", entity);
            return;
        }

        // Skip widgets that have 0 opacity
        if state.data.get_opacity(entity) == 0.0 {
            //println!("Zero Opacity: {}", entity);
            return;
        }

        // let posx = state.data.get_posx(entity);
        // let posy = state.data.get_posy(entity);
        // let width = state.data.get_width(entity);
        // let height = state.data.get_height(entity);

        let bounds = state.data.get_bounds(entity);

        let padding_left = match state
            .style
            .padding_left
            .get(entity)
            .unwrap_or(&Units::Auto)
        {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_right = match state
            .style
            .padding_right
            .get(entity)
            .unwrap_or(&Units::Auto)
        {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_top = match state.style.padding_top.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_bottom = match state
            .style
            .padding_bottom
            .get(entity)
            .unwrap_or(&Units::Auto)
        {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let background_color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();



        let font_color = state
            .style
            .font_color
            .get(entity)
            .cloned()
            .unwrap_or(crate::Color::rgb(255, 255, 255));

        let border_color = state
            .style
            .border_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let border_radius_top_left = match state
            .style
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let opacity = state.data.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        // Skip widgets with no width or no height
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        // Apply transformations
        let rotate = state.style.rotate.get(entity).unwrap_or(&0.0);
        let scaley = state.style.scaley.get(entity).cloned().unwrap_or_default();

        canvas.save();
        canvas.translate(bounds.x + bounds.w / 2.0, bounds.y + bounds.h / 2.0);
        canvas.rotate(rotate.to_radians());
        canvas.translate(-(bounds.x + bounds.w / 2.0), -(bounds.y + bounds.h / 2.0));

        canvas.translate(bounds.x, bounds.y);

        //let pt = canvas.transform().inversed().transform_point(posx + width / 2.0, posy + height / 2.0);
        //canvas.translate(posx + width / 2.0, posy + width / 2.0);
        // canvas.translate(pt.0, pt.1);
        // canvas.scale(1.0, scaley.0);
        // canvas.translate(-pt.0, -pt.1);

        // Apply Scissor
        let mut clip_region = state.data.get_clip_region(entity);
        canvas.scissor(
            clip_region.x - bounds.x,
            clip_region.y - bounds.y,
            clip_region.w,
            clip_region.h,
        );

        let outer_shadow_h_offset = match state
            .style
            .outer_shadow_h_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let outer_shadow_v_offset = match state
            .style
            .outer_shadow_v_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let outer_shadow_blur = match state
            .style
            .outer_shadow_blur
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let outer_shadow_color = state
            .style
            .outer_shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let mut outer_shadow_color: femtovg::Color = outer_shadow_color.into();
        outer_shadow_color.set_alphaf(outer_shadow_color.a * opacity);

        let inner_shadow_h_offset = match state
            .style
            .inner_shadow_h_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let inner_shadow_v_offset = match state
            .style
            .inner_shadow_v_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let inner_shadow_blur = match state
            .style
            .inner_shadow_blur
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let inner_shadow_color = state
            .style
            .inner_shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let mut inner_shadow_color: femtovg::Color = inner_shadow_color.into();
        inner_shadow_color.set_alphaf(inner_shadow_color.a * opacity);

        // Draw outer shadow
        let mut path = Path::new();
        path.rounded_rect_varying(
            0.0 - outer_shadow_blur + outer_shadow_h_offset,
            0.0 - outer_shadow_blur + outer_shadow_v_offset,
            bounds.w + 2.0 * outer_shadow_blur,
            bounds.h + 2.0 * outer_shadow_blur,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        path.rounded_rect_varying(
            0.0,
            0.0,
            bounds.w,
            bounds.h,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        path.solidity(Solidity::Hole);

        let mut paint = Paint::box_gradient(
            0.0 + outer_shadow_h_offset,
            0.0 + outer_shadow_v_offset,
            bounds.w,
            bounds.h,
            border_radius_top_left
                .max(border_radius_top_right)
                .max(border_radius_bottom_left)
                .max(border_radius_bottom_right),
            outer_shadow_blur,
            outer_shadow_color,
            femtovg::Color::rgba(0, 0, 0, 0),
        );

        canvas.fill_path(&mut path, paint);

        let mut path = Path::new();

        if border_radius_bottom_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_bottom_right == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_right == (bounds.w - 2.0 * border_width) / 2.0
        {
            path.circle(
                0.0 + (border_width / 2.0) + (bounds.w - border_width) / 2.0,
                0.0 + (border_width / 2.0) + (bounds.h - border_width) / 2.0,
                bounds.w / 2.0,
            );
        } else {
            // Draw rounded rect
            path.rounded_rect_varying(
                0.0 + (border_width / 2.0),
                0.0 + (border_width / 2.0),
                bounds.w - border_width,
                bounds.h - border_width,
                border_radius_top_left,
                border_radius_top_right,
                border_radius_bottom_right,
                border_radius_bottom_left,
            );
        }

        // Fill with background color
        let mut paint = Paint::color(background_color);

        if let Some(background_image) = state.style.background_image.get(entity) {
            if let Some(image_id) = state.resource_manager.image_ids.get(background_image) {
                match image_id {
                    crate::ImageOrId::Id(id) => {
                        paint = Paint::image(*id, 0.0, 0.0, 100.0, 100.0, 0.0, 1.0);
                    }

                    _=> {}
                }

            }
        }

        // Gradient overrides background color
        if let Some(background_gradient) = state.style.background_gradient.get_mut(entity) {
            let (start_x, start_y, end_x, end_y) = match background_gradient.direction {
                Direction::LeftToRight => (0.0, 0.0, bounds.w, 0.0),
                Direction::TopToBottom => (0.0, 0.0, 0.0, bounds.h),
                _ => (0.0, 0.0, bounds.w, 0.0),
            };

            paint = Paint::linear_gradient_stops(
                start_x,
                start_y,
                end_x,
                end_y,
                background_gradient
                    .get_stops(parent_width)
                    .iter()
                    .map(|stop| {
                        let col: femtovg::Color = stop.1.into();
                        (stop.0, col)
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
        }

        // Fill the quad
        canvas.fill_path(&mut path, paint);

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        canvas.stroke_path(&mut path, paint);

        // Draw inner shadow
        let mut path = Path::new();
        path.rounded_rect_varying(
            0.0 + border_width,
            0.0 + border_width,
            bounds.w - border_width * 2.0,
            bounds.h - border_width * 2.0,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );

        let mut paint = Paint::box_gradient(
            0.0 + inner_shadow_h_offset + border_width,
            0.0 + inner_shadow_v_offset + border_width,
            bounds.w - border_width * 2.0,
            bounds.h - border_width * 2.0,
            border_radius_top_left
                .max(border_radius_top_right)
                .max(border_radius_bottom_left)
                .max(border_radius_bottom_right),
            inner_shadow_blur,
            femtovg::Color::rgba(0, 0, 0, 0),
            inner_shadow_color,
        );
        canvas.fill_path(&mut path, paint);

        // Draw text
        if let Some(text) = state.style.text.get_mut(entity) {
            let font_id = match text.font.as_ref() {
                "sans" => state.fonts.regular.unwrap(),
                "icons" => state.fonts.icons.unwrap(),
                "emoji" => state.fonts.emoji.unwrap(),

                _ => state.fonts.regular.unwrap(),
            };

            // let mut x = posx + (border_width / 2.0);
            // let mut y = posy + (border_width / 2.0);

            let mut x = 0.0;
            let mut y = 0.0;

            let text_string = text.text.to_owned();

            let text_align = state
                .style
                .text_align
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let text_justify = state
                .style
                .text_justify
                .get(entity)
                .cloned()
                .unwrap_or_default();

            let align = match text_justify {
                Justify::Start => {
                    x += padding_left + border_width;
                    Align::Left
                }
                Justify::Center => {
                    x += 0.5 * bounds.w;
                    Align::Center
                }
                Justify::End => {
                    x += bounds.w - padding_right - border_width;
                    Align::Right
                }
            };

            let baseline = match text_align {
                crate::Align::Start => {
                    y += padding_top + border_width;
                    Baseline::Top
                }
                crate::Align::Center => {
                    y += 0.5 * bounds.h;
                    Baseline::Middle
                }
                crate::Align::End => {
                    y += bounds.h - padding_bottom - border_width;
                    Baseline::Bottom
                }
            };

            let mut font_color: femtovg::Color = font_color.into();
            font_color.set_alphaf(font_color.a * opacity);

            let font_size = state.style.font_size.get(entity).cloned().unwrap_or(16.0);

            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id]);
            paint.set_text_align(align);
            paint.set_text_baseline(baseline);
            paint.set_anti_alias(false);

            canvas.fill_text(x, y, &text_string, paint);
        }

        canvas.translate(-bounds.x, -bounds.y);
        canvas.restore();
    }
}

impl<T> EventHandler for T
where
    T: Widget + 'static,
{
    fn on_event_(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        <T as Widget>::on_event(self, state, entity, event);
    }

    fn on_draw_(&mut self, state: &mut State, entity: Entity, canvas: &mut RenderCanvas) {
        <T as Widget>::on_draw(self, state, entity, canvas);
    }
}
