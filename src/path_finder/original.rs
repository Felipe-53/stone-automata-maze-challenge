use crate::automaton::{Automaton, Position};
use crate::path_finder::shared::{get_possible_moves, manhattan_distance};
use rand::Rng;

use super::astar::automaton_map::InMemoryAutomatonMap;

pub fn path_finder(automaton: Automaton) -> Option<Vec<Position>> {
    let starting_point = automaton.starting_point;
    let finishing_point = automaton.finishing_point;

    let automaton_map = InMemoryAutomatonMap::new(automaton.clone());

    let result = helper(
        automaton.generation,
        starting_point,
        finishing_point,
        vec![starting_point],
        &automaton_map,
    );

    if result.1 == false {
        return None;
    }

    return Some(result.0);
}
static mut FUNCTION_CALL_COUNT: u32 = 0;

fn helper(
    generation: u32,
    starting_point: Position,
    ending_point: Position,
    path: Vec<Position>,
    automaton_map: &InMemoryAutomatonMap,
) -> (Vec<Position>, bool) {
    unsafe {
        FUNCTION_CALL_COUNT += 1;

        if FUNCTION_CALL_COUNT >= 10000 {
            return (path, false);
        }
    }

    if starting_point == ending_point {
        return (path, true);
    }

    if generation == 1000 {
        println!("Generation limit reached");
        return (path, false);
    }

    let next_generation = generation + 1;

    let next_gen_automaton_matrix = automaton_map.get_automaton_matrix_generation(next_generation);

    let mut possible_moves = get_possible_moves(&next_gen_automaton_matrix, starting_point);
    if possible_moves.len() == 0 {
        return (path, false);
    }

    possible_moves.sort_by(|vec1, vec2| {
        let a_distance = manhattan_distance(ending_point, *vec1);
        let b_distance = manhattan_distance(ending_point, *vec2);
        a_distance.cmp(&b_distance)
    });

    // There will be at most 4 possible moves
    // Of those, only 2 can have the same distance to the finish
    // If this is the case, we randomly choose which one to go first
    // That way, if there are multiple solutions,
    // We can get slightly different results each time

    if possible_moves.len() > 1 {
        let first_option_distance = manhattan_distance(ending_point, possible_moves[0]);
        let second_option_distance = manhattan_distance(ending_point, possible_moves[1]);

        if first_option_distance == second_option_distance {
            let mut rng = rand::thread_rng();
            let random_bool = rng.gen_bool(0.5);

            if random_bool {
                let first_option = possible_moves[0].clone();
                let second_option = possible_moves[1].clone();

                possible_moves[0] = second_option;
                possible_moves[1] = first_option;
            }
        }
    }

    for point in possible_moves {
        let mut path_clone = path.clone();
        path_clone.push(point);

        let result = helper(
            next_generation,
            point,
            ending_point,
            path_clone,
            &automaton_map,
        );

        if result.1 {
            return result;
        }
    }

    return (path, false);
}
