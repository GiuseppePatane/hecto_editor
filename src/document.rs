use crate::row::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}
impl Document {
    /// function to open a new document
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename).expect("Failed to open the file");
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self { rows })
    }
    /// function to get the row at a particular index
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
