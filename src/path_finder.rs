use crate::automaton::{Automaton, CellState, Position};
use std::time::Instant;
mod astar;
mod original;
pub mod shared;

use astar::path_finder;

pub fn find_path(automaton: Automaton) -> Option<Vec<Position>> {
    if automaton.generation != 1 {
        panic!("Automaton must be in generation 1");
    }

    let start_time = Instant::now();
    let result = path_finder(automaton.clone());
    let end_time = Instant::now();
    println!("Time elapsed: {:?}", end_time.duration_since(start_time));

    match result {
        Some(result) => {
            if verify_result(&automaton, &result) {
                return Some(result);
            } else {
                return None;
            }
        }
        None => None,
    }
}

pub fn verify_result(automaton: &Automaton, result: &Vec<Position>) -> bool {
    let mut automaton = automaton.clone();

    let verification_result = result.clone();

    for current_position in verification_result {
        let (i, j) = current_position;
        if automaton.matrix[i][j] == CellState::Alive {
            println!(
                "Current position {:?} coincids with alive cell at iteration {}",
                current_position, automaton.generation
            );

            return false;
        }

        automaton = automaton.iterate();
    }

    true
}
