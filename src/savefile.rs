#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveFile {
    pub raw_csv: String,
    pub table_data: TableData
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableData {
    pub columns: Vec<TableColumn>,
    pub excluded_rows: Vec<u32>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum TableColumnType {
    Numerical,
    CategoricalOneHot,
    Scoring,
    Excluded,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableColumn {
    pub title: String,
    pub column_type: TableColumnType,
    pub column_entries: Vec<String>
}