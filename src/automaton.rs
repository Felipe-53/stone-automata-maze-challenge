use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
pub struct Automaton {
    pub matrix: Vec<Vec<CellState>>,
    pub generation: u32,
    pub starting_point: Position,
    pub finishing_point: Position,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Start,
    Finish,
    Dead,
    Alive,
}

pub type Position = (usize, usize);

fn stay_alive_rule(number_of_living_neighbours: i32) -> bool {
    number_of_living_neighbours > 3 && number_of_living_neighbours < 6
}

fn become_alive_rule(number_of_living_neighbours: i32) -> bool {
    number_of_living_neighbours > 1 && number_of_living_neighbours < 5
}

impl Iterator for Automaton {
    type Item = Vec<Vec<i32>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterate();
        Some(self.to_integer_matrix())
    }
}

impl Automaton {
    pub fn new(pattern: Vec<Vec<CellState>>) -> Automaton {
        let mut starting_point: Option<Position> = None;
        let mut finishing_point: Option<Position> = None;

        for i in 0..pattern.len() {
            for j in 0..pattern[i].len() {
                if pattern[i][j] == CellState::Start {
                    starting_point = Some((i, j));
                }
                if pattern[i][j] == CellState::Finish {
                    finishing_point = Some((i, j));
                }
            }
        }

        if starting_point.is_none() {
            panic!("Starting point is not defined");
        }

        if finishing_point.is_none() {
            panic!("Finishing point is not defined");
        }

        Automaton {
            matrix: pattern,
            starting_point: starting_point.unwrap(),
            finishing_point: finishing_point.unwrap(),
            generation: 1,
        }
    }

    pub fn from_integer_matrix(matrix: Vec<Vec<i32>>) -> Automaton {
        if matrix.len() == 0 {
            panic!("Matrix is empty");
        }

        if matrix[0].len() == 0 {
            panic!("Matrix is empty");
        }

        let column_size = matrix[0].len();
        for row in matrix.iter() {
            if row.len() != column_size {
                panic!("Matrix is not rectangular");
            }
        }

        let mut pattern = vec![vec![CellState::Dead; matrix[0].len()]; matrix.len()];

        let has_defined_starting_and_ending_point: bool = matrix
            .iter()
            .flatten()
            .filter(|&x| *x == 2 || *x == 3)
            .count()
            == 2;

        if !has_defined_starting_and_ending_point {
            panic!("Starting and/or ending points are not defined");
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let cell = matrix[i][j];

                match cell {
                    0 => pattern[i][j] = CellState::Dead,
                    1 => pattern[i][j] = CellState::Alive,
                    2 => pattern[i][j] = CellState::Start,
                    3 => pattern[i][j] = CellState::Finish,
                    _ => (),
                }
            }
        }

        return Automaton::new(pattern);
    }

    pub fn to_integer_matrix(&self) -> Vec<Vec<i32>> {
        let mut pattern = vec![vec![0; self.matrix[0].len()]; self.matrix.len()];

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                pattern[i][j] = match self.matrix[i][j] {
                    CellState::Alive => 1,
                    CellState::Dead => 0,
                    CellState::Start => 2,
                    CellState::Finish => 3,
                }
            }
        }

        pattern
    }

    fn get_number_of_alive_nighbours(&self, (x, y): Position) -> i32 {
        let matrix = &self.matrix;

        let left_boundary = x.checked_sub(1);
        let right_boundary = matrix.get(x + 1);
        let down_boundary = y.checked_sub(1);
        let up_boundary = matrix[x].get(y + 1);

        let x_min = match left_boundary {
            Some(_) => x - 1,
            None => x,
        };

        let x_max = match right_boundary {
            Some(_) => x + 1,
            None => x,
        };

        let y_min = match down_boundary {
            Some(_) => y - 1,
            None => y,
        };

        let y_max = match up_boundary {
            Some(_) => y + 1,
            None => y,
        };

        let mut number_of_alive_neighbours = 0;

        for i in x_min..=x_max {
            for j in y_min..=y_max {
                if i == x && j == y {
                    continue;
                }

                let cell = matrix[i][j];

                match cell {
                    CellState::Alive => number_of_alive_neighbours += 1,
                    CellState::Dead => (),
                    CellState::Start => (),
                    CellState::Finish => (),
                }
            }
        }

        return number_of_alive_neighbours;
    }

    pub fn iterate(&self) -> Automaton {
        let matrix = &self.matrix;

        let mut new_matrix = matrix.clone();

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let cell = matrix[i][j];

                let number_of_alive_neighbours = self.get_number_of_alive_nighbours((i, j));

                new_matrix[i][j] = CellState::Dead;

                match cell {
                    CellState::Alive => {
                        if stay_alive_rule(number_of_alive_neighbours) {
                            new_matrix[i][j] = CellState::Alive
                        }
                    }
                    CellState::Dead => {
                        if become_alive_rule(number_of_alive_neighbours) {
                            new_matrix[i][j] = CellState::Alive
                        }
                    }
                    CellState::Start => new_matrix[i][j] = CellState::Start,
                    CellState::Finish => new_matrix[i][j] = CellState::Finish,
                }
            }
        }

        Automaton {
            matrix: new_matrix,
            starting_point: self.starting_point,
            finishing_point: self.finishing_point,
            generation: self.generation + 1,
        }
    }

    #[allow(dead_code)]
    pub fn print(&self, current_position: Option<Position>) {
        let matrix = &self.matrix;

        let (x, y) = current_position.unwrap_or((usize::MAX, usize::MAX));

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i == x && j == y {
                    print!("O");
                    continue;
                }

                if matrix[i][j] == CellState::Alive {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        println!();
    }
}

impl Automaton {
    #[allow(dead_code)]
    pub fn from_file(path: &str) -> Result<Automaton, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let value: Value = serde_json::from_str(&data)?;
        let input: Vec<Vec<i32>> = value
            .as_array()
            .unwrap()
            .iter()
            .map(|vec| {
                vec.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_i64().unwrap() as i32)
                    .collect()
            })
            .collect();

        Ok(Automaton::from_integer_matrix(input))
    }
}
