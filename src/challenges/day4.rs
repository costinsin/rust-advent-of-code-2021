use std::{collections::HashMap, error::Error};

struct BingoMatrix {
    number_map: HashMap<u32, (u32, u32)>,
    number_matrix: Vec<Vec<(u32, bool)>>,
    won: bool,
}

impl BingoMatrix {
    pub fn new(string: &str) -> Self {
        let lines = string.split("\n");

        // Construct number map
        let mut number_map = HashMap::new();
        lines.clone().fold(0 as u32, |line_no, line| {
            line.trim()
                .split_whitespace()
                .fold(0 as u32, |col_no, elem| {
                    number_map.insert(elem.parse::<u32>().unwrap_or_default(), (line_no, col_no));

                    col_no + 1
                });

            line_no + 1
        });

        // Construct number matrix
        let number_matrix = lines
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|elem| (elem.parse::<u32>().unwrap_or_default(), false))
                    .collect::<Vec<(u32, bool)>>()
            })
            .collect();

        BingoMatrix {
            number_map: number_map,
            number_matrix: number_matrix,
            won: false,
        }
    }

    pub fn update(&mut self, number: u32) -> bool {
        if self.won {
            return false;
        }

        let (mut line, mut col) = (false, false);
        if let Some(x) = self.number_map.get(&number) {
            self.number_matrix[x.0 as usize][x.1 as usize].1 = true;

            line = (0..self.number_matrix[0].len()).fold(true, |acc, e| {
                acc && self.number_matrix[x.0 as usize][e as usize].1
            });
            col = (0..self.number_matrix[0].len()).fold(true, |acc, e| {
                acc && self.number_matrix[e as usize][x.1 as usize].1
            });
        }

        if line || col {
            self.won = true;
        }

        line || col
    }

    pub fn get_unmarked_sum(&self) -> u32 {
        self.number_matrix.iter().fold(0 as u32, |acc, line| {
            acc + line
                .iter()
                .fold(0, |acc, e| if e.1 { acc } else { acc + e.0 })
        })
    }
}

pub fn part1(file_content: &String) -> Result<(), Box<dyn Error>> {
    let mut lines = file_content.split("\n\n");

    let number_stream: Vec<u32> = lines
        .next()
        .unwrap_or_default()
        .trim()
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    let mut bingo_matrices: Vec<BingoMatrix> = lines.map(|e| BingoMatrix::new(e)).collect();

    for number in number_stream {
        for bmatrix in bingo_matrices.iter_mut() {
            let winner = bmatrix.update(number);

            if winner {
                println!("{}", bmatrix.get_unmarked_sum() * number);

                return Ok(());
            }
        }
    }

    Ok(())
}

pub fn part2(file_content: &String) -> Result<(), Box<dyn Error>> {
    let mut lines = file_content.split("\n\n");

    let number_stream: Vec<u32> = lines
        .next()
        .unwrap_or_default()
        .trim()
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    let mut bingo_matrices: Vec<BingoMatrix> = lines.map(|e| BingoMatrix::new(e)).collect();

    let (mut last_winner_sum, mut last_winner_number) = (0, 0);

    for number in number_stream {
        for bmatrix in bingo_matrices.iter_mut() {
            let winner = bmatrix.update(number);

            if winner {
                last_winner_sum = bmatrix.get_unmarked_sum();
                last_winner_number = number;
            }
        }
    }

    println!("{}", last_winner_sum * last_winner_number);

    Ok(())
}
