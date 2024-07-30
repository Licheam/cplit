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
pub use self::dijkstra::dijkstra;
pub use self::distance::Distance;
