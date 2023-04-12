#![allow(unused_variables, dead_code)]

use std::time::Instant;

use stone_challenge::{
    automaton::Automaton, path_finder::shared::get_possible_moves,
    sparse_automaton::get_possible_moves as get_sparse_possible_moves,
    sparse_automaton::SparseAutomaton,
};

fn main() {
    sparse();
}

fn original() {
    let automaton = Automaton::from_file("inputs/final_pt_1.json").unwrap();

    assert_eq!(
        automaton.matrix.len(),
        automaton.matrix[0].len(),
        "The automaton matrix needs to be square!"
    );
    let automaton_length = automaton.matrix.len();
    let division_factor = 8;

    let iterations = automaton_length / division_factor;

    let mut automaton_clone = automaton.clone();

    println!("Running {} iterations of the automaton", iterations);

    let start = Instant::now();

    for i in 0..iterations {
        get_possible_moves(&automaton_clone.matrix, (i, i));
        automaton_clone = automaton_clone.iterate();

        if i % 10 == 0 {
            println!("Ran {i} iterations");
        }
    }

    let end = Instant::now();

    println!("The elapsed time was: {:?}", end.duration_since(start));
}

fn sparse() {
    let automaton = SparseAutomaton::from_file("inputs/final_pt_1.json").unwrap();

    assert_eq!(
        automaton.width, automaton.height,
        "The automaton matrix needs to be square!"
    );
    let automaton_length = automaton.width;
    let division_factor = 8;

    let iterations = automaton_length / division_factor;

    let mut automaton_clone = automaton.clone();

    println!("Running {} iterations of the automaton", iterations);

    let start = Instant::now();

    for i in 0..iterations {
        get_sparse_possible_moves(
            (automaton.height, automaton.width),
            &automaton_clone.live_cells,
            (i, i),
        );
        automaton_clone = automaton_clone.iterate();

        if i % 10 == 0 {
            println!("Ran {i} iterations");
        }
    }

    let end = Instant::now();

    println!("The elapsed time was: {:?}", end.duration_since(start));
}
