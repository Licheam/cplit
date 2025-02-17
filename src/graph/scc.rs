use crate::graph::Graph;

fn tarjan<V, E>(
    graph: &Graph<V, E>,
    u: usize,
    cnt: &mut usize,
    dfn: &mut Vec<usize>,
    low: &mut Vec<usize>,
    sc: &mut usize,
    scc: &mut Vec<usize>,
    stack: &mut Vec<usize>,
    in_stack: &mut Vec<bool>,
) where
    V: Default + Clone,
    E: Default + Clone,
{
    *cnt += 1;
    low[u] = *cnt;
    dfn[u] = *cnt;
    stack.push(u);
    in_stack[u] = true;
    for (&v, _) in graph.get_edges(u) {
        if dfn[v] == 0 {
            tarjan(graph, v, cnt, dfn, low, sc, scc, stack, in_stack);
            low[u] = low[u].min(low[v]);
        } else if in_stack[v] {
            low[u] = low[u].min(dfn[v]);
        }
    }
    if dfn[u] == low[u] {
        *sc += 1;
        loop {
            let v = stack.pop().unwrap();
            in_stack[v] = false;
            scc[v] = *sc;
            if u == v {
                break;
            }
        }
    }
}

pub fn scc<V, E>(graph: &Graph<V, E>) -> (usize, Vec<usize>)
where
    V: Default + Clone,
    E: Default + Clone,
{
    let n = graph.len_nodes();
    let mut dfn = vec![0; n + 1];
    let mut low = vec![0; n + 1];
    let mut sc = 0;
    let mut scc = vec![0; n + 1];
    let mut stack = vec![];
    let mut in_stack = vec![false; n + 1];
    let mut cnt = 0;
    for u in 1..=n {
        if dfn[u] == 0 {
            tarjan(
                graph,
                u,
                &mut cnt,
                &mut dfn,
                &mut low,
                &mut sc,
                &mut scc,
                &mut stack,
                &mut in_stack,
            );
        }
    }
    (sc, scc)
}

#[cfg(test)]
mod tests {
    use crate::fscanln;
    use crate::graph::{scc, Graph};
    use std::collections::VecDeque;
    use std::io::{BufReader, Cursor};

    #[test]
    fn luogu_p3387() {
        let mut reader = BufReader::new(Cursor::new(
            r#"
10 20
970 369 910 889 470 106 658 659 916 964 
3 2
3 6
3 4
9 5
8 3
5 8
9 1
9 7
9 8
7 5
3 7
7 8
1 7
10 2
1 10
4 8
2 6
3 1
3 5
8 5
"#,
        ));
        let (n, m): (usize, usize);
        fscanln!(reader, n, m);
        let a: Vec<usize>;
        fscanln!(reader, a;n);
        let mut graph = Graph::<_, ()>::from_nodes(a);
        for _ in 0..m {
            let (u, v): (usize, usize);
            fscanln!(reader, u, v);
            graph.add_edge(u, v, ());
        }
        let (sc, scc) = scc(&graph);
        let mut in_dgr: Vec<usize> = vec![0; sc + 1];
        let mut sum = vec![0; sc + 1];
        for u in 1..=n {
            sum[scc[u]] += graph.nodes[u];
            graph
                .get_edges(u)
                .filter(|&(&v, _)| scc[u] != scc[v])
                .for_each(|(&v, _)| {
                    in_dgr[scc[v]] += 1;
                });
        }
        let mut dp = vec![0; sc + 1];
        let mut q = VecDeque::new();
        let mut vis = vec![false; n + 1];
        for u in 1..=n {
            if in_dgr[scc[u]] == 0 && !vis[u] {
                dp[scc[u]] = sum[scc[u]];
                q.push_back(u);
                while !q.is_empty() {
                    let u = q.pop_front().unwrap();
                    let mut q2 = VecDeque::new();
                    vis[u] = true;
                    q2.push_back(u);
                    while !q2.is_empty() {
                        let u = q2.pop_front().unwrap();
                        graph.get_edges(u).for_each(|(&v, _)| {
                            if scc[u] != scc[v] {
                                dp[scc[v]] = dp[scc[v]].max(dp[scc[u]] + sum[scc[v]]);
                                in_dgr[scc[v]] -= 1;
                                if in_dgr[scc[v]] == 0 {
                                    q.push_back(v);
                                }
                            } else if !vis[v] {
                                vis[v] = true;
                                q2.push_back(v);
                            }
                        });
                    }
                }
            }
        }
        let ans = dp.iter().max().unwrap();
        assert_eq!(ans, &6911);
    }
}
