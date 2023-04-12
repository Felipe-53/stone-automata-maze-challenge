mod automaton;
mod benchmark;
mod config;
mod path_finder;
mod sparse_automaton;

use automaton::Automaton;
use path_finder::find_path;

use std::env;
use std::fs::{read_to_string, File};
use std::io::prelude::*;

#[allow(unused_variables)]
fn main() {
    let (input_file_path, output_file_path) = get_input_and_output_file_path();

    let automaton = match Automaton::from_file(&input_file_path) {
        Ok(automaton) => automaton,
        Err(error) => panic!(
            "Error loading automaton:\n {}\n Path: {}",
            error, input_file_path
        ),
    };

    let best_result = get_current_best_result(&output_file_path);
    println!("Current best result: {}", best_result);

    let path = find_path(automaton.clone()).expect("Unable to find path");
    println!("Found path with length: {}", path.len());

    if path.len() < best_result {
        let mut file = File::create(output_file_path).unwrap();
        writeln!(file, "{}", path_to_json(&path)).unwrap();
        println!("New best result: {}", path.len());
    }
}

fn path_to_json(result: &Vec<(usize, usize)>) -> String {
    let arr: Vec<[usize; 2]> = result.iter().map(|(x, y)| [*x, *y]).collect();

    serde_json::to_string(&arr).unwrap()
}

fn get_input_and_output_file_path() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).unwrap_or(&String::from("sample")).clone();

    let input_file_path = format!("inputs/{}.json", file);
    let output_file_path = format!("outputs/{}.result.json", file);

    return (input_file_path, output_file_path);
}

fn get_current_best_result(output_file_path: &String) -> usize {
    let json = match read_to_string(output_file_path) {
        Ok(content) => content,
        Err(_) => {
            let empty_array_str = String::from("[]");
            let mut file = File::create(output_file_path).unwrap();
            writeln!(file, "{empty_array_str}").unwrap();
            empty_array_str
        }
    };

    let best_result_path: Vec<[usize; 2]> = serde_json::from_str(&json).unwrap();
    let mut best_result = best_result_path.len();

    if best_result == 0 {
        best_result = 99999999999;
    }

    best_result
}
