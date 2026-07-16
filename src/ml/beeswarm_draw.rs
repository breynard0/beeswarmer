use crate::ml::beeswarm_prep::{DrawRow, JETBRAINS_MONO, ScaleData};
use piet::kurbo::{Circle, Line, Rect, Size};
use piet::{
    Color, Error, RenderContext, StrokeStyle, Text, TextLayout, TextLayoutBuilder, UnitPoint,
};

pub fn beeswarm_draw(rows: Vec<DrawRow>, scale_data: ScaleData) -> Result<Vec<u8>, Error> {
    const ROW_HEIGHT: f64 = 80.0;
    let longest_title = rows
        .iter()
        .max_by(|a, b| a.title_width.total_cmp(&b.title_width))
        .unwrap()
        .title_width;
    let side_margin = ROW_HEIGHT / 16.0;
    let between_margin = ROW_HEIGHT / 4.0;
    let content_width = ROW_HEIGHT * 16.0;
    let dot_span_width = content_width * 0.8;
    let axis_label_thickness = ROW_HEIGHT * 2.0;
    let top_margin = ROW_HEIGHT;
    let row_margin = ROW_HEIGHT / 2.0;
    let line_thickness = 4.0;
    let colour_axis_margin = ROW_HEIGHT / 2.0;
    let colour_axis_height = ROW_HEIGHT;

    let bin_size = 0.01;
    let dot_size = ROW_HEIGHT / 6.0;

    let size = Size::new(
        2.0 * side_margin + between_margin + content_width + axis_label_thickness + longest_title,
        (ROW_HEIGHT + row_margin) * rows.len() as f64
            + axis_label_thickness
            + top_margin
            + colour_axis_margin
            + colour_axis_height,
    );

    let mut piet = piet_svg::RenderContext::new(size);
    let jetbrains_mono = piet
        .text()
        .load_font(JETBRAINS_MONO)
        .expect("Couldn't load embedded font");

    piet.fill(Rect::new(0.0, 0.0, size.width, size.height), &Color::WHITE);

    let mut center_line_location;

    for (idx, draw_row) in rows.iter().enumerate() {
        let x = side_margin + longest_title + between_margin;
        let y = (ROW_HEIGHT + row_margin) * idx as f64 + top_margin;

        // Title
        let text = piet.text();
        let layout = text
            .new_text_layout(draw_row.title.clone())
            .font(jetbrains_mono.clone(), 32.0)
            .text_color(Color::BLACK)
            .build()
            .unwrap();
        piet.draw_text(
            &layout,
            (
                side_margin + (longest_title - layout.image_bounds().width()),
                y + (ROW_HEIGHT - layout.image_bounds().height()) / 2.0,
            ),
        );

        // Center lines
        center_line_location =
            x + scale_data.line_location * dot_span_width + (content_width - dot_span_width) / 2.0;
        piet.fill(
            Rect::new(
                center_line_location - line_thickness / 2.0,
                y,
                center_line_location + line_thickness / 2.0,
                y + ROW_HEIGHT + row_margin,
            ),
            &Color::BLACK,
        );

        let dash_pattern = &[line_thickness, line_thickness * 2.0];
        let mut stroke_style = StrokeStyle::new();
        stroke_style.set_dash_pattern(*dash_pattern);

        piet.stroke_styled(
            Line::new(
                (x, y + ROW_HEIGHT / 2.0 - line_thickness / 2.0),
                (
                    x + content_width,
                    y + ROW_HEIGHT / 2.0 - line_thickness / 2.0,
                ),
            ),
            &Color::GRAY,
            line_thickness,
            &stroke_style,
        );

        // Dots
        let mut dots = draw_row.dots.clone();
        dots.sort_by(|a, b| a.0.total_cmp(&b.0));
        let bins_array = dots
            .chunk_by(|a, b| (a.0 - b.0).abs() < bin_size && a.1 == b.1)
            .collect::<Vec<_>>();
        for bin in bins_array {
            for (idx, (position, colour)) in bin.iter().enumerate() {
                let max_no_touching = (ROW_HEIGHT / dot_size).floor() as usize;
                let dot_y = match bin.len() > max_no_touching {
                    true => idx as f64 * ROW_HEIGHT / bin.len() as f64 + dot_size / 2.0,
                    false => {
                        let total_height = dot_size * bin.len() as f64;
                        let starting_y = (ROW_HEIGHT - total_height) / 2.0;
                        starting_y + idx as f64 * dot_size
                    }
                };
                piet.fill(
                    Circle::new(
                        (
                            x + *position * dot_span_width + (content_width - dot_span_width) / 2.0,
                            y + dot_y + dot_size / 4.0,
                        ),
                        dot_size / 2.0,
                    ),
                    colour,
                );
            }
        }
    }

    // Color
    let colour_axis_start_y =
        top_margin + (ROW_HEIGHT + row_margin) * rows.len() as f64 + colour_axis_margin;
    let colour_axis_start_x = side_margin + longest_title + between_margin;
    let bounds = Rect::new(
        colour_axis_start_x,
        colour_axis_start_y,
        colour_axis_start_x + content_width,
        colour_axis_start_y + colour_axis_height / 2.0,
    );
    piet.fill(
        bounds,
        &piet::LinearGradient::new(UnitPoint::LEFT, UnitPoint::RIGHT, (Color::BLUE, Color::RED)),
    );
    let text = piet.text();
    let layout = text
        .new_text_layout("Low")
        .font(jetbrains_mono.clone(), colour_axis_height / 2.0 - 2.0)
        .text_color(Color::BLACK)
        .build()
        .unwrap();
    piet.draw_text(
        &layout,
        (
            colour_axis_start_x,
            colour_axis_start_y + colour_axis_height / 2.0 + 2.0,
        ),
    );
    let text = piet.text();
    let layout = text
        .new_text_layout("High")
        .font(jetbrains_mono.clone(), colour_axis_height / 2.0 - 2.0)
        .text_color(Color::BLACK)
        .build()
        .unwrap();
    piet.draw_text(
        &layout,
        (
            colour_axis_start_x + content_width - layout.image_bounds().width(),
            colour_axis_start_y + colour_axis_height / 2.0 + 2.0,
        ),
    );

    piet.finish()?;
    let mut out_buffer = vec![];
    let _ = piet.write(&mut out_buffer);
    Ok(out_buffer)
}
