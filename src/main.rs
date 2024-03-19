use std::{fmt, thread, time, env};
extern crate rand;

use rand::distributions::{Distribution, Uniform};

const DEFAULT_SIZE: (usize, usize) = (23, 40);

struct Matrix {
    m: usize,
    n: usize,
    rows: Vec<Vec<usize>>,
}

impl Matrix {
    fn new(m: usize, n:usize) -> Matrix {
        let mut rows = Vec::new();

        for _ in 0..m {
            rows.push(vec![0; n]);
        };

        Matrix {
            m: m,
            n: n,
            rows: rows,
        }
    }

    fn seed(&mut self, cells: Option<usize>) -> () {
        let mut rng = rand::thread_rng();
        let row = Uniform::from(0..self.m);
        let col = Uniform::from(0..self.n);
        let iterations: usize = match cells {
            Some(n) => n,
            None => self.n * (self.m / 2),
        };

        for _ in 0..iterations {
            let m = row.sample(&mut rng);
            let n = col.sample(&mut rng);
            self.rows[m][n] = 1;
        }
    }

    fn pulse(&mut self) -> () {
        let mut new_matrix = Matrix::new(self.m, self.n);
        
        for (i_r, row) in self.rows.iter().enumerate() {
            let neighbour1 = {
                if i_r > 0 {
                    &self.rows[i_r - 1]
                } else {
                    &self.rows[self.m - 1]
                }
            };

            let neighbour2 = {
                if i_r < self.m - 1 {
                    &self.rows[i_r + 1]
                } else {
                    &self.rows[0]
                }
            };
            
            for (i_c, cell) in row.iter().enumerate() {
                let sum: usize;
                if i_c == 0 {
                    sum = neighbour1[0] + neighbour1[1]
                        + neighbour1[self.n - 1]
                        + neighbour2[0] + neighbour2[1]
                        + neighbour2[self.n - 1]
                        + row[1] + row[self.n - 1];
                } else if i_c == self.n - 1 {
                    sum = neighbour1[i_c - 1] + neighbour1[i_c]
                        + neighbour1[0]
                        + neighbour2[i_c - 1] + neighbour2[i_c]
                        + neighbour2[0]
                        + row[i_c - 1] + row[0];
                } else {
                    sum = neighbour1[i_c - 1..i_c + 2].iter().sum::<usize>()
                        + neighbour2[i_c - 1..i_c + 2].iter().sum::<usize>()
                        + row[i_c - 1] + row[i_c + 1];
                }

                if sum > 0 && sum < 5 {
                    if sum == 3 {
                        new_matrix.rows[i_r][i_c] = 1;
                    } else {
                        new_matrix.rows[i_r][i_c] = *cell;
                    }
                } else {
                    new_matrix.rows[i_r][i_c] = 0;
                }
            }
        }
        self.rows = new_matrix.rows;
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for row in &self.rows[0..self.rows.len() - 1] {
            for num in row {
                if *num == 0 {
                    out.push_str("░░");
                } else {
                    out.push_str("▓▓");
                };
            }
            out.push('\n');
        }

        for num in &self.rows[self.rows.len() - 1] {
            if *num == 0 {
                out.push_str("░░");
            } else {
                out.push_str("▓▓");
            };
        }
        
        write!(f, "{}", out)
    }
}

fn parse_args() -> (usize, usize) {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 3 {
        DEFAULT_SIZE
    } else {
        let m: usize = match args[1].trim().parse() {
            Ok(m) => m,
            Err(_) => DEFAULT_SIZE.0,
        };

        let n: usize = match args[2].trim().parse() {
            Ok(n) => n,
            Err(_) => DEFAULT_SIZE.1,
        };

        if n > 0 && m > 0 {
            (m, n)
        } else {
            DEFAULT_SIZE
        }
    }
}

fn main() {
    let (m, n) = parse_args();
    let mut matrix = Matrix::new(m, n);
    matrix.seed(None);
    println!("\x1B[2J{}", &matrix);

    loop {
        matrix.pulse();
        println!("\x1B[H{}", &matrix);
        thread::sleep(time::Duration::new(1, 0));
    }
}
