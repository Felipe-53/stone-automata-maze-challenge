use crate::path_finder::shared::{get_possible_moves, manhattan_distance};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

pub mod automaton_map;
mod priority_queue;

use crate::path_finder::astar::priority_queue::PriorityQueue;

use crate::config::REPORTING_ITERATION_STEP;

use self::automaton_map::InMemoryAutomatonMap;
use crate::automaton::{Automaton, Position};

use self::priority_queue::Node;

pub fn path_finder(automaton: Automaton) -> Option<Vec<Position>> {
    let starting_point = automaton.starting_point;
    let ending_point = automaton.finishing_point;

    let result = algorithm(automaton, starting_point, ending_point);

    result
}

fn algorithm(
    automaton: Automaton,
    starting_point: Position,
    ending_point: Position,
) -> Option<Vec<Position>> {
    let mut closed_states: HashSet<State> = HashSet::new();
    let mut results: HashMap<State, Result> = HashMap::new();
    let mut priority_queue = PriorityQueue::new();

    let automaton_map = InMemoryAutomatonMap::new(automaton.clone());

    let starting_state = State {
        automaton_generation: 1,
        position: starting_point,
    };

    let starting_result = Result {
        cost: calculate_cost(
            starting_state.automaton_generation,
            manhattan_distance(ending_point, starting_point),
        ),
        previous_position: starting_point,
    };

    results.insert(starting_state, starting_result);

    priority_queue.enqueue(Node {
        state: starting_state,
        cost: starting_result.cost,
    });

    let mut counter: u64 = 0;

    while let Some(current) = priority_queue.dequeue() {
        let current_node = current;

        if counter % REPORTING_ITERATION_STEP == 0 {
            let current_result = results.get(&current_node).unwrap();
            println!("Current iteration: {:?}", counter);
            println!("Current result: {:?}", current_result);
            println!("Results length {:?}", results.len());
            println!("Queue length {:?}", priority_queue.get_length());
        }

        if current_node.position == ending_point {
            return Some(recreate_steps(&results, &current_node));
        }

        let current_position = current_node.position;

        let next_generation = current_node.automaton_generation + 1;

        let next_gen_automaton_matrix =
            automaton_map.get_automaton_matrix_generation(next_generation);

        let possible_moves = get_possible_moves(&next_gen_automaton_matrix, current_position);

        for position in possible_moves {
            let new_state = State {
                automaton_generation: next_generation,
                position,
            };

            if closed_states.contains(&new_state) {
                continue;
            }

            let calculated_cost =
                calculate_cost(next_generation, manhattan_distance(ending_point, position));

            let calculated_result = Result {
                previous_position: current_node.position,
                cost: calculated_cost,
            };

            if results.contains_key(&new_state) {
                let past_result = results.get(&new_state).unwrap();

                if calculated_cost < past_result.cost {
                    results.insert(new_state, calculated_result);
                    priority_queue.update(Node {
                        state: new_state,
                        cost: calculated_cost,
                    })
                }
            } else {
                results.insert(new_state, calculated_result);
                priority_queue.enqueue(Node {
                    state: new_state,
                    cost: calculated_cost,
                })
            }
        }

        closed_states.insert(State {
            automaton_generation: current_node.automaton_generation,
            position: current_node.position,
        });

        counter += 1;
    }

    None
}

fn calculate_cost(starting_node_cost: u32, heuristic_cost: u32) -> u32 {
    starting_node_cost + heuristic_cost
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct State {
    position: Position,
    automaton_generation: u32,
}

fn recreate_steps(results: &HashMap<State, Result>, entry: &State) -> Vec<Position> {
    let mut state = entry.to_owned();
    let mut result = results.get(&state).unwrap().clone();

    let mut path: Vec<Position> = vec![];

    path.push(state.position);

    println!("Made it till the end, recreating steps...");

    while let Some(previous_result) = results.get(&State {
        automaton_generation: state.automaton_generation - 1,
        position: result.previous_position,
    }) {
        path.push(result.previous_position);
        result = previous_result.to_owned();
        state = State {
            automaton_generation: state.automaton_generation - 1,
            position: result.previous_position,
        };
    }

    path.reverse();

    path
}

#[derive(Clone, Copy, Debug)]
pub struct Result {
    cost: u32,
    previous_position: Position,
}
