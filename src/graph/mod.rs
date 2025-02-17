//! # Graph Theory
//!
//! This module contains graph algorithms.
//!
//! All graph algorithms are implemented for the following graph representation: [`Graph`],
//! which is a simple graph representation using adjacency list.

use std::cmp::Ordering::{self, Less};
use std::cmp::max;
use std::iter::from_fn;
/// Graph representation using adjacency list.
///
/// `V` is the information stored in each node, and `E` is the information stored in each edge.
pub struct Graph<V = (), E = ()>
where
    V: Default + Clone,
    E: Default + Clone,
{
    /// The information stored in each node.
    pub nodes: Vec<V>,

    /// The head node in adjacency list for each node in graph.
    pub head: Vec<usize>,

    /// The information stored in each edge.
    pub edges: Vec<(usize, usize, E)>,

    /// The erased edges index.
    erased: Vec<usize>,
}

impl<V, E> Graph<V, E>
where
    V: Default + Clone,
    E: Default + Clone,
{
    /// Create a new graph with `n` nodes.
    pub fn new(n: usize) -> Self {
        Self {
            nodes: vec![V::default(); n + 1],
            head: vec![0; n + 1],
            edges: vec![Default::default()],
            erased: Vec::new(),
        }
    }

    /// Create a new graph from a given vector of nodes.
    pub fn from_nodes(nodes: Vec<V>) -> Self {
        Self {
            head: vec![0; nodes.len()],
            nodes,
            edges: vec![Default::default()],
            erased: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self::new(0)
    }

    /// Get the number of nodes in the graph.
    pub fn len_nodes(&self) -> usize {
        self.nodes.len() - 1
    }

    /// Get the number of edges in the graph.
    pub fn len_edges(&self) -> usize {
        self.edges.len() - self.erased.len() - 1
    }

    /// Erase the edge with index `idx`.
    pub fn erase_edge(&mut self, idx: &mut usize) {
        if *idx == 0 {
            return;
        }
        *idx = self.edges[*idx].0;
        self.erased.push(*idx);
    }

    fn sort_edges_inner<F>(&mut self, edge: usize, len: usize, is_less: &mut F) -> usize
    where
        F: FnMut(&(usize, &V), &(usize, &V)) -> bool,
    {
        if len <= 1 {
            return edge;
        }
        let mut p1 = edge;
        let mut p2 = self.get_edges_enum_from(edge).nth(len / 2 - 1).unwrap().0;
        (self.edges[p2].0, p2) = (0, self.edges[p2].0);

        p1 = self.sort_edges_inner(p1, len / 2, is_less);
        p2 = self.sort_edges_inner(p2, (len + 1) / 2, is_less);
        let mut lst;
        if is_less(
            &(self.edges[p1].1, &self.nodes[self.edges[p1].1]),
            &(self.edges[p2].1, &self.nodes[self.edges[p2].1]),
        ) {
            lst = p1;
            p1 = self.edges[p1].0;
        } else {
            lst = p2;
            p2 = self.edges[p2].0;
        }
        let head = lst;

        while p1 != 0 || p2 != 0 {
            if p1 != 0
                && (p2 == 0
                    || is_less(
                        &(self.edges[p1].1, &self.nodes[self.edges[p1].1]),
                        &(self.edges[p2].1, &self.nodes[self.edges[p2].1]),
                    ))
            {
                self.edges[lst].0 = p1;
                p1 = self.edges[p1].0;
            } else {
                self.edges[lst].0 = p2;
                p2 = self.edges[p2].0;
            }
            lst = self.edges[lst].0;
        }

        head
    }

    /// Sort the edges of a node by the node id.
    pub fn sort_edges(&mut self, node: usize) {
        self.sort_edges_by(node, |(a, _), (b, _)| a.cmp(b));
    }

    /// Sort the edges of a node by the given comparator.
    pub fn sort_edges_by<F>(&mut self, node: usize, mut compare: F)
    where
        F: FnMut(&(usize, &V), &(usize, &V)) -> Ordering,
    {
        let len = self.get_edges(node).count();
        self.head[node] =
            self.sort_edges_inner(self.head[node], len, &mut |a, b| compare(a, b) == Less);
    }

    /// Add an undirected edge between `from` and `to` with information `info`.
    pub fn add_edge(&mut self, from: usize, to: usize, info: E) {
        if max(from, to) >= self.nodes.len() {
            self.nodes.resize(max(from, to) + 1, V::default());
            self.head.resize(max(from, to) + 1, 0);
        }
        if self.erased.is_empty() {
            self.edges.push((self.head[from], to, info));
            self.head[from] = self.edges.len() - 1;
        } else {
            let idx = self.erased.pop().unwrap();
            self.edges[idx] = (self.head[from], to, info);
            self.head[from] = idx;
        }
    }

    /// Returns an iterator over the edges from the edge with index `edge`.
    /// The iterator returns the destination node, and the information stored in the edge.
    pub fn get_edges_from(&self, mut edge: usize) -> impl Iterator<Item = (&usize, &E)> {
        from_fn(move || {
            if edge == 0 {
                return None;
            }
            let (next, to, edge_info) = &self.edges[edge];
            edge = *next;
            Some((to, edge_info))
        })
    }

    /// Returns an iterator over the edges from the edge with index `edge`.
    /// The iterator returns the index of the edge, the destination node, and the information stored in the edge.
    pub fn get_edges_enum_from(
        &self,
        mut edge: usize,
    ) -> impl Iterator<Item = (usize, (&usize, &E))> {
        from_fn(move || {
            if edge == 0 {
                return None;
            }
            let (next, to, edge_info) = &self.edges[edge];
            let idx = edge;
            edge = *next;
            Some((idx, (to, edge_info)))
        })
    }

    /// Returns an iterator over the edges of a node.
    /// The iterator returns the destination node, and the information stored in the edge.
    pub fn get_edges(&self, node: usize) -> impl Iterator<Item = (&usize, &E)> {
        self.get_edges_from(self.head[node])
    }

    /// Returns an iterator over the edges of a node.
    /// The iterator returns the index of the edge, the destination node, and the information stored in the edge.
    pub fn get_edges_enum(&self, node: usize) -> impl Iterator<Item = (usize, (&usize, &E))> {
        self.get_edges_enum_from(self.head[node])
    }

    /// Returns an iterator over the edges of a node.
    /// The iterator returns the destination node, and the information stored in the edge.
    /// This is used if you want to iterate edges only once.
    pub fn get_edges_from_once<'a>(
        &'a self,
        cur: &'a mut usize,
    ) -> impl Iterator<Item = (&usize, &E)> + 'a {
        from_fn(move || {
            if *cur == 0 {
                return None;
            }
            let (next, to, edge_info) = &self.edges[*cur];
            *cur = *next;
            Some((to, edge_info))
        })
    }

    /// Returns an iterator over the edges of a node.
    /// The iterator returns the index of the edge, the destination node, and the information stored in the edge.
    /// This is used if you want to iterate edges only once.
    pub fn get_edges_enum_from_once<'a>(
        &'a self,
        cur: &'a mut usize,
    ) -> impl Iterator<Item = (usize, (&usize, &E))> + 'a {
        from_fn(move || {
            if *cur == 0 {
                return None;
            }
            let (next, to, edge_info) = &self.edges[*cur];
            let idx = *cur;
            *cur = *next;
            Some((idx, (to, edge_info)))
        })
    }

    /// Get the edge information of the edge with index `idx`.
    pub fn get_edge(&self, idx: usize) -> (usize, usize, &E) {
        let (next, to, edge_info) = &self.edges[idx];
        (*next, *to, edge_info)
    }

    /// Get the twin edge information of the edge with index `idx`.
    /// Often used in undirected graphs as the reverse edge.
    pub fn get_twin_edge(&self, idx: usize) -> (usize, usize, &E) {
        let (next, to, edge_info) = &self.edges[TWIN(idx)];
        (*next, *to, edge_info)
    }

    /// Get the edge information of the edge with index `idx`.
    pub fn get_edge_mut(&mut self, idx: usize) -> (usize, usize, &mut E) {
        let (next, to, edge_info) = &mut self.edges[idx];
        (*next, *to, edge_info)
    }

    /// Get the twin edge information of the edge with index `idx`.
    /// Often used in undirected graphs as the reverse edge.
    pub fn get_twin_edge_mut(&mut self, idx: usize) -> (usize, usize, &mut E) {
        let (next, to, edge_info) = &mut self.edges[TWIN(idx)];
        (*next, *to, edge_info)
    }
}

pub const TWIN: fn(usize) -> usize = |idx| ((idx - 1) ^ 1) + 1;
pub mod degree;
pub mod dijkstra;
pub mod distance;
pub mod hierholzer;
pub mod scc;

#[doc(inline)]
pub use self::dijkstra::dijkstra;
#[doc(inline)]
pub use self::hierholzer::{hierholzer_directed, hierholzer_undirected};
#[doc(inline)]
pub use self::scc::scc;

#[doc(inline)]
pub use self::degree::Degree;
#[doc(inline)]
pub use self::distance::Distance;
