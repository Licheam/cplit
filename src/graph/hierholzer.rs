use crate::graph::{Degree, Graph};

fn dfs<V, E>(node: usize, graph: &Graph<V, E>, cur: &mut Vec<usize>) -> Vec<usize>
where
    V: Default + Clone + Degree,
    E: Default + Clone,
{
    let mut res = vec![];

    while cur[node] != 0 {
        let (next, v, _) = graph.edges[cur[node]];
        cur[node] = next;
        res.extend(dfs(v, graph, cur));
    }

    res.push(node);
    res
}

pub fn hierholzer<V, E>(graph: &Graph<V, E>) -> Option<Vec<usize>>
where
    V: Default + Clone + Degree,
    E: Default + Clone,
{
    let n = graph.nodes.len() - 1;
    let mut s = 0;
    for i in 1..=n {
        if graph.nodes[i].out_dgr() > graph.nodes[i].in_dgr() + 1 {
            return None;
        } else if graph.nodes[i].out_dgr() > graph.nodes[i].in_dgr() {
            if s != 0 {
                return None;
            }
            s = i;
        }
    }
    let mut cur = graph.head.clone();
    let mut res = dfs(if s != 0 { s } else { 1 }, graph, &mut cur);
    if res.len() != graph.edges.len() {
        return None;
    }
    res.reverse();
    Some(res)
}

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use crate::{fscanln, graph::hierholzer};
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

        for i in 1..=n {
            graph.sort_edges(i);
        }

        let ans = hierholzer(&graph);
        assert_eq!(ans, Some(vec![1, 2, 1, 3, 3, 4, 2]));
    }
}
