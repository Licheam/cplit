//! # Graph Theory
//!
//! This module contains graph algorithms.
//!
//! All graph algorithms are implemented for the following graph representation: [`Graph`],
//! which is a simple graph representation using adjacency list.

/// Graph representation using adjacency list.
///
/// `V` is the information stored in each node, and `E` is the information stored in each edge.
pub struct Graph<V, E>
where
    V: Default + Clone,
    E: Default + Clone,
{
    /// The information stored in each node.
    pub nodes: Vec<V>,
    pub head: Vec<usize>,

    /// The information stored in each edge.
    pub edges: Vec<(usize, usize, E)>,
}

impl<V, E> Graph<V, E>
where
    V: Default + Clone,
    E: Default + Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            nodes: vec![V::default(); n + 1],
            head: vec![0; n + 1],
            edges: vec![Default::default()],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge: E) {
        self.edges.push((self.head[from], to, edge));
        self.head[from] = self.edges.len() - 1;
    }

    pub fn get_edges(&self, node: usize) -> impl Iterator<Item = (&usize, &E)> {
        let mut edge = self.head[node];
        std::iter::from_fn(move || {
            if edge == 0 {
                return None;
            }
            let (next, to, edge_info) = &self.edges[edge];
            edge = *next;
            Some((to, edge_info))
        })
    }
}

pub mod dijkstra;
pub mod distance;
#[doc(inline)]
pub use self::dijkstra::dijkstra;
#[doc(inline)]
pub use self::distance::Distance;
