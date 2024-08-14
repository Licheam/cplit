use std::collections::LinkedList;

use crate::graph::Graph;

fn dfs<V, E>(node: usize, graph: &Graph<V, E>, cur: &mut Vec<usize>) -> LinkedList<usize>
where
    V: Default + Clone,
    E: Default + Clone,
{
    let mut res = LinkedList::new();
    
    while cur[node] != 0 {
        let (next, v, _) = graph.edges[cur[node]];
        cur[node] = next;
        res.append(&mut dfs(v, graph, cur));
    }

    res.push_back(node);
    res
}

pub fn hierholzer<V, E>(start: usize, graph: &Graph<V, E>) -> Option<Vec<usize>>
where
    V: Default + Clone,
    E: Default + Clone,
{
    let mut cur = graph.head.clone();
    let mut res: Vec<_> = dfs(start, graph, &mut cur).into_iter().collect();
    if res.len() != graph.edges.len() {
        return None;
    }
    res.reverse();
    Some(res)
}

#[cfg(test)]
mod tests {
    use crate::fscanln;
    use crate::graph::{hierholzer, Degree, Graph};
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
                assert!(false);
            } else if graph.nodes[i].out_dgr() > graph.nodes[i].in_dgr() {
                if start != 0 {
                    assert!(false);
                }
                start = i;
            }
        }

        for i in 1..=n {
            graph.sort_edges(i);
        }

        let ans = hierholzer(start, &graph);
        assert_eq!(ans, Some(vec![1, 2, 1, 3, 3, 4, 2]));
    }
}
