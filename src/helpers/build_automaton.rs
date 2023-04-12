use std::io::Write;
use stone_challenge::automaton::Automaton;
use stone_challenge::config::{
    AUTOMATON_FILE_PATH, AUTOMATON_SNAPSHOTS_DIRECTORY, GENERATION_STEP,
};

fn main() {
    let mut automaton = match Automaton::from_file(AUTOMATON_FILE_PATH) {
        Ok(automaton) => automaton,
        Err(error) => panic!(
            "Error loading automaton:\n {}\n Path: {}",
            error, AUTOMATON_FILE_PATH
        ),
    };

    let generations = 2.5 * (automaton.matrix.len() + automaton.matrix[0].len()) as f32;

    println!("Target generation span: {}", generations);
    println!("Generation step: {}", GENERATION_STEP);
    println!(
        "Building a total of {} automaton snapshots",
        (generations / GENERATION_STEP as f32) as u32
    );

    let generations = generations as u32;

    for i in 1..=generations {
        if i % GENERATION_STEP == 0 {
            let integer_matrix = automaton.to_integer_matrix();

            let file_path = format!("{}/{}.json", AUTOMATON_SNAPSHOTS_DIRECTORY, i);
            let mut file = std::fs::File::create(file_path).unwrap();

            let mut file_str = String::from("[\n");

            for row in integer_matrix {
                let mut row_str = String::from("[");

                row_str += &row
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                row_str += "],";

                file_str += &row_str;
            }

            // remove trailing comma
            file_str.pop();

            file_str += "\n]";

            writeln!(file, "{}", file_str).unwrap();
        }

        automaton = automaton.iterate();

        if i % 100 == 0 {
            println!("Iteration: {}", i);
        }
    }
}
