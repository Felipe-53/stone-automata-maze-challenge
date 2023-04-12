use crate::automaton::{CellState, Position};

pub fn get_possible_moves(matrix: &Vec<Vec<CellState>>, position: Position) -> Vec<Position> {
    let mut possible_moves: Vec<Position> = Vec::new();

    let (i, j) = position;

    if let Some(_) = j.checked_sub(1) {
        if matrix[i][j - 1] != CellState::Alive {
            possible_moves.push((i, j - 1));
        }
    }

    if let Some(_) = matrix[i].get(j + 1) {
        if matrix[i][j + 1] != CellState::Alive {
            possible_moves.push((i, j + 1));
        }
    }

    if let Some(_) = matrix.get(i + 1) {
        if matrix[i + 1][j] != CellState::Alive {
            possible_moves.push((i + 1, j));
        }
    }

    if let Some(_) = i.checked_sub(1) {
        if matrix[i - 1][j] != CellState::Alive {
            possible_moves.push((i - 1, j));
        }
    }

    return possible_moves;
}

pub fn manhattan_distance(from: Position, to: Position) -> u32 {
    let i = to.0 as i32;
    let j = to.1 as i32;
    let x = from.0 as i32;
    let y = from.1 as i32;

    let distance = ((i - x) as i32).abs() + ((j - y) as i32).abs();

    return distance as u32;
}
