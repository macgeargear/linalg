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
        Self { rows, cols, data }
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
        Self {
            rows,
            cols,
            data: mat,
        }
    }

    // TODO: create matrix from string
    pub fn from_string(input: &str) -> Self {
        let mut data: Vec<Vec<f64>> = Vec::new();
        let rows: Vec<&str> = input.split(";").collect();
        for row in rows {
            let entries: Vec<&str> = row.split_whitespace().collect();
            let mut tmp_row: Vec<f64> = Vec::new();
            for ent in entries {
                tmp_row.push(ent.parse::<f64>().unwrap());
            }
            data.push(tmp_row);
        }
        let rows = data.len();
        let cols = data[0].len();
        Matrix { rows, cols, data }
    }

    pub fn copy(&self) -> Matrix {
        let mut data: Vec<Vec<f64>> = Vec::new();
        for row in &self.data {
            data.push(row.to_vec());
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    pub fn print(&self) {
        self.data.iter().for_each(|v| println!("{:?}", v));
        println!();
    }

    pub fn apply(&mut self, f: impl Fn(f64) -> f64) {
        self.data = self
            .data
            .iter()
            .map(|v| v.iter().map(|x| f(*x)).collect::<Vec<f64>>())
            .collect::<Vec<Vec<f64>>>();
    }

    pub fn add(&self, b: Matrix) -> Matrix {
        if self.rows != b.rows || self.cols != b.cols {
            panic!("Matrices must be of the same size");
        }
        let mut sum = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                sum.data[i][j] = self.data[i][j] + b.data[i][j];
            }
        }
        return sum;
    }

    pub fn subtract(&self, b: Matrix) -> Matrix {
        if self.rows != b.rows || self.cols != b.cols {
            panic!("Matrices must be of the same size");
        }
        let mut sum = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                sum.data[i][j] = self.data[i][j] - b.data[i][j];
            }
        }
        return sum;
    }

    pub fn dot(&self, b: Matrix) -> Matrix {
        if self.rows != b.cols || self.cols != b.rows {
            panic!(
                "Dimensions not matched. M1 is {} by {}, M2 is {} by {}",
                self.rows, self.cols, b.rows, b.cols
            );
        }
        let mut dp: Matrix = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..b.cols {
                let mut sum = 0.0;
                for k in 0..b.rows {
                    sum += self.data[i][k] * b.data[k][j];
                }
                dp.data[i][j] = sum;
            }
        }
        dp
    }

    // row reduce echelon form
    pub fn rref(&mut self) {
        if self.data[0][0] == 0.0 {
            swap_rows(self, 0);
        }
        let mut lead: usize = 0;
        let rows = self.rows;
        while lead < rows {
            for r in 0..rows {
                let div = self.data[lead][lead];
                let mult = self.data[r][lead] / div;
                if r == lead {
                    self.data[lead] = self.data[lead].iter().map(|entry| entry / div).collect();
                } else {
                    for c in 0..self.cols {
                        self.data[r][c] -= self.data[lead][c] * mult;
                    }
                }
            }
            lead += 1
        }
        correct(self);
    }
}

fn swap_rows(m: &mut Matrix, row: usize) {
    let mut n_r = 0;
    for r in 0..m.rows {
        if m.data[r][0] > 0.0 {
            n_r = r;
            break;
        }
    }
    let temp: Vec<f64> = m.data[row].clone();
    m.data[row] = m.data[n_r].clone();
    m.data[n_r] = temp;
}

fn correct(m: &mut Matrix) {
    for row in 0..m.rows {
        for col in 0..m.cols {
            let elem = m.data[row][col];
            if elem == -0.0 {
                m.data[row][col] = 0.0;
            }
            let floored = elem.floor();
            if elem - floored > 0.999999 {
                m.data[row][col] = elem.round();
            }
            if elem > 0.0 && elem < 0.000001 {
                m.data[row][col] = 0.0;
            }
            if elem < 0.0 && elem > -0.0001 {
                m.data[row][col] = 0.0;
            }
        }
    }
}

