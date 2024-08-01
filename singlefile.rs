pub mod cplit { #![allow(clippy::needless_doctest_main)]  pub mod data_structure {  pub mod binary_indexed_tree { use crate::cplit::num::{Numeric, NumericAssOps, NumericOps}; use std::collections::binary_heap::Iter; use std::iter::successors; use std::ops::{Bound, RangeBounds};  macro_rules! low_bit { ($idx: expr) => { ($idx & (!$idx + 1)) }; }  #[derive(Debug)] pub struct BinaryIndexedTree<N> where N: Numeric + NumericOps + NumericAssOps + Clone + Copy, { body: Vec<N>, }  impl<N> BinaryIndexedTree<N> where N: Numeric + NumericOps + NumericAssOps + Clone + Copy, { pub fn with_len(n: usize) -> Self { Self { body: vec![N::ZERO; n + 1], } }  pub fn len(&self) -> usize { self.body.len() - 1 }  pub fn add(&mut self, mut idx: usize, delta: N) { if idx >= self.len() || idx == 0 { panic!( "Index out of bounds: the range is 1..={} but the index is {}", self.len(), idx ); } while idx <= self.len() { self.body[idx] += delta; idx += low_bit!(idx); } }  pub fn sum(&self, bounds: impl RangeBounds<usize>) -> N { let mut start = match bounds.start_bound() { Bound::Included(&s) => s, Bound::Excluded(&s) => s + 1, Bound::Unbounded => 1, }; let mut end = match bounds.end_bound() { Bound::Included(&e) => e + 1, Bound::Excluded(&e) => e, Bound::Unbounded => self.len() + 1, };  if !(1..=self.len()).contains(&start) || !(1..=self.len() + 1).contains(&end) { panic!( "Query out of bounds: the range is 1..={} but the query is {}..{}", self.len(), start, end, ); } if start >= end { return N::ZERO; }  start -= 1; end -= 1;  let mut s = N::ZERO; while end > start { s += self.body[end]; end -= low_bit!(end); } while start > end { s -= self.body[start]; start -= low_bit!(start); } s }  fn init(&mut self) { let len = self.len(); successors(Some(1), |&step| Some(step << 1)) .take_while(|&step| step << 1 <= len) .for_each(|step: usize| { successors(Some(step), |idx| Some(idx + (step << 1))) .take_while(|&idx| idx + step <= len) .for_each(|idx| { let t = self.body[idx]; self.body[idx + step] += t; }); }); } }  impl<N, T> From<T> for BinaryIndexedTree<N> where N: Numeric + NumericOps + NumericAssOps + Clone + Copy, T: Into<Vec<N>>, { fn from(a: T) -> Self { let mut bit = BinaryIndexedTree { body: a.into() }; bit.init(); bit } } } pub mod disjoint_set_union {  #[derive(Debug)] pub struct DisjointSetUnion { fa: Vec<usize>, }  impl DisjointSetUnion { pub fn new(n: usize) -> Self { let fa = (0..=n).collect(); DisjointSetUnion { fa } }  pub fn find(&mut self, x: usize) -> usize { if self.fa[x] == x { x } else { self.fa[x] = self.find(self.fa[x]); self.fa[x] } }  pub fn join(&mut self, x: usize, y: usize) { let fx = self.find(x); self.fa[fx] = self.find(y); } } } pub mod segment_tree { #[derive(Debug)] pub struct SegmentTree { sum: Vec<i64>, tag: Vec<i64>, }  impl SegmentTree { pub fn new(n: usize) -> Self { Self { sum: vec![0; n << 2], tag: vec![0; n << 2], } }  fn pushup(&mut self, x: usize) { let SegmentTree { sum, .. } = self; sum[x] = sum[x << 1] + sum[x << 1 | 1]; }  fn pushdown(&mut self, x: usize, l: usize, r: usize) { let SegmentTree { sum, tag } = self; let m = (l + r) >> 1; sum[x << 1] += tag[x] * (m - l + 1) as i64; tag[x << 1] += tag[x]; sum[x << 1 | 1] += tag[x] * (r - m) as i64; tag[x << 1 | 1] += tag[x]; tag[x] = 0; }  pub fn build(&mut self, x: usize, l: usize, r: usize, a: &Vec<i64>) { let SegmentTree { sum, .. } = self; if l == r { sum[x] = a[l - 1] } else { let m = (l + r) >> 1; self.build(x << 1, l, m, a); self.build(x << 1 | 1, m + 1, r, a); self.pushup(x); } }  pub fn modify(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize, del: i64) { if ql <= l && r <= qr { let SegmentTree { sum, tag } = self; sum[x] += del * (r - l + 1) as i64; tag[x] += del; } else { self.pushdown(x, l, r); let m = (l + r) >> 1; if ql <= m { self.modify(x << 1, l, m, ql, qr, del) } if m < qr { self.modify(x << 1 | 1, m + 1, r, ql, qr, del) } self.pushup(x); } }  pub fn query(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 { if ql <= l && r <= qr { self.sum[x] } else { self.pushdown(x, l, r); let m = (l + r) >> 1; (if ql <= m { self.query(x << 1, l, m, ql, qr) } else { 0 }) + (if m < qr { self.query(x << 1 | 1, m + 1, r, ql, qr) } else { 0 }) } } } }  #[doc(inline)] pub use self::binary_indexed_tree::BinaryIndexedTree; #[doc(inline)] pub use self::disjoint_set_union::DisjointSetUnion; #[doc(inline)] pub use self::segment_tree::SegmentTree; } pub mod graph {  pub struct Graph<V, E> where V: Default + Clone, E: Clone, { pub nodes: Vec<V>, pub edges: Vec<Vec<(usize, E)>>, }  impl<V, E> Graph<V, E> where V: Default + Clone, E: Clone, { pub fn new(n: usize) -> Self { Self { nodes: vec![V::default(); n + 1], edges: vec![vec![]; n + 1], } }  pub fn add_edge(&mut self, from: usize, to: usize, edge: E) { self.edges[from].push((to, edge)); } }  pub mod dijkstra { use super::Distance; use crate::cplit::num::{Numeric, NumericAssOps, NumericCmpOps, NumericOps}; use std::cmp::Reverse; use std::collections::BinaryHeap;  pub fn dijkstra<V, E, N>(source: usize, graph: &super::Graph<V, E>) -> Vec<Option<N>> where N: Numeric + NumericOps + NumericCmpOps + NumericAssOps + Clone + Copy, V: Default + Clone, E: Clone + Distance<N>, { let n = graph.nodes.len() - 1; let mut dist = vec![None; n + 1]; let mut visited = vec![false; n + 1]; dist[source] = Some(N::ZERO); let mut pq = BinaryHeap::new(); pq.push((Reverse(N::ZERO), source)); while let Some((_, u)) = pq.pop() { if visited[u] { continue; } visited[u] = true; for (v, e) in &graph.edges[u] { if dist[*v].map_or(true, |distv| distv > dist[u].unwrap() + e.dist()) { dist[*v] = Some(dist[u].unwrap() + e.dist()); pq.push((Reverse(dist[*v].unwrap()), *v)); } } } dist } } pub mod distance { use crate::cplit::num::Numeric; pub trait Distance<N> where N: Numeric + Copy, { fn dist(&self) -> N; }  impl<N> Distance<N> for N where N: Numeric + Copy, { fn dist(&self) -> N { *self } } } #[doc(inline)] pub use self::dijkstra::dijkstra; #[doc(inline)] pub use self::distance::Distance; } mod io { #[macro_use] mod macros { #[macro_export] macro_rules! fscanln { ($reader:expr, $($i:expr), +) => {{ #[allow(unused_imports)] use std::io::BufRead; let mut iter = std::iter::repeat_with(|| { let mut buf = String::new(); $reader.read_line(&mut buf).unwrap(); buf.split_whitespace() .map(|x| x.to_string()) .collect::<Vec<_>>() }) .flatten(); $( $i = iter.next().unwrap().parse().unwrap(); )* }};  ($reader:expr, $coll:expr ; $n:expr) => {{ #[allow(unused_imports)] use std::io::BufRead; $coll = std::iter::once(Default::default()) .chain( std::iter::repeat_with(|| { let mut buf = String::new(); $reader.read_line(&mut buf).unwrap(); buf.split_whitespace() .map(|x| x.parse().unwrap()) .collect::<Vec<_>>() }) .flatten() .take($n), ) .collect(); }};  ($reader:expr, $coll:expr ;) => {{ #[allow(unused_imports)] use std::io::BufRead; let mut buf = String::new(); $reader.read_line(&mut buf).unwrap(); $coll = std::iter::once(Default::default()) .chain(buf.split_whitespace().map(|x| x.parse().unwrap())) .collect(); }}; }  #[macro_export] macro_rules! scanln { ($($i:expr), +) => { $crate::fscanln!(std::io::stdin(), $($i), +); };  ($coll:expr ; $n:expr) => { $crate::fscanln!(std::io::stdin(), $($i), +); };  ($coll:expr ;) => { $crate::fscanln!(std::io::stdin(), $($i), +); }; } } } pub mod num { use std::cmp::{Eq, Ord, PartialEq, PartialOrd}; use std::ops::{Add, Div, Mul, Rem, Sub}; use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign}; pub mod bounds { pub trait LowerBounded { const MIN: Self; }  pub trait UpperBounded { const MAX: Self; }  pub trait Bounded: LowerBounded + UpperBounded {}  macro_rules! bounded_trait_impl { ($t:ty, $min:expr, $max:expr) => { impl LowerBounded for $t { const MIN: Self = $min; }  impl UpperBounded for $t { const MAX: Self = $max; } }; }  bounded_trait_impl!(usize, usize::MIN, usize::MAX); bounded_trait_impl!(u8, u8::MIN, u8::MAX); bounded_trait_impl!(u16, u16::MIN, u16::MAX); bounded_trait_impl!(u32, u32::MIN, u32::MAX); bounded_trait_impl!(u64, u64::MIN, u64::MAX); bounded_trait_impl!(u128, u128::MIN, u128::MAX);  bounded_trait_impl!(isize, isize::MIN, isize::MAX); bounded_trait_impl!(i8, i8::MIN, i8::MAX); bounded_trait_impl!(i16, i16::MIN, i16::MAX); bounded_trait_impl!(i32, i32::MIN, i32::MAX); bounded_trait_impl!(i64, i64::MIN, i64::MAX); bounded_trait_impl!(i128, i128::MIN, i128::MAX);  bounded_trait_impl!(f32, f32::MIN, f32::MAX); bounded_trait_impl!(f64, f64::MIN, f64::MAX); } #[doc(inline)] pub use self::bounds::{Bounded, LowerBounded, UpperBounded};  pub trait Numeric: Default + Zero + One {}  impl<T> Numeric for T where T: Default + Zero + One {}  pub trait NumericOps<Rhs = Self, Output = Self>: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> { }  impl<T, Rhs, Output> NumericOps<Rhs, Output> for T where T: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> { }  pub trait NumericAssOps<Rhs = Self>: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs> { }  impl<T, Rhs> NumericAssOps<Rhs> for T where T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs> { }  pub trait NumericCmpOps<Rhs = Self>: PartialEq<Rhs> + PartialOrd<Rhs> + Eq + Ord {}  impl<T, Rhs> NumericCmpOps<Rhs> for T where T: PartialEq<Rhs> + PartialOrd<Rhs> + Eq + Ord {}  pub trait Zero { const ZERO: Self; }  pub trait One { const ONE: Self; }  macro_rules! zero_trait_impl { ($name:ident for $($t:ty)*) => ($( impl $name for $t { const ZERO: Self = 0 as $t; } )*) }  zero_trait_impl!(Zero for usize u8 u16 u32 u64 u128); zero_trait_impl!(Zero for isize i8 i16 i32 i64 i128); zero_trait_impl!(Zero for f32 f64);  macro_rules! one_trait_impl { ($name:ident for $($t:ty)*) => ($( impl $name for $t { const ONE: Self = 1 as $t; } )*) }  one_trait_impl!(One for usize u8 u16 u32 u64 u128); one_trait_impl!(One for isize i8 i16 i32 i64 i128); one_trait_impl!(One for f32 f64); } }

