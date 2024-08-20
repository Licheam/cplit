use std::collections::LinkedList;
use std::ptr::{addr_of, addr_of_mut};

use crate::graph::{Graph, TWIN};
use crate::utils::Flag;

fn dfs_directed<V, E>(node: usize, graph: &Graph<V, E>, cur: &mut Vec<usize>) -> LinkedList<usize>
where
    V: Default + Clone,
    E: Default + Clone,
{
    let mut res = LinkedList::new();

    while cur[node] != 0 {
        let (next, v, _) = graph.edges[cur[node]];
        cur[node] = next;
        res.append(&mut dfs_directed(v, graph, cur));
    }

    res.push_back(node);
    res
}

fn dfs_undirected<V, E>(
    node: usize,
    graph: &mut Graph<V, E>,
    cur: &mut Vec<usize>,
) -> LinkedList<usize>
where
    V: Default + Clone,
    E: Default + Clone + Flag,
{
    let mut res = LinkedList::new();

    unsafe { (*addr_of!(*graph)).get_edges_enum_from_once(&mut *addr_of_mut!(cur[node])) }
        .for_each(|(idx, (&v, _))| {
            if !graph.edges[idx].2.get() {
                graph.edges[idx].2.set(true);
                graph.edges[TWIN(idx)].2.set(true);
                res.append(&mut dfs_undirected(v, graph, cur));
            }
        });

    res.push_back(node);
    res
}

/// Hierholzer's algorithm for directed graph.
pub fn hierholzer_directed<V, E>(start: usize, graph: &Graph<V, E>) -> Vec<usize>
where
    V: Default + Clone,
    E: Default + Clone,
{
    let mut cur = graph.head.clone();
    let mut res: Vec<_> = dfs_directed(start, graph, &mut cur).into_iter().collect();
    res.reverse();
    res
}

/// Hierholzer's algorithm for undirected graph.
pub fn hierholzer_undirected<V, E>(start: usize, graph: &mut Graph<V, E>) -> Vec<usize>
where
    V: Default + Clone,
    E: Default + Clone + Flag,
{
    let mut cur = graph.head.clone();
    let mut res: Vec<_> = dfs_undirected(start, graph, &mut cur).into_iter().collect();
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use crate::fscanln;
    use crate::graph::{hierholzer_directed, hierholzer_undirected, Degree, Graph};
    use std::io::{BufReader, Cursor};

    #[test]
    fn luogu_p7771() {
        let mut reader = BufReader::new(Cursor::new(
            r#"
4 6
1 3
2 1
4 2
3 3
1 2
3 4
"#,
        ));
        let (n, m): (usize, usize);
        fscanln!(reader, n, m);
        let mut graph = Graph::<(usize, usize), ()>::new(n);
        for _ in 0..m {
            let (u, v): (usize, usize);
            fscanln!(reader, u, v);
            graph.add_edge(u, v, ());
            graph.nodes[v].0 += 1;
            graph.nodes[u].1 += 1;
        }

        let mut start = 0;
        for i in 1..=n {
            if graph.nodes[i].out_dgr() > graph.nodes[i].in_dgr() + 1 {
                unreachable!();
            } else if graph.nodes[i].out_dgr() > graph.nodes[i].in_dgr() {
                if start != 0 {
                    unreachable!();
                }
                start = i;
            }
        }

        for i in 1..=n {
            graph.sort_edges(i);
        }

        let ans = hierholzer_directed(start, &graph);
        assert_eq!(ans, vec![1, 2, 1, 3, 3, 4, 2]);
    }

    #[test]
    fn luogu_p2731() {
        let mut reader = BufReader::new(Cursor::new(
            r#"
9
1 2
2 3
3 4
4 2
4 5
2 5
5 6
5 7
4 6
"#,
        ));
        let (n, m): (usize, usize);
        fscanln!(reader, m);
        let mut graph = Graph::<usize, bool>::empty();
        for _ in 0..m {
            let (u, v): (usize, usize);
            fscanln!(reader, u, v);
            graph.add_edge(u, v, false);
            graph.add_edge(v, u, false);
            graph.nodes[v] += 1;
            graph.nodes[u] += 1;
        }
        n = graph.len_nodes();

        for i in 1..=n {
            graph.sort_edges(i);
        }

        let mut start = 0;
        for i in 1..=n {
            if graph.nodes[i].dgr() != 0 && start == 0 {
                start = i;
            }
            if graph.nodes[i].dgr() % 2 != 0 {
                break;
            }
        }

        let ans = hierholzer_undirected(start, &mut graph);
        assert_eq!(ans, vec![1, 2, 3, 4, 2, 5, 4, 6, 5, 7]);
    }
}
