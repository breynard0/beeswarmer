use crate::BeeswarmTheme;
use crate::ml::ConfigurationLock;
use crate::ml::model::categorical_to_one_hot;
use piet::kurbo::Size;
use piet::{Color, Error, RenderContext, Text, TextLayout, TextLayoutBuilder};
use crate::callbacks::result::fix_hex;

pub const JETBRAINS_MONO: &[u8] = include_bytes!("../../resources/JetBrainsMono-Regular.ttf");

fn to_snake_case_and_shorten(text: &String) -> String {
    text.clone()
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .split("_")
        .take(3)
        .map(|s| format!("{s}_").chars().collect::<Vec<char>>())
        .flatten()
        .collect::<String>()
        .trim_end_matches('_')
        .to_string()
}

fn lerp(start: f64, end: f64, progress: f64) -> f64 {
    let delta = (end - start) * progress;
    start + delta
}

fn lerp_colour(start: Color, end: Color, progress: f64) -> Color {
    let start = start.as_rgba();
    let end = end.as_rgba();

    Color::rgba(
        lerp(start.0, end.0, progress),
        lerp(start.1, end.1, progress),
        lerp(start.2, end.2, progress),
        lerp(start.3, end.3, progress),
    )
}

pub struct DrawRow {
    pub title: String,
    pub title_width: f64,
    pub dots: Vec<(f64, Color)>,
}

pub struct ScaleData {
    pub line_location: f64,
}

pub fn beeswarm_prep(
    contribution_matrix: forust_ml::Matrix<f64>,
    configuration_lock: &ConfigurationLock,
    theme: &BeeswarmTheme,
) -> Result<(Vec<DrawRow>, ScaleData), Error> {
    let max_contribution = contribution_matrix
        .data
        .iter()
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(&0.0);
    let min_contribution = contribution_matrix
        .data
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or(&0.0);

    let mut rows: Vec<DrawRow> = vec![];

    let mut piet_sampler = piet_svg::RenderContext::new(Size::INFINITY);
    let jetbrains_mono = piet_sampler
        .text()
        .load_font(JETBRAINS_MONO)
        .expect("Couldn't load embedded font");

    let mut feature_values = vec![];
    for col in &configuration_lock.categorical_columns {
        let one_hot_columns = categorical_to_one_hot(&col.1);
        for one_hot_col in &one_hot_columns {
            let title = format!(
                "{}_{}",
                to_snake_case_and_shorten(&col.0),
                to_snake_case_and_shorten(&one_hot_col.0)
            );
            feature_values.push((title, one_hot_col.1.clone()));
        }
    }
    for col in &configuration_lock.numerical_columns {
        feature_values.push((to_snake_case_and_shorten(&col.0), col.1.clone()))
    }

    for idx in 0..contribution_matrix.cols {
        let (title, features) = &feature_values[idx];
        let contributions = contribution_matrix.get_col(idx);

        let text = piet_sampler.text();
        let layout = text
            .new_text_layout(title.clone())
            .font(jetbrains_mono.clone(), 32.0)
            .build()
            .unwrap();

        let feature_max = *features
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);
        let feature_min = *features
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);
        let mut dots = vec![];
        for dot_idx in 0..features.len() {
            let feature_value = features[dot_idx];
            let contribution_value = contributions[dot_idx];

            let progress = (feature_value - feature_min) / (feature_max - feature_min);
            let colour = lerp_colour(
                Color::from_hex_str(&fix_hex(&theme.low)).unwrap_or(Color::BLUE),
                Color::from_hex_str(&fix_hex(&theme.high)).unwrap_or(Color::RED),
                progress,
            );

            let position = (contribution_value + min_contribution.abs())
                / (max_contribution + min_contribution.abs());

            dots.push((position, colour));
        }

        rows.push(DrawRow {
            title: title.clone(),
            title_width: layout.image_bounds().width(),
            dots,
        })
    }

    Ok((
        rows,
        ScaleData {
            line_location: min_contribution.abs() / (min_contribution.abs() + max_contribution),
        },
    ))
}
