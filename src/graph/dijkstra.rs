use crate::graph::{Distance, Graph};
use crate::num::{Numeric, NumericAssOps, NumericCmpOps, NumericOps};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Dijkstra - Finding shortest paths from given vertex
///
/// For more information, see [Dijkstra algorithm](https://cp-algorithms.com/graph/dijkstra.html)
/// - Input:
///     - `source` - the source vertex
///     - `graph` - the graph with **positive** distance on edges
/// - Output:
///     - A vector of optional distances from the source vertex to each vertex
///
/// # Examples
///
/// ```no_run
/// use cplit::graph::{dijkstra, Graph};
/// use cplit::scanln;
///
/// fn main() {
///     let (n, m, s): (usize, usize, usize);
///     // Read the number of nodes, the number of edges, and the source node.
///     scanln!(n, m, s);
///     // Create a graph with n nodes and storing nothing in each node,
///     // and a usize in each edge as weight.
///     let mut graph = Graph::<(), usize>::new(n);
///     for _ in 0..m {
///         // Read an edge u->v with weight w.
///         let (u, v, w): (usize, usize, usize);
///         scanln!(u, v, w);
///         // Create an edge u->v with weight w.
///         graph.add_edge(u, v, w);
///     }
///     let dist = dijkstra(s, &graph);
///     println!("{:?}", dist);
/// }
/// ```
pub fn dijkstra<V, E, N>(source: usize, graph: &Graph<V, E>) -> Vec<Option<N>>
where
    N: Numeric + NumericOps + NumericCmpOps + NumericAssOps + Clone + Copy,
    V: Default + Clone,
    E: Default + Clone + Distance<N>,
{
    let n = graph.nodes.len() - 1;
    let mut dist = vec![None; n + 1];
    let mut visited = vec![false; n + 1];
    dist[source] = Some(N::ZERO);
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(N::ZERO), source));
    while let Some((_, u)) = pq.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        graph.get_edges(u).for_each(|(&v, e)| {
            if dist[v].map_or(true, |distv| distv > dist[u].unwrap() + e.dist()) {
                dist[v] = Some(dist[u].unwrap() + e.dist());
                pq.push((Reverse(dist[v].unwrap()), v));
            }
        });
    }
    dist
}

#[cfg(test)]
mod tests {
    use crate::fscanln;
    use crate::graph::{Graph, dijkstra};
    use std::io::{BufReader, Cursor};

    #[test]
    fn luogu_p4779() {
        let mut reader = BufReader::new(Cursor::new(
            r#"
4 6 1
1 2 2
2 3 2
2 4 1
1 3 5
3 4 3
1 4 4
"#,
        ));
        let (n, m, s): (usize, usize, usize);
        fscanln!(reader, n, m, s);
        let mut graph = Graph::<(), usize>::new(n);
        for _ in 0..m {
            let (u, v, w): (usize, usize, usize);
            fscanln!(reader, u, v, w);
            graph.add_edge(u, v, w);
        }
        let dist = dijkstra(s, &graph);
        assert_eq!(
            dist[1..],
            vec![0, 2, 4, 3].into_iter().map(Some).collect::<Vec<_>>()
        );
    }
}
