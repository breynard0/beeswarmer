pub mod beeswarm_draw;
pub mod beeswarm_prep;
pub mod model;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum OutputColumnData {
    Regressive(Vec<f64>),
    BinaryClassificatory(Vec<String>),
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ConfigurationLock {
    pub numerical_columns: Vec<(String, Vec<f64>)>,
    pub categorical_columns: Vec<(String, Vec<String>)>,
    pub output_name: String,
    pub output_data: OutputColumnData,
}