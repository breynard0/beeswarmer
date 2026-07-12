use crate::ml::beeswarm_draw::beeswarm_draw;
use crate::ml::beeswarm_prep::{JETBRAINS_MONO, beeswarm_prep};
use crate::ml::{ConfigurationLock, OutputColumnData};
use forust_ml::gradientbooster::ContributionsMethod;
use forust_ml::metric::Metric;
use forust_ml::objective::ObjectiveType;
use forust_ml::{GradientBooster, Matrix};
use resvg::usvg::Options;
use spdlog::{error, info};
use std::sync::Arc;

pub fn categorical_to_one_hot(entries: &Vec<String>) -> Vec<(String, Vec<f64>)> {
    let mut out = vec![];

    let mut unique_checker = vec![];

    for value in entries {
        if unique_checker.contains(value) {
            continue;
        }
        unique_checker.push(value.clone());

        let mut one_hot_column = vec![];

        for inner in entries {
            if *inner == *value {
                one_hot_column.push(1.0);
            } else {
                one_hot_column.push(0.0);
            }
        }
        out.push((value.clone(), one_hot_column));
    }

    out
}

pub fn get_mapped_binary(column: Vec<String>) -> Vec<f64> {
    let mut out = vec![];
    let first = &column[0];

    for entry in &column {
        if *entry == *first {
            out.push(1.0)
        } else {
            out.push(0.0)
        }
    }

    out
}

pub fn gen_model(data: ConfigurationLock) {
    std::thread::spawn(move || {
        info!("Opened new thread to train model");

        // Get all data into one contiguous array then build matrix
        let mut array: Vec<f64> = vec![];
        let mut categorical_cols_count = 0;
        for col in &data.categorical_columns {
            let categorical_one_hot = categorical_to_one_hot(&col.1);
            categorical_cols_count += categorical_one_hot.len();
            for (_, mut column) in categorical_one_hot {
                array.append(&mut column);
            }
        }
        for col in &data.numerical_columns {
            for entry in &col.1 {
                array.push(*entry)
            }
        }

        let y = match &data.output_data {
            OutputColumnData::Regressive(array) => array.clone(),
            OutputColumnData::BinaryClassificatory(array) => get_mapped_binary(array.clone()),
        };

        let matrix = Matrix::new(
            array.as_slice(),
            y.len(),
            categorical_cols_count + data.numerical_columns.len(),
        );

        info!("Matrix generation successful. Printing:");
        for idx in 0..matrix.rows {
            info!("{:?}", matrix.get_row(idx));
        }
        info!("Dependent data:");
        info!("{:?}", y);

        let mut model = GradientBooster::default()
            .set_objective_type(ObjectiveType::SquaredLoss)
            .set_evaluation_metric(Some(Metric::LogLoss))
            .set_learning_rate(0.01)
            .set_iterations(100)
            .set_max_depth(10);
        info!("Starting training");
        if let Err(e) = model.fit_unweighted(&matrix, y.as_slice(), None) {
            error!("Model training failed: {}", e);
        }
        info!("Training complete");
        info!("Starting Shapley value generation");
        let mut shap_values_raw =
            model.predict_contributions(&matrix, ContributionsMethod::Shapley, false);

        let shap_values_second_raw = shap_values_raw
            .iter()
            .enumerate()
            .filter(|(idx, val)| (idx + 1) % (shap_values_raw.len() / y.len()) != 0)
            .map(|(idx, val)| *val)
            .collect::<Vec<_>>();

        let mut shap_values = vec![];
        let mut iteration = 0;
        while iteration < 24 {
            let mut bounce = 0;
            while bounce < 20 {
                shap_values.push(
                    shap_values_second_raw
                        [iteration + (shap_values_second_raw.len() / y.len()) * bounce],
                );
                bounce += 1;
            }
            iteration += 1;
        }

        let contribution_matrix =
            Matrix::new(shap_values.as_slice(), y.len(), shap_values.len() / y.len());
        let a = contribution_matrix.get_col(1);

        info!("Shapley values computed");
        info!("Starting beeswarm plot generation");
        let (beeswarm_prep, scale_data) = beeswarm_prep(contribution_matrix, &data).unwrap();
        let beeswarm_raw = beeswarm_draw(beeswarm_prep, scale_data).unwrap();
        info!("SVG Generated");

        let mut fontdb = resvg::usvg::fontdb::Database::new();
        let _ = fontdb.load_font_data(JETBRAINS_MONO.to_vec());
        let tree = resvg::usvg::Tree::from_data(
            beeswarm_raw.as_slice(),
            &Options {
                fontdb: Arc::new(fontdb),
                ..Default::default()
            },
        )
        .unwrap();
        let pixmap_size = tree.size().to_int_size();
        let mut pixmap =
            resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::default(),
            &mut pixmap.as_mut(),
        );
        // pixmap.save_png("out.png").unwrap();
        std::fs::write("out.svg", beeswarm_raw).unwrap();
        info!("Beeswarm plot generation complete");
    });
}
