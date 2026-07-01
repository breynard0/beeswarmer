use crate::table::TableData;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveFile {
    pub table_data: TableData
}