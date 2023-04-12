use std::{collections::HashSet, error::Error, fs::File, io::Read};

use serde_json::Value;

pub type Position = (usize, usize);

#[derive(Clone, Debug)]
pub struct SparseAutomaton {
    pub width: usize,
    pub height: usize,
    pub live_cells: HashSet<Position>,
    pub starting_point: Position,
    pub finishing_point: Position,
    pub generation: u32,
}

fn stay_alive_rule(number_of_living_neighbors: u32) -> bool {
    number_of_living_neighbors > 3 && number_of_living_neighbors < 6
}

fn become_alive_rule(number_of_living_neighbors: u32) -> bool {
    number_of_living_neighbors > 1 && number_of_living_neighbors < 5
}

impl SparseAutomaton {
    pub fn from_file(path: &str) -> Result<SparseAutomaton, Box<dyn Error>> {
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

        let width = input[0].len();
        let height = input.len();

        // default
        let mut starting_point = (0, 0);
        let mut finishing_point = (height - 1, width - 1);

        let mut live_cells = HashSet::new();

        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j] == 1 {
                    live_cells.insert((i, j));
                }

                if input[i][j] == 2 {
                    starting_point = (i, j);
                }

                if input[i][j] == 3 {
                    finishing_point = (i, j);
                }
            }
        }

        Ok(SparseAutomaton {
            width,
            height,
            live_cells,
            starting_point,
            finishing_point,
            generation: 1,
        })
    }

    fn get_number_of_alive_neighbors(&self, (x, y): Position) -> u32 {
        let mut count = 0;

        let x = x as i32;
        let y = y as i32;

        let mut filter: Vec<(i32, i32)> = Vec::new();

        filter.push((x, y + 1));
        filter.push((x + 1, y));
        filter.push((x + 1, y + 1));

        if y - 1 >= 0 {
            filter.push((x, y - 1));
            filter.push((x + 1, y - 1));
        }

        if x - 1 >= 0 {
            filter.push((x - 1, y));
            filter.push((x - 1, y + 1));
        }

        if x - 1 >= 0 && y - 1 >= 0 {
            filter.push((x - 1, y - 1));
        }

        let possible: Vec<Position> = filter
            .iter()
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect();

        for position in possible {
            if self.live_cells.contains(&position) {
                count += 1;
            }
        }

        count
    }

    pub fn iterate(&self) -> Self {
        let mut new_live_cells = HashSet::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let position = (i, j);

                if position == self.starting_point || position == self.finishing_point {
                    continue;
                }

                let number_of_alive_neighbors = self.get_number_of_alive_neighbors(position);

                if self.live_cells.contains(&position) {
                    if stay_alive_rule(number_of_alive_neighbors) {
                        new_live_cells.insert(position);
                    }
                } else {
                    if become_alive_rule(number_of_alive_neighbors) {
                        new_live_cells.insert(position);
                    }
                }
            }
        }

        SparseAutomaton {
            live_cells: new_live_cells,
            generation: self.generation + 1,
            ..*self
        }
    }
}

pub fn get_possible_moves(
    bounds: (usize, usize),
    live_cells: &HashSet<Position>,
    position: Position,
) -> Vec<Position> {
    let (height, width) = bounds;

    let position = (position.0 as i32, position.1 as i32);

    let left = (position.0, position.1 - 1);
    let right = (position.0, position.1 + 1);
    let top = (position.0 - 1, position.1);
    let bottom = (position.0 + 1, position.1);

    let all: Vec<(i32, i32)> = vec![left, right, top, bottom];
    let mut filtered = vec![];

    for position in all {
        if position.0 < 0 || position.0 >= height as i32 {
            continue;
        }

        if position.1 < 0 || position.1 >= width as i32 {
            continue;
        }

        filtered.push(position);
    }

    let mut valid_moves: Vec<Position> = filtered
        .clone()
        .iter()
        .map(|position| ((position.0 as usize, position.1 as usize)))
        .collect();

    valid_moves.retain(|position| !live_cells.contains(position));

    valid_moves
}
