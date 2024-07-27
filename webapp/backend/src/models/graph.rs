use sqlx::FromRow;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, start: i32, goal: i32) -> i32 {
        let mut dist: HashMap<i32, i32> = HashMap::new();
        let mut heap = BinaryHeap::new();

        heap.push(State { cost: 0, position: start });
        dist.insert(start, 0);

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal { return cost; }

            if cost > *dist.get(&position).unwrap_or(&i32::MAX) { continue; }

            if let Some(edges) = self.edges.get(&position) {
                for edge in edges {
                    let next = State { cost: cost + edge.weight, position: edge.node_b_id };

                    if next.cost < *dist.get(&next.position).unwrap_or(&i32::MAX) {
                        heap.push(next);
                        dist.insert(next.position, next.cost);
                    }
                }
            }
        }

        i32::MAX
    }
}