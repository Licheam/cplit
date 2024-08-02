/// Disjoint Set Union, also known as Union-Find Set.
///
/// For more information, see [Disjoint Set Union](https://cp-algorithms.com/data_structures/disjoint_set_union.html).
///
/// # Examples
///
/// ```no_run
/// use cplit::data_structure::DisjointSetUnion;
/// use cplit::scanln;
///
/// fn main() {
///     let (n, m): (usize, usize);
///     scanln!(n, m);
///     let mut dsu = DisjointSetUnion::with_len(n);
///     for _ in 0..m {
///         let (op, x, y): (usize, usize, usize);
///         scanln!(op, x, y);
///         match op {
///             1 => {
///                 dsu.union(x, y);
///             }
///             2 => {
///                 println!("{}", if dsu.find(x) == dsu.find(y) { 'Y' } else { 'N' });
///             }
///             _ => unreachable!(),
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct DisjointSetUnion {
    //TODO: Add rank to optimize the union operation. Add size maybe?
    parent: Vec<usize>,
}

impl DisjointSetUnion {
    /// Constructs a new disjoint set union with the specified `len`.
    pub fn with_len(len: usize) -> Self {
        DisjointSetUnion {
            parent: (0..=len).collect(),
        }
    }

    /// The length of the disjoint set union.
    pub fn len(&self) -> usize {
        self.parent.len() - 1
    }

    /// Returns `true` if the disjoint set union is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        self.parent[fx] = self.find(y);
    }
}

impl<T> From<T> for DisjointSetUnion
where
    T: Into<Vec<usize>>,
{
    /// Constructs a new disjoint set union with the specified `parent`.
    fn from(a: T) -> Self {
        DisjointSetUnion { parent: a.into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::DisjointSetUnion;
    use crate::fscanln;
    use std::io::{BufReader, Cursor};

    #[test]
    fn luogu_p3367() {
        let mut reader = BufReader::new(Cursor::new(
            r#"
4 7
2 1 2
1 1 2
2 1 2
1 3 4
2 1 4
1 2 3
2 1 4
"#,
        ));

        let (n, m): (usize, usize);
        fscanln!(reader, n, m);
        let mut dsu = DisjointSetUnion::with_len(n);
        let mut ans = String::new();
        for _ in 0..m {
            let (op, x, y): (usize, usize, usize);
            fscanln!(reader, op, x, y);
            match op {
                1 => {
                    dsu.union(x, y);
                }
                2 => {
                    ans.push(if dsu.find(x) == dsu.find(y) { 'Y' } else { 'N' });
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(ans, "NYNY");
    }
}
