//! # Graph Theory
//! 
//! This module contains graph algorithms.
//! 
//! All graph algorithms are implemented for the following graph representation: [`Graph`],
//! which is a simple graph representation using adjacency list.
//! 
//! For example, the following code snippet demonstrates how to use the Dijkstra algorithm:
//! 
//! ```no_run
//! use cplit::graph::{dijkstra, Graph};
//! use cplit::scanln;
//! 
//! fn main() {
//!     let (n, m, s): (usize, usize, usize);
//!     scanln!(n, m, s);
//!     let mut graph = Graph::<(), usize>::new(n);
//!     for _ in 0..m {
//!         let (u, v, w): (usize, usize, usize);
//!         scanln!(u, v, w);
//!         graph.add_edge(u, v, w);
//!     }
//!     let dist = dijkstra(s, &graph);
//!     println!("{}", dist.into_iter().skip(1).map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
//! }
//! ```

pub struct Graph<V, E>
where
    V: Default + Clone,
    E: Clone,
{
    pub nodes: Vec<V>,
    pub edges: Vec<Vec<(usize, E)>>,
}

impl<V, E> Graph<V, E>
where
    V: Default + Clone,
    E: Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            nodes: vec![V::default(); n + 1],
            edges: vec![vec![]; n + 1],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge: E) {
        self.edges[from].push((to, edge));
    }
}

pub mod dijkstra;
pub mod distance;
#[doc(inline)]
pub use self::dijkstra::dijkstra;
#[doc(inline)]
pub use self::distance::Distance;
