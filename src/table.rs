use crate::{TableColumnSlint, TableColumnTypeSlint, TableDataSlint};
use slint::{Model, ModelRc, SharedString, VecModel};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct TableData {
    pub columns: Vec<TableColumn>,
    pub excluded_rows: Vec<u32>,
    pub number_rows: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum TableColumnType {
    Unset,
    Numerical,
    Categorical,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TableColumn {
    pub title: String,
    pub column_type: TableColumnType,
    pub column_entries: Vec<String>,
    pub enabled: bool,
}

impl TableData {
    pub fn from_csv(csv: String) -> Result<Self, String> {
        let mut lines = csv.lines().collect::<Vec<_>>();
        let column_headers_string = lines.get(0).ok_or("Bad CSV headers")?;

        let column_headers = Self::split_csv(column_headers_string);

        let mut columns = vec![];

        for header in column_headers {
            columns.push(TableColumn {
                title: header.to_string(),
                column_type: TableColumnType::Unset,
                column_entries: vec![],
                enabled: true,
            });
        }

        lines.remove(0);
        for (line_number, line) in lines.iter().enumerate() {
            for (index, entry) in Self::split_csv(line).iter().enumerate() {
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

    fn split_csv(input_string: &&str) -> Vec<String> {
        let mut split_out = vec![];
        let mut cur_string = String::new();
        let mut in_quotes = false;
        for c in input_string.chars() {
            if c == '"' {
                if in_quotes {
                    in_quotes = false;
                } else {
                    in_quotes = true;
                }
                continue;
            }

            if c == ',' {
                if in_quotes {
                    cur_string.push(c);
                } else {
                    split_out.push(cur_string);
                    cur_string = String::new();
                }
                continue;
            }

            cur_string.push(c);
        }
        split_out.push(cur_string);
        split_out
    }
}

impl From<TableColumnSlint> for TableColumn {
    fn from(value: TableColumnSlint) -> Self {
        Self {
            title: value.title.to_string(),
            column_type: match value.column_type {
                TableColumnTypeSlint::Unset => TableColumnType::Unset,
                TableColumnTypeSlint::Numerical => TableColumnType::Numerical,
                TableColumnTypeSlint::Categorical => TableColumnType::Categorical,
            },
            column_entries: value
                .column_entries
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            enabled: value.enabled,
        }
    }
}

impl From<TableDataSlint> for TableData {
    fn from(value: TableDataSlint) -> Self {
        Self {
            columns: value.columns.iter().map(|x| TableColumn::from(x)).collect(),
            excluded_rows: value.excluded_rows.iter().map(|x| x as u32).collect(),
            number_rows: value.number_rows as u32,
        }
    }
}

impl Into<TableColumnSlint> for TableColumn {
    fn into(self) -> TableColumnSlint {
        TableColumnSlint {
            column_entries: ModelRc::new(VecModel::from(
                self.column_entries
                    .iter()
                    .map(|s| SharedString::from(s))
                    .collect::<Vec<_>>(),
            )),
            column_type: match self.column_type {
                TableColumnType::Unset => TableColumnTypeSlint::Unset,
                TableColumnType::Numerical => TableColumnTypeSlint::Numerical,
                TableColumnType::Categorical => TableColumnTypeSlint::Categorical,
            },
            enabled: self.enabled,
            title: self.title.into(),
        }
    }
}

impl Into<TableDataSlint> for TableData {
    fn into(self) -> TableDataSlint {
        TableDataSlint {
            columns: ModelRc::new(VecModel::from(
                self.columns
                    .iter()
                    .map(|x| x.clone().into())
                    .collect::<Vec<_>>(),
            )),
            excluded_rows: ModelRc::new(VecModel::from(
                self.excluded_rows
                    .iter()
                    .map(|x| *x as i32)
                    .collect::<Vec<_>>(),
            )),
            number_rows: self.number_rows as i32,
        }
    }
}
