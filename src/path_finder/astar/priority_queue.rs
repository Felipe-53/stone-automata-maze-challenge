use crate::path_finder::astar::State;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub state: State,
    pub cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PriorityQueue {
    items: BinaryHeap<Node>,
}

impl PriorityQueue {
    pub fn new() -> Self {
        PriorityQueue {
            items: BinaryHeap::new(),
        }
    }

    pub fn enqueue(&mut self, node: Node) {
        self.items.push(node);
    }

    pub fn dequeue(&mut self) -> Option<State> {
        self.items.pop().map(|node| node.state)
    }

    pub fn update(&mut self, node: Node) {
        self.items.retain(|item| node.state != item.state);
        self.enqueue(node);
    }

    pub fn get_length(&self) -> usize {
        self.items.len()
    }
}
