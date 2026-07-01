#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableData {
    pub columns: Vec<TableColumn>,
    pub excluded_rows: Vec<u32>,
    pub number_rows: u32
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum TableColumnType {
    Unset,
    Numerical,
    Categorical,
    Excluded,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableColumn {
    pub title: String,
    pub column_type: TableColumnType,
    pub column_entries: Vec<String>,
}

impl TableData {
    pub fn from_csv(csv: String) -> Result<Self, String> {
        let mut lines = csv.lines().collect::<Vec<_>>();
        let column_headers = lines
            .get(0)
            .ok_or("Bad CSV headers")?
            .split(',')
            .collect::<Vec<_>>();

        let mut columns = vec![];

        for header in column_headers {
            columns.push(TableColumn {
                title: header.to_string(),
                column_type: TableColumnType::Unset,
                column_entries: vec![],
            });
        }

        lines.remove(0);
        for (line_number, line) in lines.iter().enumerate() {
            for (index, entry) in line.split(',').enumerate() {
                columns
                    .get_mut(index)
                    .ok_or(format!("Bad CSV at line {line_number}"))?
                    .column_entries
                    .push(entry.to_string());
            }
        }

        Ok(TableData {
            columns,
            number_rows: lines.len() as u32,
            excluded_rows: vec![],
        })
    }
}
