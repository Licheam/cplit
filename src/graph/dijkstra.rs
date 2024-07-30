use super::Distance;
use crate::num::{bounds::UpperBounded, Numeric, NumericAssOps, NumericCmpOps, NumericOps};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Find the shortest path from the source node to all other nodes in the graph.
pub fn dijkstra<V, E, N>(source: usize, graph: &super::Graph<V, E>) -> Vec<N>
where
    N: Numeric + UpperBounded + NumericOps + NumericCmpOps + NumericAssOps,
    V: Default + Clone,
    E: Clone + Distance<N>,
{
    let n = graph.nodes.len() - 1;
    let mut dist = vec![N::MAX; n + 1];
    let mut visited = vec![false; n + 1];
    dist[source] = N::ZERO;
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(N::ZERO), source));
    while let Some((_, u)) = pq.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        for (v, e) in &graph.edges[u] {
            if dist[*v] > dist[u] + e.dist() {
                dist[*v] = dist[u] + e.dist();
                pq.push((Reverse(dist[*v]), *v));
            }
        }
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
        assert_eq!(dist[1..], vec![0, 2, 4, 3]);
    }
}
