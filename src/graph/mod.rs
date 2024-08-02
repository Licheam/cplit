//! # Graph Theory
//!
//! This module contains graph algorithms.
//!
//! All graph algorithms are implemented for the following graph representation: [`Graph`],
//! which is a simple graph representation using adjacency list.
//!
//! # Examples
//!
//! ```no_run
//! use cplit::graph::{dijkstra, Graph};
//! use cplit::scanln;
//!
//! fn main() {
//!     let (n, m, s): (usize, usize, usize);
//!     // Read the number of nodes, the number of edges, and the source node.
//!     scanln!(n, m, s);
//!     // Create a graph with n nodes and storing nothing in each node,
//!     // and a usize in each edge as weight.
//!     let mut graph = Graph::<(), usize>::new(n);
//!     for _ in 0..m {
//!         // Read an edge u->v with weight w.
//!         let (u, v, w): (usize, usize, usize);
//!         scanln!(u, v, w);
//!         // Create an edge u->v with weight w.
//!         graph.add_edge(u, v, w);
//!     }
//!     let dist = dijkstra(s, &graph);
//!     println!("{:?}", dist);
//! }
//! ```

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
