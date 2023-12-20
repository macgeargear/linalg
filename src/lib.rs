use std::fs;

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];
        return Self { rows, cols, data };
    }
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
        let mut mat: Vec<Vec<f64>> = Vec::new();
        for rows in content.lines() {
            let mut row: Vec<f64> = Vec::new();
            let entries: Vec<&str> = rows.split_whitespace().collect();
            for ent in entries {
                row.push(ent.parse::<f64>().unwrap());
            }
            mat.push(row);
        }
        let rows = mat.len();
        let cols = mat[0].len();
        return Self {
            rows,
            cols,
            data: mat,
        };
    }
    // TODO: create matrix from string
    pub fn from_string(input: &str) -> Self {
        let mut data: Vec<Vec<f64>> = Vec::new();
        let rows: Vec<&str> = input.split(";").collect();
        // for row in rows {
        //     let entries: Vec<&str>
        // }
    }
}

