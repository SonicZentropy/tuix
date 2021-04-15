use crate::{events::{Event, EventManager, Message}, state, RenderCanvas};

use crate::builder::Builder;

use crate::{Entity, Hierarchy, State};

use std::collections::{HashMap, VecDeque};

use femtovg::{
    renderer::OpenGl, Align, Baseline, FillRule, FontId, ImageFlags, ImageId, LineCap, LineJoin,
    Paint, Path, Renderer, Solidity,
};

use crate::style::{Direction, Justify, Units, Visibility};

use std::any::{Any, TypeId};

pub(crate) trait EventHandler: Any {
    // Called when events are flushed
    fn on_event_(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}

    //fn on_draw_(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas);
    // Called when a redraw occurs

    fn on_draw_(&mut self, state: &mut State, entity: Entity, canvas: &mut RenderCanvas) {
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

        let posx = state.data.get_posx(entity);
        let posy = state.data.get_posy(entity);
        let width = state.data.get_width(entity);
        let height = state.data.get_height(entity);

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
        if width == 0.0 || height == 0.0 {
            return;
        }

        // Apply transformations
        let rotate = state.style.rotate.get(entity).unwrap_or(&0.0);
        let scaley = state.style.scaley.get(entity).cloned().unwrap_or_default();

        canvas.save();
        canvas.translate(posx + width / 2.0, posy + height / 2.0);
        canvas.rotate(rotate.to_radians());
        canvas.translate(-(posx + width / 2.0), -(posy + height / 2.0));

        canvas.translate(posx, posy);

        //let pt = canvas.transform().inversed().transform_point(posx + width / 2.0, posy + height / 2.0);
        //canvas.translate(posx + width / 2.0, posy + width / 2.0);
        // canvas.translate(pt.0, pt.1);
        // canvas.scale(1.0, scaley.0);
        // canvas.translate(-pt.0, -pt.1);

        // Apply Scissor
        let mut clip_region = state.data.get_clip_region(entity);
        canvas.scissor(
            clip_region.x - posx,
            clip_region.y - posy,
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
            width + 2.0 * outer_shadow_blur,
            height + 2.0 * outer_shadow_blur,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        path.rounded_rect_varying(
            0.0,
            0.0,
            width,
            height,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        path.solidity(Solidity::Hole);

        let mut paint = Paint::box_gradient(
            0.0 + outer_shadow_h_offset,
            0.0 + outer_shadow_v_offset,
            width,
            height,
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

        if border_radius_bottom_left == (width - 2.0 * border_width) / 2.0
            && border_radius_bottom_right == (width - 2.0 * border_width) / 2.0
            && border_radius_top_left == (width - 2.0 * border_width) / 2.0
            && border_radius_top_right == (width - 2.0 * border_width) / 2.0
        {
            path.circle(
                0.0 + (border_width / 2.0) + (width - border_width) / 2.0,
                0.0 + (border_width / 2.0) + (height - border_width) / 2.0,
                width / 2.0,
            );
        } else {
            // Draw rounded rect
            path.rounded_rect_varying(
                0.0 + (border_width / 2.0),
                0.0 + (border_width / 2.0),
                width - border_width,
                height - border_width,
                border_radius_top_left,
                border_radius_top_right,
                border_radius_bottom_right,
                border_radius_bottom_left,
            );
        }

        // Fill with background color
        let mut paint = Paint::color(background_color);

        // Gradient overrides background color
        if let Some(background_gradient) = state.style.background_gradient.get_mut(entity) {
            let (start_x, start_y, end_x, end_y) = match background_gradient.direction {
                Direction::LeftToRight => (0.0, 0.0, width, 0.0),
                Direction::TopToBottom => (0.0, 0.0, 0.0, height),
                _ => (0.0, 0.0, width, 0.0),
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
            width - border_width * 2.0,
            height - border_width * 2.0,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );

        let mut paint = Paint::box_gradient(
            0.0 + inner_shadow_h_offset + border_width,
            0.0 + inner_shadow_v_offset + border_width,
            width - border_width * 2.0,
            height - border_width * 2.0,
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
                    x += 0.5 * width;
                    Align::Center
                }
                Justify::End => {
                    x += width - padding_right - border_width;
                    Align::Right
                }
            };

            let baseline = match text_align {
                crate::Align::Start => {
                    y += padding_top + border_width;
                    Baseline::Top
                }
                crate::Align::Center => {
                    y += 0.5 * height;
                    Baseline::Middle
                }
                crate::Align::End => {
                    y += height - padding_bottom - border_width;
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

        canvas.translate(-posx, -posy);
        canvas.restore();

        /*
        window.context.borrow_mut().frame(
            (
                state.transform.get_width(state.root),
                state.transform.get_height(state.root),
            ),
            1.0 as f32,
            |mut frame| {

                let zoom = Transform::new().scale(state.transform.get_zoom_scale(entity), state.transform.get_zoom_scale(entity));
                frame.transformed(Transform::new(), |frame| {
                    if entity == Entity::new(0, 0) {
                        return;
                    }

                    // Skip invisible widgets
                    if state.transform.get_visibility(entity) == Visibility::Invisible {
                        //println!("Entity: {} is invisible", entity);
                        return;
                    }

                    if state.transform.get_opacity(entity) == 0.0 {
                        //println!("Entity: {} has 0 opacity", entity);
                        return;
                    }

                    let posx = state.transform.get_posx(entity);
                    let posy = state.transform.get_posy(entity);
                    let width = state.transform.get_width(entity);
                    let height = state.transform.get_height(entity);

                    //println!("DRAW: {} {} {} {} {}", entity, posx, posy, width, height);

                    // Skip widgets with no width or no height
                    if width == 0.0 || height == 0.0 {
                        return;
                    }

                    let parent = state.hierarchy.get_parent(entity).unwrap();

                    let parent_width = state.transform.get_width(parent);

                    // let clip_entity = state
                    //     .style
                    //     .clip_widget
                    //     .get(entity)
                    //     .cloned()
                    //     .unwrap_or_default();

                    let clip_entity = state.transform.get_clip_widget(entity);

                    //let clip_entity = state.root;

                    let clip_posx = state.transform.get_posx(clip_entity);
                    let clip_posy = state.transform.get_posy(clip_entity);
                    let clip_width = state.transform.get_width(clip_entity);
                    let clip_height = state.transform.get_height(clip_entity);

                    //let mut path_opts: PathOptions = Default::default();

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

                    let padding_top = match state.style.padding_top.get(entity).unwrap_or(&Units::Auto)
                    {
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

                    let rotate = state.style.rotate.get(entity).unwrap_or(&0.0);

                    //let rotate = &10.0;

                    let trans1 = Transform::new().translate(-posx - width / 2.0, -posy - height / 2.0);
                    let rotation = Transform::new().rotate((*rotate as f32).to_radians());
                    let trans2 = Transform::new().translate(posx + width / 2.0, posy + height / 2.0);

                    let transform = trans1 * rotation * trans2;
                    //let rotation = Transform::new().translate(50.0, 0.0);

                    let path_opts = PathOptions {
                        clip: Clip::Scissor(Scissor {
                            x: clip_posx,
                            y: clip_posy,
                            width: clip_width,
                            height: clip_height,
                            transform: None,

                        }),
                        transform: Some(transform),
                        ..Default::default()
                    };

                    let background_color = state
                        .style
                        .background_color
                        .get(entity)
                        .cloned()
                        .unwrap_or_default();

                    let border_color = state
                        .style
                        .border_color
                        .get(entity)
                        .cloned()
                        .unwrap_or_default();

                    let border_radius = state
                        .style
                        .border_radius
                        .get(entity)
                        .cloned()
                        .unwrap_or_default();

                    let border_radius_top_left = match border_radius.top_left {
                        Units::Pixels(val) => val,
                        Units::Percentage(val) => parent_width * val,
                        _ => 0.0,
                    };

                    let border_radius_top_right = match border_radius.top_right {
                        Units::Pixels(val) => val,
                        Units::Percentage(val) => parent_width * val,
                        _ => 0.0,
                    };

                    let border_radius_bottom_left = match border_radius.bottom_left {
                        Units::Pixels(val) => val,
                        Units::Percentage(val) => parent_width * val,
                        _ => 0.0,
                    };

                    let border_radius_bottom_right = match border_radius.bottom_right {
                        Units::Pixels(val) => val,
                        Units::Percentage(val) => parent_width * val,
                        _ => 0.0,
                    };

                    let opacity = state.transform.get_opacity(entity);

                    let mut background_color: nanovg::Color = background_color.into();
                    //let mut background_color: nvg::Color = background_color.into();
                    background_color.set_alpha(background_color.alpha() * opacity);
                    //background_color.a = background_color.a * opacity;

                    let mut border_color: nanovg::Color = border_color.into();
                    //let mut border_color: nvg::Color = border_color.into();
                    border_color.set_alpha(border_color.alpha() * opacity);
                    //border_color.a = border_color.a * opacity;

                    let border_width = state
                        .style
                        .border_width
                        .get(entity)
                        .cloned()
                        .unwrap_or_default();

                    frame.path(
                        |path| {
                            path.rounded_rect_varying(
                                (posx, posy),
                                (width, height),
                                (border_radius_top_left, border_radius_top_right),
                                (border_radius_bottom_left, border_radius_bottom_right),
                            );
                            // if let Some(background_image) = state.style.background_image.get(entity) {
                            //     let image = images.get(background_image).unwrap();
                            //     path.fill(
                            //         ImagePattern {
                            //             image: &image,
                            //             origin: (posx, posy),
                            //             size: (width, height),
                            //             angle: 0.0,
                            //             alpha: opacity,
                            //         },
                            //         Default::default(),
                            //     );
                            // } else {
                                path.fill(background_color, Default::default());
                            //}

                            path.stroke(
                                border_color,
                                StrokeOptions {
                                    width: border_width,
                                    ..Default::default()
                                },
                            );
                        },
                        path_opts,
                    );

                    if let Some(text) = state.style.text.get_mut(entity) {
                        let sans =
                            Font::find(frame.context(), "Roboto-Regular").expect("Failed to load font");
                        let icons = Font::find(frame.context(), "Icons").expect("Failed to load font");

                        let font = match text.font.as_ref() {
                            "Sans" => sans,
                            "Icons" => icons,
                            _ => sans,
                        };
                        let mut align = Alignment::new();

                        let mut x = posx;
                        let mut y = posy;

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

                        match text_justify {
                            Justify::Start => {
                                align = align.left();
                                x += padding_left;
                            }
                            Justify::Center => {
                                align = align.center();
                                x += 0.5 * width;
                            }
                            Justify::End => {
                                align = align.right();
                                x += width - padding_right;
                            }
                        }

                        match text_align {
                            crate::Align::Start => {
                                align = align.top();
                                y += padding_top;
                            }
                            crate::Align::Center => {
                                align = align.middle();
                                y += 0.5 * height;
                            }
                            crate::Align::End => {
                                align = align.bottom();
                                y += height - padding_bottom;
                            }
                        }

                        //x += text.indent;

                        let mut font_color: nanovg::Color = text.font_color.into();
                        font_color.set_alpha(font_color.alpha() * opacity);

                        let text_options = TextOptions {
                            color: font_color,
                            size: text.font_size,
                            align: align,
                            clip: Clip::Scissor(Scissor {
                                x: clip_posx,
                                y: clip_posy,
                                width: clip_width,
                                height: clip_height,
                                transform: None,
                            }),
                            transform: Some(transform),
                            //line_height: 1.0,
                            ..Default::default()
                        };

                        frame.text(font, (x, y), &text_string, text_options);
                    }
                });




                //     context.begin_path();
                //     context.reset_transform();
                //     context.translate(posx+width/2.0, posy+height/2.0);
                //     context.rotate(rotate * std::f32::consts::PI / 180.0);
                //     context.translate(-posx-width/2.0,-posy-height/2.0);
                //     context.rounded_rect_varying((posx, posy, width, height), border_radius_top_left, border_radius_top_right, border_radius_bottom_right, border_radius_bottom_left);
                //     context.fill_paint(background_color);
                //     context.stroke_width(border_width);
                //     context.stroke_paint(border_color);
                //     context.fill().unwrap();
                //     context.stroke().unwrap();

                //     if let Some(text) = state.style.text.get_mut(entity) {

                //         let mut font_color: nvg::Color = text.font_color.into();
                //         font_color.a = font_color.a * opacity;

                //         context.fill_paint(font_color);
                //         match text.font.as_ref() {
                //             "Sans" => {context.font("roboto");}
                //             "Icons" => {context.font("entypo");}
                //             _=> {}
                //         }
                //         //context.reset_transform();
                //         //context.rotate(45.0 * std::f32::consts::PI / 180.0);
                //         context.font_size(text.font_size);
                //         context.begin_path();

                //         let text_align = state.style.text_align.get(entity).cloned().unwrap_or_default();
                //         let text_justify = state.style.text_justify.get(entity).cloned().unwrap_or_default();

                //         let mut alignment = Align::empty();

                //         let mut x = posx;
                //         let mut y = posy;

                //         match text_align {
                //             crate::Align::Start => {
                //                 alignment.insert(Align::TOP);
                //                 y += padding_top;
                //             }
                //             crate::Align::Center => {
                //                 alignment.insert(Align::MIDDLE);
                //                 y += 0.5 * height;
                //             }
                //             crate::Align::End => {
                //                 alignment.insert(Align::BOTTOM);
                //                 y += height - padding_bottom;
                //             }
                //         }

                //         match text_justify {
                //             crate::Justify::Start => {
                //                 alignment.insert(Align::LEFT);
                //                 x += padding_left;
                //             }
                //             crate::Justify::Center => {
                //                 alignment.insert(Align::CENTER);
                //                 x += 0.5 * width;
                //             }
                //             crate::Justify::End => {
                //                 alignment.insert(Align::RIGHT);
                //                 x += width - padding_right;
                //             }
                //         }

                //         context.text_align(alignment);
                //         context.text((x, y), &text.text);

                //         context.fill().unwrap();
                //     }
            },
        );
        */
    }
}

impl dyn EventHandler {
    // Check if a message is a certain type
    pub fn is<T: EventHandler + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    // Casts a message to the specified type if the message is of that type
    pub fn downcast<T>(&mut self) -> Option<&mut T>
    where
        T: EventHandler + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn EventHandler as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: EventHandler + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn EventHandler as *const T)) }
        } else {
            None
        }
    }
}
