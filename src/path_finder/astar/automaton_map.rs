use std::{collections::HashMap, fs};

use crate::automaton::{self, Automaton, CellState};
use crate::config::{AUTOMATON_SNAPSHOTS_DIRECTORY, GENERATION_STEP};

pub trait AutomatonMap {
    fn new(automaton: Automaton) -> Self;
    fn get_automaton_matrix_generation(&self, generation: u32) -> Vec<Vec<CellState>>;
}

pub struct SnapshotAutomatonMap {
    snapshots: HashMap<u32, Vec<Vec<CellState>>>,
    automaton: Automaton,
}

impl SnapshotAutomatonMap {
    pub fn new(automaton: Automaton) -> Self {
        let snapshots = get_snapshots_from_files();

        SnapshotAutomatonMap {
            snapshots,
            automaton,
        }
    }

    pub fn get_automaton_matrix_generation(&self, generation: u32) -> Vec<Vec<CellState>> {
        let steps = generation / GENERATION_STEP;
        let mut remainder = generation % GENERATION_STEP;

        let snapshot_generation = steps * GENERATION_STEP;

        let mut automaton: Automaton;

        if steps == 0 {
            automaton = self.automaton.clone();
            remainder -= 1;
        } else {
            let snapshot_matrix = self.snapshots.get(&snapshot_generation).unwrap();
            automaton = Automaton::new(snapshot_matrix.clone());
            automaton.generation = snapshot_generation;
        }

        for _ in 0..remainder {
            automaton = automaton.iterate();
        }

        assert_eq!(automaton.generation, generation);

        return automaton.matrix;
    }
}

pub struct InMemoryAutomatonMap {
    automaton_map: HashMap<u32, Vec<Vec<CellState>>>,
}

impl InMemoryAutomatonMap {
    pub fn new(automaton: Automaton) -> Self {
        let automaton_map = get_automaton_map(&automaton);

        Self { automaton_map }
    }

    pub fn get_automaton_matrix_generation(&self, generation: u32) -> &Vec<Vec<CellState>> {
        match self.automaton_map.get(&generation) {
            Some(matrix) => matrix,
            None => panic!("No automaton matrix for generation {}", generation),
        }
    }
}

fn get_automaton_map(automaton: &Automaton) -> HashMap<u32, Vec<Vec<CellState>>> {
    let mut map: HashMap<u32, Vec<Vec<CellState>>> = HashMap::new();

    let generations = 2.5 * (automaton.matrix.len() + automaton.matrix[0].len()) as f32;

    println!("Generating {} automaton maps", generations);

    let mut automaton_clone = automaton.clone();

    for i in 1..=generations as u32 {
        map.insert(i, automaton_clone.matrix.clone());
        automaton_clone = automaton_clone.iterate();

        if i % 100 == 0 {
            println!("Generated {} automaton maps", i);
        }
    }

    return map;
}

fn get_snapshots_from_files() -> HashMap<u32, Vec<Vec<CellState>>> {
    let directory = String::from(AUTOMATON_SNAPSHOTS_DIRECTORY);

    let mut snapshots: HashMap<u32, Vec<Vec<CellState>>> = HashMap::new();

    println!("Reading snapshots from {}", directory);

    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_string = path.to_str().unwrap();
            let generation = path_string_to_generation(path_string);

            let automaton = Automaton::from_file(path_string).unwrap();

            snapshots.insert(generation, automaton.matrix);
        }
    }

    println!("Read {} snapshots", snapshots.len());

    snapshots
}

fn build_snapshots(automaton: Automaton) -> HashMap<u32, Vec<Vec<CellState>>> {
    let mut clone_automaton = automaton.clone();

    let generations = 2.5 * (clone_automaton.matrix.len() + clone_automaton.matrix[0].len()) as f32;

    println!("Building {} / {} snapshots", generations, GENERATION_STEP);

    let mut snapshots = HashMap::new();

    for i in 1..=generations as u32 {
        if i % GENERATION_STEP == 0 {
            snapshots.insert(i, clone_automaton.matrix.clone());
        }

        clone_automaton = clone_automaton.iterate();

        if i % 100 == 0 {
            println!("Built {} snapshots", i);
        }
    }

    snapshots
}

fn path_string_to_generation(path_string: &str) -> u32 {
    let position = path_string.find('/').unwrap();
    let after_slash: String = path_string[position + 1..].into();
    let position = after_slash.find('.').unwrap();
    let number_string: String = after_slash[..position].into();
    let parsed_number: u32 = number_string.parse().unwrap();

    parsed_number
}
