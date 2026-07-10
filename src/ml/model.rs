use crate::ml::{ConfigurationLock, OutputColumnData};
use forust_ml::gradientbooster::ContributionsMethod;
use forust_ml::objective::ObjectiveType;
use forust_ml::{GradientBooster, Matrix};
use spdlog::{error, info};

pub fn categorical_to_one_hot(entries: &Vec<String>) -> Vec<Vec<f64>> {
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
        out.push(one_hot_column);
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
            for mut column in categorical_one_hot {
                array.append(&mut column);
            }
        }
        for col in &data.numerical_columns {
            for entry in &col.1 {
                array.push(*entry)
            }
        }

        let y = match data.output_data {
            OutputColumnData::Regressive(array) => array,
            OutputColumnData::BinaryClassificatory(array) => get_mapped_binary(array),
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
            .set_learning_rate(0.01)
            .set_objective_type(ObjectiveType::SquaredLoss)
            .set_iterations(500)
            .set_max_depth(10);
        info!("Starting training");
        if let Err(e) = model.fit_unweighted(&matrix, y.as_slice(), None) {
            error!("Model training failed: {}", e);
        }
        info!("Training complete");
        info!("Starting Shapley value generation");
        let shap_values = model.predict_contributions(&matrix, ContributionsMethod::Shapley, false);
        info!("Shapley values computed");
    });
}
