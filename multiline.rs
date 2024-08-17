pub mod cplit {
	#![allow(clippy::needless_doctest_main)]
	
	pub mod data_structure {
		
		pub mod binary_indexed_tree {
			use crate::cplit::num::{Numeric, NumericAssOps, NumericOps};
			use std::ops::{Bound, RangeBounds};
			use std::vec;
			
			macro_rules! low_bit {
			    ($index: expr) => {
			        ($index & (!$index + 1))
			    };
			}
			
			#[derive(Debug)]
			pub struct BinaryIndexedTree<N>
			where
			    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
			{
			    body: Vec<N>,
			}
			
			impl<N> BinaryIndexedTree<N>
			where
			    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
			{
			    pub fn new() -> Self {
			        Self {
			            body: vec![N::ZERO],
			        }
			    }
			
			    pub fn with_capacity(capacity: usize) -> Self {
			        let mut body = Vec::with_capacity(capacity + 1);
			        body.push(N::ZERO);
			        Self { body }
			    }
			
			    pub fn len(&self) -> usize {
			        self.body.len() - 1
			    }
			
			    pub fn is_empty(&self) -> bool {
			        self.len() == 0
			    }
			
			    pub fn add(&mut self, mut index: usize, delta: N) {
			        if !(1..=self.len()).contains(&index) {
			            panic!(
			                "Index out of bounds: the range is 1..={} but the index is {}",
			                self.len(),
			                index
			            );
			        }
			        while index <= self.len() {
			            self.body[index] += delta;
			            index += low_bit!(index);
			        }
			    }
			
			    pub fn sum(&self, bounds: impl RangeBounds<usize>) -> N {
			        let mut start = match bounds.start_bound() {
			            Bound::Included(&s) => s,
			            Bound::Excluded(&s) => s + 1,
			            Bound::Unbounded => 1,
			        };
			        let mut end = match bounds.end_bound() {
			            Bound::Included(&e) => e + 1,
			            Bound::Excluded(&e) => e,
			            Bound::Unbounded => self.len() + 1,
			        };
			
			        if !(1..=self.len() + 1).contains(&start) || !(1..=self.len() + 1).contains(&end) {
			            panic!(
			                "Query out of bounds: the range is 1..={} but the query is {}..{}",
			                self.len(),
			                start,
			                end,
			            );
			        }
			        if start >= end {
			            return N::ZERO;
			        }
			
			        start -= 1;
			        end -= 1;
			
			        let mut s = N::ZERO;
			        while end > start {
			            s += self.body[end];
			            end -= low_bit!(end);
			        }
			        while start > end {
			            s -= self.body[start];
			            start -= low_bit!(start);
			        }
			        s
			    }
			
			    pub fn push(&mut self, value: N) {
			        let len = self.len() + 1;
			        let sum = self.sum(len - low_bit!(len) + 1..len);
			        self.body.push(value + sum);
			    }
			
			    pub fn pop(&mut self) -> Option<N> {
			        self.body.pop()
			    }
			
			    fn init(&mut self) {
			        for i in 1..=self.len() {
			            let j = i + low_bit!(i);
			            if j <= self.len() {
			                let t = self.body[i];
			                self.body[j] += t;
			            }
			        }
			    }
			}
			
			impl<N, T> From<T> for BinaryIndexedTree<N>
			where
			    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
			    T: Into<Vec<N>>,
			{
			    fn from(a: T) -> Self {
			        let mut bit = BinaryIndexedTree { body: a.into() };
			        bit.init();
			        bit
			    }
			}
			
			impl<N> Default for BinaryIndexedTree<N>
			where
			    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
			{
			    fn default() -> Self {
			        Self::new()
			    }
			}
		}
		pub mod disjoint_set_union {
			#[derive(Debug)]
			pub struct DisjointSetUnion {
			    parent: Vec<usize>,
			}
			
			impl DisjointSetUnion {
			    pub fn with_len(len: usize) -> Self {
			        DisjointSetUnion {
			            parent: (0..=len).collect(),
			        }
			    }
			
			    pub fn len(&self) -> usize {
			        self.parent.len() - 1
			    }
			
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
			    fn from(a: T) -> Self {
			        DisjointSetUnion { parent: a.into() }
			    }
			}
		}
		pub mod segment_tree {
			pub mod ops {
				use crate::cplit::num::{Numeric, NumericOps};
				use std::convert::TryFrom;
				use std::fmt::Debug;
				use std::marker::PhantomData;
				
				pub trait Operation<V, T>
				where
				    V: Clone + Copy,
				    T: Clone + Copy,
				{
				    const COMBINE: fn(left_val: V, right_val: V) -> V;
				
				    const PUSH_VAL: fn(val: V, tag: T, len: usize) -> V;
				
				    const PUSH_TAG: fn(child_tag: T, tag: T) -> T;
				
				    const VAL_IDENTITY: V;
				
				    const TAG_IDENTITY: T;
				}
				
				#[derive(Debug)]
				pub struct AddSum;
				
				impl<V> Operation<V, V> for AddSum
				where
				    V: Numeric + NumericOps + Clone + Copy + TryFrom<usize>,
				    <V as TryFrom<usize>>::Error: Debug,
				{
				    const COMBINE: fn(V, V) -> V = |left_val, right_val| left_val + right_val;
				    const PUSH_VAL: fn(V, V, usize) -> V = |val, tag, len| val + tag * V::try_from(len).unwrap();
				    const PUSH_TAG: fn(V, V) -> V = |child_tag, tag| child_tag + tag;
				    const TAG_IDENTITY: V = V::ZERO;
				    const VAL_IDENTITY: V = V::ZERO;
				}
				
				pub struct OperationPair<V1, V2, T1, T2, O1, O2>
				where
				    V1: Clone + Copy,
				    V2: Clone + Copy,
				    T1: Clone + Copy,
				    T2: Clone + Copy,
				    O1: Operation<V1, T1>,
				    O2: Operation<V2, T2>,
				{
				    _phantoms: (
				        PhantomData<V1>,
				        PhantomData<V2>,
				        PhantomData<T1>,
				        PhantomData<T2>,
				        PhantomData<O1>,
				        PhantomData<O2>,
				    ),
				}
				
				impl<V1, V2, T1, T2, O1, O2> OperationPair<V1, V2, T1, T2, O1, O2>
				where
				    V1: Clone + Copy,
				    V2: Clone + Copy,
				    T1: Clone + Copy,
				    T2: Clone + Copy,
				    O1: Operation<V1, T1>,
				    O2: Operation<V2, T2>,
				{
				    const COMBINE: fn(left_val: (V1, V2), right_val: (V1, V2)) -> (V1, V2) =
				        |left_val, right_val| {
				            (
				                O1::COMBINE(left_val.0, right_val.0),
				                O2::COMBINE(left_val.1, right_val.1),
				            )
				        };
				
				    const PUSH_VAL: fn((V1, V2), (T1, T2), usize) -> (V1, V2) = |val, tag, len| {
				        (
				            O1::PUSH_VAL(val.0, tag.0, len),
				            O2::PUSH_VAL(val.1, tag.1, len),
				        )
				    };
				
				    const PUSH_TAG: fn((T1, T2), (T1, T2)) -> (T1, T2) = |child_tag, tag| {
				        (
				            O1::PUSH_TAG(child_tag.0, tag.0),
				            O2::PUSH_TAG(child_tag.1, tag.1),
				        )
				    };
				
				    const TAG_IDENTITY: (T1, T2) = (O1::TAG_IDENTITY, O2::TAG_IDENTITY);
				
				    const VAL_IDENTITY: (V1, V2) = (O1::VAL_IDENTITY, O2::VAL_IDENTITY);
				}
			}
			#[doc(inline)]
			pub use self::ops::{AddSum, Operation, OperationPair};
			
			use std::fmt::Debug;
			use std::marker::PhantomData;
			
			#[derive(Debug)]
			pub struct SegmentTree<V, T, O>
			where
			    V: Clone + Copy,
			    T: Clone + Copy,
			    O: Operation<V, T>,
			{
			    val: Vec<V>,
			    tag: Vec<T>,
			    len: usize,
			    phantom: PhantomData<O>,
			}
			
			impl<V, T, O> SegmentTree<V, T, O>
			where
			    V: Clone + Copy,
			    T: Clone + Copy,
			    O: Operation<V, T>,
			{
			    pub fn len(&self) -> usize {
			        self.len
			    }
			
			    pub fn is_empty(&self) -> bool {
			        self.len == 0
			    }
			
			    fn pushup(&mut self, x: usize) {
			        self.val[x] = O::COMBINE(self.val[x << 1], self.val[x << 1 | 1]);
			    }
			
			    fn pushdown(&mut self, x: usize, l: usize, r: usize) {
			        let m = (l + r) >> 1;
			        self.val[x << 1] = O::PUSH_VAL(self.val[x << 1], self.tag[x], m - l + 1);
			        self.val[x << 1 | 1] = O::PUSH_VAL(self.val[x << 1 | 1], self.tag[x], r - m);
			        self.tag[x << 1] = O::PUSH_TAG(self.tag[x << 1], self.tag[x]);
			        self.tag[x << 1 | 1] = O::PUSH_TAG(self.tag[x << 1 | 1], self.tag[x]);
			        self.tag[x] = O::TAG_IDENTITY;
			    }
			
			    pub fn modify(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize, delta: T) {
			        if ql <= l && r <= qr {
			            self.val[x] = O::PUSH_VAL(self.val[x], delta, r - l + 1);
			            self.tag[x] = O::PUSH_TAG(self.tag[x], delta);
			        } else {
			            self.pushdown(x, l, r);
			            let m = (l + r) >> 1;
			            if ql <= m {
			                self.modify(x << 1, l, m, ql, qr, delta)
			            }
			            if m < qr {
			                self.modify(x << 1 | 1, m + 1, r, ql, qr, delta)
			            }
			            self.pushup(x);
			        }
			    }
			
			    pub fn query(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize) -> V {
			        if ql <= l && r <= qr {
			            self.val[x]
			        } else {
			            self.pushdown(x, l, r);
			            let m = (l + r) >> 1;
			            O::COMBINE(
			                if ql <= m {
			                    self.query(x << 1, l, m, ql, qr)
			                } else {
			                    O::VAL_IDENTITY
			                },
			                if m < qr {
			                    self.query(x << 1 | 1, m + 1, r, ql, qr)
			                } else {
			                    O::VAL_IDENTITY
			                },
			            )
			        }
			    }
			
			    fn init(&mut self, x: usize, l: usize, r: usize, a: &Vec<V>) {
			        if l == r {
			            self.val[x] = a[l]
			        } else {
			            let m = (l + r) >> 1;
			            self.init(x << 1, l, m, a);
			            self.init(x << 1 | 1, m + 1, r, a);
			            self.pushup(x);
			        }
			    }
			}
			
			impl<V, T, O, Q> From<Q> for SegmentTree<V, T, O>
			where
			    V: Clone + Copy,
			    T: Clone + Copy,
			    O: Operation<V, T>,
			    Q: Into<Vec<V>>,
			{
			    fn from(a: Q) -> Self {
			        let v = a.into();
			        let len = v.len() - 1;
			        let mut st = SegmentTree {
			            val: vec![O::VAL_IDENTITY; 1 + (len << 2)],
			            tag: vec![O::TAG_IDENTITY; 1 + (len << 2)],
			            len,
			            phantom: PhantomData,
			        };
			        st.init(1, 1, len, &v);
			        st
			    }
			}
		}
		
		#[doc(inline)]
		pub use self::binary_indexed_tree::BinaryIndexedTree;
		#[doc(inline)]
		pub use self::disjoint_set_union::DisjointSetUnion;
		#[doc(inline)]
		pub use self::segment_tree::SegmentTree;
	}
	pub mod general {
		
		pub mod binary_search {
			use crate::cplit::num::{Bounded, Numeric, NumericCmpOps, NumericOps};
			use std::ops::{Bound, RangeBounds};
			
			pub fn binary_search<N>(bounds: impl RangeBounds<N>, f: impl Fn(N) -> bool) -> N
			where
			    N: Numeric + NumericOps + NumericCmpOps + Copy + Clone + Bounded,
			{
			    let mut left = match bounds.start_bound() {
			        Bound::Included(&s) => s,
			        Bound::Excluded(&s) => s + N::ONE,
			        Bound::Unbounded => N::MIN,
			    };
			    let mut right = match bounds.end_bound() {
			        Bound::Included(&e) => e + N::ONE,
			        Bound::Excluded(&e) => e,
			        Bound::Unbounded => N::MAX,
			    };
			
			    while left < right {
			        let mid = (left + right) / (N::ONE + N::ONE);
			        if f(mid) {
			            right = mid;
			        } else {
			            left = mid + N::ONE;
			        }
			    }
			
			    left
			}
		}
		#[doc(inline)]
		pub use self::binary_search::binary_search;
	}
	pub mod geometry {
		use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
		
		use crate::cplit::utils::F64;
		
		#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
		pub struct Point {
		    pub x: F64,
		    pub y: F64,
		}
		
		impl Add for Point {
		    type Output = Self;
		
		    fn add(self, rhs: Self) -> Self::Output {
		        Self {
		            x: self.x.add(rhs.x),
		            y: self.y.add(rhs.y),
		        }
		    }
		}
		
		impl Sub for Point {
		    type Output = Self;
		
		    fn sub(self, rhs: Self) -> Self::Output {
		        Self {
		            x: self.x.sub(rhs.x),
		            y: self.y.sub(rhs.y),
		        }
		    }
		}
		
		impl Mul<F64> for Point {
		    type Output = Point;
		
		    fn mul(self, rhs: F64) -> Self::Output {
		        Self {
		            x: self.x.mul(rhs),
		            y: self.y.mul(rhs),
		        }
		    }
		}
		
		impl Div<F64> for Point {
		    type Output = Point;
		
		    fn div(self, rhs: F64) -> Self::Output {
		        Self {
		            x: self.x.div(rhs),
		            y: self.y.div(rhs),
		        }
		    }
		}
		
		impl Mul<Point> for Point {
		    type Output = F64;
		
		    fn mul(self, rhs: Point) -> Self::Output {
		        self.x * rhs.x + self.y * rhs.y
		    }
		}
		
		impl BitXor<Point> for Point {
		    type Output = F64;
		
		    fn bitxor(self, rhs: Point) -> Self::Output {
		        self.x * rhs.y - self.y * rhs.x
		    }
		}
		
		impl AddAssign for Point {
		    fn add_assign(&mut self, rhs: Self) {
		        self.x.add_assign(rhs.x);
		        self.y.add_assign(rhs.y);
		    }
		}
		
		impl SubAssign for Point {
		    fn sub_assign(&mut self, rhs: Self) {
		        self.x.sub_assign(rhs.x);
		        self.y.sub_assign(rhs.y);
		    }
		}
		
		impl MulAssign<F64> for Point {
		    fn mul_assign(&mut self, rhs: F64) {
		        self.x.mul_assign(rhs);
		        self.y.mul_assign(rhs);
		    }
		}
		
		impl DivAssign<F64> for Point {
		    fn div_assign(&mut self, rhs: F64) {
		        self.x.div_assign(rhs);
		        self.y.div_assign(rhs);
		    }
		}
		
		type Vector = Point;
		
		impl Vector {
		    pub fn length(&self) -> F64 {
		        F64((self.x * self.x + self.y * self.y).0.sqrt())
		    }
		
		    pub fn angle(&self, other: &Self) -> F64 {
		        F64((*self * *other / self.length() / other.length()).0.acos())
		    }
		
		    pub fn area2(&self, other: &Self) -> F64 {
		        *self ^ *other
		    }
		
		    pub fn rotate(&self, rad: F64) -> Self {
		        Self {
		            x: self.x * F64(rad.0.cos()) - self.y * F64(rad.0.sin()),
		            y: self.x * F64(rad.0.sin()) + self.y * F64(rad.0.cos()),
		        }
		    }
		
		    pub fn normal(&self) -> Self {
		        let len = self.length();
		        Self {
		            x: -self.y / len,
		            y: self.x / len,
		        }
		    }
		
		    pub fn to_left_test(&self, p: &Vector) -> bool {
		        *self ^ *p > F64(0.0)
		    }
		}
		
		pub struct Line {
		    pub p1: Point,
		    pub p2: Point,
		}
		
		impl Line {
		    pub fn new(p1: Point, p2: Point) -> Self {
		        Self { p1, p2 }
		    }
		
		    pub fn at(&self, t: F64) -> Point {
		        self.p1 + (self.p2 - self.p1) / (self.p2 - self.p1).length() * t
		    }
		
		    pub fn on_line(&self, p: &Point) -> bool {
		        (*p - self.p1) ^ (self.p2 - self.p1) == F64(0.0)
		    }
		
		    pub fn intersects_with_line(&self, other: &Self) -> bool {
		        (self.p2 - self.p1) ^ (other.p2 - other.p1) != F64(0.0)
		    }
		
		    pub fn get_intersection(&self, other: &Self) -> Option<Point> {
		        if !self.intersects_with_line(other) {
		            return None;
		        }
		        Some(
		            self.at((other.p2 - other.p1) ^ (self.p1 - other.p1))
		                / ((self.p2 - self.p1) ^ (other.p2 - other.p1)),
		        )
		    }
		
		    pub fn get_distance(&self, p: &Point) -> F64 {
		        F64(((self.p2 - self.p1) ^ (*p - self.p1)).0.abs())
		    }
		
		    pub fn get_projection(&self, p: &Point) -> Point {
		        self.at((*p - self.p1) * (self.p2 - self.p1))
		    }
		}
		
		type Segment = Line;
		
		impl Segment {
		    pub fn on_segment(&self, p: &Point) -> bool {
		        self.on_line(p) && (*p - self.p1) * (*p - self.p2) <= F64(0.0)
		    }
		
		    pub fn on_segment_strict(&self, p: &Point) -> bool {
		        self.on_line(p) && (*p - self.p1) * (*p - self.p2) < F64(0.0)
		    }
		}
	}
	pub mod graph {
		
		use std::cmp::max;
		use std::cmp::Ordering::{self, Less};
		use std::iter::from_fn;
		pub struct Graph<V = (), E = ()>
		where
		    V: Default + Clone,
		    E: Default + Clone,
		{
		    pub nodes: Vec<V>,
		
		    pub head: Vec<usize>,
		
		    pub edges: Vec<(usize, usize, E)>,
		
		    erased: Vec<usize>,
		}
		
		impl<V, E> Graph<V, E>
		where
		    V: Default + Clone,
		    E: Default + Clone,
		{
		    pub fn new(n: usize) -> Self {
		        Self {
		            nodes: vec![V::default(); n + 1],
		            head: vec![0; n + 1],
		            edges: vec![Default::default()],
		            erased: Vec::new(),
		        }
		    }
		
		    pub fn empty() -> Self {
		        Self::new(0)
		    }
		
		    pub fn len_nodes(&self) -> usize {
		        self.nodes.len() - 1
		    }
		
		    pub fn len_edges(&self) -> usize {
		        self.edges.len() - self.erased.len() - 1
		    }
		
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
		        let mut p2 = self
		            .get_edges_enum_from(edge)
		            .skip(len / 2 - 1)
		            .next()
		            .unwrap()
		            .0;
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
		
		    pub fn sort_edges(&mut self, node: usize) {
		        self.sort_edges_by(node, |(a, _), (b, _)| a.cmp(b));
		    }
		
		    pub fn sort_edges_by<F>(&mut self, node: usize, mut compare: F)
		    where
		        F: FnMut(&(usize, &V), &(usize, &V)) -> Ordering,
		    {
		        let len = self.get_edges(node).count();
		        self.head[node] =
		            self.sort_edges_inner(self.head[node], len, &mut |a, b| compare(a, b) == Less);
		    }
		
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
		
		    fn get_edges_from(&self, mut edge: usize) -> impl Iterator<Item = (&usize, &E)> {
		        from_fn(move || {
		            if edge == 0 {
		                return None;
		            }
		            let (next, to, edge_info) = &self.edges[edge];
		            edge = *next;
		            Some((to, edge_info))
		        })
		    }
		
		    fn get_edges_enum_from(&self, mut edge: usize) -> impl Iterator<Item = (usize, (&usize, &E))> {
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
		
		    pub fn get_edges(&self, node: usize) -> impl Iterator<Item = (&usize, &E)> {
		        self.get_edges_from(self.head[node])
		    }
		
		    pub fn get_edges_enum(&self, node: usize) -> impl Iterator<Item = (usize, (&usize, &E))> {
		        self.get_edges_enum_from(self.head[node])
		    }
		
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
		
		    pub fn get_edge(&self, idx: usize) -> (usize, usize, &E) {
		        let (next, to, edge_info) = &self.edges[idx];
		        (*next, *to, edge_info)
		    }
		
		    pub fn get_twin_edge(&self, idx: usize) -> (usize, usize, &E) {
		        let (next, to, edge_info) = &self.edges[TWIN(idx)];
		        (*next, *to, edge_info)
		    }
		
		    pub fn get_edge_mut(&mut self, idx: usize) -> (usize, usize, &mut E) {
		        let (next, to, edge_info) = &mut self.edges[idx];
		        (*next, *to, edge_info)
		    }
		
		    pub fn get_twin_edge_mut(&mut self, idx: usize) -> (usize, usize, &mut E) {
		        let (next, to, edge_info) = &mut self.edges[TWIN(idx)];
		        (*next, *to, edge_info)
		    }
		}
		
		pub const TWIN: fn(usize) -> usize = |idx| ((idx - 1) ^ 1) + 1;
		pub mod degree {
			pub trait Degree {
			    fn in_dgr(&self) -> usize;
			    fn out_dgr(&self) -> usize;
			    fn dgr(&self) -> usize;
			}
			
			impl Degree for usize {
			    fn in_dgr(&self) -> usize {
			        *self
			    }
			
			    fn out_dgr(&self) -> usize {
			        *self
			    }
			
			    fn dgr(&self) -> usize {
			        *self
			    }
			}
			
			impl Degree for (usize, usize) {
			    fn in_dgr(&self) -> usize {
			        self.0
			    }
			
			    fn out_dgr(&self) -> usize {
			        self.1
			    }
			
			    fn dgr(&self) -> usize {
			        self.0 + self.1
			    }
			}
		}
		pub mod dijkstra {
			use crate::cplit::graph::{Distance, Graph};
			use crate::cplit::num::{Numeric, NumericAssOps, NumericCmpOps, NumericOps};
			use std::cmp::Reverse;
			use std::collections::BinaryHeap;
			
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
		}
		pub mod distance {
			use crate::cplit::num::Numeric;
			pub trait Distance<N>
			where
			    N: Numeric + Copy,
			{
			    fn dist(&self) -> N;
			}
			
			impl<N> Distance<N> for N
			where
			    N: Numeric + Copy,
			{
			    fn dist(&self) -> N {
			        *self
			    }
			}
		}
		pub mod hierholzer {
			use std::collections::LinkedList;
			use std::ptr::{addr_of, addr_of_mut};
			
			use crate::cplit::graph::{Graph, TWIN};
			use crate::cplit::utils::Flag;
			
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
		}
		
		#[doc(inline)]
		pub use self::dijkstra::dijkstra;
		#[doc(inline)]
		pub use self::hierholzer::{hierholzer_directed, hierholzer_undirected};
		
		#[doc(inline)]
		pub use self::degree::Degree;
		#[doc(inline)]
		pub use self::distance::Distance;
	}
	pub mod num {
		use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
		use std::ops::{Add, Div, Mul, Rem, Sub};
		use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
		pub mod bounds {
			pub trait LowerBounded {
			    const MIN: Self;
			}
			
			pub trait UpperBounded {
			    const MAX: Self;
			}
			
			pub trait Bounded: LowerBounded + UpperBounded {}
			
			impl<T> Bounded for T where T: LowerBounded + UpperBounded {}
			
			macro_rules! bounded_trait_impl {
			    ($t:ty, $min:expr, $max:expr) => {
			        impl LowerBounded for $t {
			            const MIN: Self = $min;
			        }
			
			        impl UpperBounded for $t {
			            const MAX: Self = $max;
			        }
			    };
			}
			
			bounded_trait_impl!(usize, usize::MIN, usize::MAX);
			bounded_trait_impl!(u8, u8::MIN, u8::MAX);
			bounded_trait_impl!(u16, u16::MIN, u16::MAX);
			bounded_trait_impl!(u32, u32::MIN, u32::MAX);
			bounded_trait_impl!(u64, u64::MIN, u64::MAX);
			bounded_trait_impl!(u128, u128::MIN, u128::MAX);
			
			bounded_trait_impl!(isize, isize::MIN, isize::MAX);
			bounded_trait_impl!(i8, i8::MIN, i8::MAX);
			bounded_trait_impl!(i16, i16::MIN, i16::MAX);
			bounded_trait_impl!(i32, i32::MIN, i32::MAX);
			bounded_trait_impl!(i64, i64::MIN, i64::MAX);
			bounded_trait_impl!(i128, i128::MIN, i128::MAX);
			
			bounded_trait_impl!(f32, f32::MIN, f32::MAX);
			bounded_trait_impl!(f64, f64::MIN, f64::MAX);
		}
		#[doc(inline)]
		pub use self::bounds::{Bounded, LowerBounded, UpperBounded};
		
		pub trait Numeric: Default + Zero + One {}
		
		impl<T> Numeric for T where T: Default + Zero + One {}
		
		pub trait NumericOps<Rhs = Self, Output = Self>:
		    Add<Rhs, Output = Output>
		    + Sub<Rhs, Output = Output>
		    + Mul<Rhs, Output = Output>
		    + Div<Rhs, Output = Output>
		{
		}
		
		impl<T, Rhs, Output> NumericOps<Rhs, Output> for T where
		    T: Add<Rhs, Output = Output>
		        + Sub<Rhs, Output = Output>
		        + Mul<Rhs, Output = Output>
		        + Div<Rhs, Output = Output>
		{
		}
		
		pub trait IntegerOps<Rhs = Self, Output = Self>:
		    NumericOps<Rhs, Output> + Rem<Rhs, Output = Output>
		{
		}
		
		impl<T, Rhs, Output> IntegerOps<Rhs, Output> for T where
		    T: NumericOps<Rhs, Output> + Rem<Rhs, Output = Output>
		{
		}
		
		pub trait NumericAssOps<Rhs = Self>:
		    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
		{
		}
		
		impl<T, Rhs> NumericAssOps<Rhs> for T where
		    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
		{
		}
		
		pub trait IntegerAssOps<Rhs = Self>: NumericAssOps<Rhs> + RemAssign<Rhs> {}
		impl<T, Rhs> IntegerAssOps<Rhs> for T where T: NumericAssOps<Rhs> + RemAssign<Rhs> {}
		
		pub trait NumericCmpOps<Rhs = Self>: PartialEq<Rhs> + PartialOrd<Rhs> + Eq + Ord {}
		
		impl<T, Rhs> NumericCmpOps<Rhs> for T where T: PartialEq<Rhs> + PartialOrd<Rhs> + Eq + Ord {}
		
		pub trait Zero {
		    const ZERO: Self;
		}
		
		pub trait One {
		    const ONE: Self;
		}
		
		macro_rules! zero_trait_impl {
		    ($name:ident for $($t:ty)*) => ($(
		        impl $name for $t {
		            const ZERO: Self = 0 as $t;
		        }
		    )*)
		}
		
		zero_trait_impl!(Zero for usize u8 u16 u32 u64 u128);
		zero_trait_impl!(Zero for isize i8 i16 i32 i64 i128);
		zero_trait_impl!(Zero for f32 f64);
		
		macro_rules! one_trait_impl {
		    ($name:ident for $($t:ty)*) => ($(
		        impl $name for $t {
		            const ONE: Self = 1 as $t;
		        }
		    )*)
		}
		
		one_trait_impl!(One for usize u8 u16 u32 u64 u128);
		one_trait_impl!(One for isize i8 i16 i32 i64 i128);
		one_trait_impl!(One for f32 f64);
		
		macro_rules! tuple_zero_impl {
		    ( $( $name:ident )+ ) => {
		        impl<$($name: Zero),+> Zero for ($($name,)+)
		        {
		            const ZERO: Self = ($($name::ZERO,)+);
		        }
		    };
		}
		
		tuple_zero_impl!(A B);
		
		macro_rules! tuple_one_impl {
		    ( $( $name:ident )+ ) => {
		        impl<$($name: One),+> One for ($($name,)+)
		        {
		            const ONE: Self = ($($name::ONE,)+);
		        }
		    };
		}
		
		tuple_one_impl!(A B);
	}
	pub mod number_theory {
		pub mod euler_sieve {
			pub fn euler_sieve<T, M>(n: usize) -> (Vec<usize>, Vec<T>)
			where
			    T: Default + Copy + Clone,
			    M: MulFunc<T>,
			{
			    let mut is_prime = vec![true; n + 1];
			    let mut primes = vec![];
			    is_prime[0] = false;
			    is_prime[1] = false;
			    let mut f = vec![T::default(); n + 1];
			    f[1] = M::ONE;
			    for i in 2..=n {
			        if is_prime[i] {
			            primes.push(i);
			            f[i] = M::P(i, primes.len());
			        }
			        for &p in &primes {
			            if i * p > n {
			                break;
			            }
			            is_prime[i * p] = false;
			            if i % p == 0 {
			                f[i * p] = M::DERIVE_DIVIDES(p, i, &(|idx: usize| f[idx]));
			                break;
			            }
			            f[i * p] = M::DERIVE_COPRIME(p, i, &(|idx: usize| f[idx]));
			        }
			    }
			    (primes, f)
			}
			
			pub trait MulFunc<T = usize> {
			    const ONE: T;
			    const P: fn(p: usize, index: usize) -> T;
			    const DERIVE_DIVIDES: fn(p: usize, x: usize, f: &dyn Fn(usize) -> T) -> T;
			    const DERIVE_COPRIME: fn(p: usize, x: usize, f: &dyn Fn(usize) -> T) -> T;
			}
			
			pub struct EulerPhi;
			
			impl MulFunc for EulerPhi {
			    const ONE: usize = 1;
			    const P: fn(usize, usize) -> usize = |p, _| p - 1;
			    const DERIVE_DIVIDES: fn(usize, usize, &dyn Fn(usize) -> usize) -> usize = |p, x, f| f(x) * p;
			    const DERIVE_COPRIME: fn(usize, usize, &dyn Fn(usize) -> usize) -> usize =
			        |p, x, f| f(x) * f(p);
			}
			
			impl MulFunc<()> for () {
			    const ONE: () = ();
			    const P: fn(usize, usize) -> () = |_, _| ();
			    const DERIVE_DIVIDES: fn(usize, usize, &dyn Fn(usize) -> ()) -> () = |_, _, _| ();
			    const DERIVE_COPRIME: fn(usize, usize, &dyn Fn(usize) -> ()) -> () = |_, _, _| ();
			}
			
			impl<T1, T2, F1, F2> MulFunc<(T1, T2)> for (F1, F2)
			where
			    F1: MulFunc<T1>,
			    F2: MulFunc<T2>,
			{
			    const ONE: (T1, T2) = (F1::ONE, F2::ONE);
			    const P: fn(usize, usize) -> (T1, T2) = |p, index| (F1::P(p, index), F2::P(p, index));
			    const DERIVE_DIVIDES: fn(usize, usize, &dyn Fn(usize) -> (T1, T2)) -> (T1, T2) = |p, x, f| {
			        (
			            F1::DERIVE_DIVIDES(p, x, &|idx| f(idx).0),
			            F2::DERIVE_DIVIDES(p, x, &|idx| f(idx).1),
			        )
			    };
			    const DERIVE_COPRIME: fn(usize, usize, &dyn Fn(usize) -> (T1, T2)) -> (T1, T2) = |p, x, f| {
			        (
			            F1::DERIVE_COPRIME(p, x, &|idx| f(idx).0),
			            F2::DERIVE_COPRIME(p, x, &|idx| f(idx).1),
			        )
			    };
			}
		}
		
		#[doc(inline)]
		pub use self::euler_sieve::euler_sieve;
	}
	pub mod utils {
		use crate::cplit::num::{One, Zero};
		use std::cmp::Ordering;
		use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
		
		pub trait Flag {
		    fn set(&mut self, val: bool);
		    fn get(&self) -> bool;
		}
		
		impl Flag for bool {
		    fn set(&mut self, val: bool) {
		        *self = val;
		    }
		
		    fn get(&self) -> bool {
		        *self
		    }
		}
		
		static mut EPSILON: F64 = F64(1e-7);
		
		pub fn set_epsilon(val: F64) {
		    unsafe {
		        EPSILON = val;
		    }
		}
		
		pub fn get_epsilon() -> F64 {
		    unsafe { EPSILON }
		}
		
		#[derive(Debug, Clone, Copy, Default)]
		pub struct F64(pub f64);
		
		macro_rules! ops_trait_impl {
		    ($($name:ident $fun:tt),* for $t:ty) => ($(
		        impl $name for $t {
		            type Output = Self;
		
		            fn $fun(self, rhs: Self) -> Self::Output {
		                Self(self.0.$fun(rhs.0))
		            }
		        }
		    )*)
		}
		
		ops_trait_impl!(Add add, Sub sub, Mul mul, Div div for F64);
		
		macro_rules! ass_ops_trait_impl {
		    ($($name:ident $fun:tt),* for $t:ty) => ($(
		        impl $name for $t {
		            fn $fun(&mut self, rhs: Self) {
		                self.0.$fun(rhs.0)
		            }
		        }
		    )*)
		}
		
		ass_ops_trait_impl!(AddAssign add_assign, SubAssign sub_assign, MulAssign mul_assign, DivAssign div_assign for F64);
		
		impl Neg for F64 {
		    type Output = Self;
		
		    fn neg(self) -> Self::Output {
		        Self(-self.0)
		    }
		}
		
		impl PartialEq for F64 {
		    fn eq(&self, other: &Self) -> bool {
		        (self.0 - other.0).abs() < get_epsilon().0
		    }
		}
		
		impl PartialOrd for F64 {
		    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		        match self.0.partial_cmp(&other.0) {
		            None => None,
		            Some(Ordering::Equal) => Some(Ordering::Equal),
		            Some(order) => {
		                if *self == *other {
		                    Some(Ordering::Equal)
		                } else {
		                    Some(order)
		                }
		            }
		        }
		    }
		}
		
		impl Zero for F64 {
		    const ZERO: Self = F64(0.0_f64);
		}
		
		impl One for F64 {
		    const ONE: Self = F64(1.0_f64);
		}
	}
	
	#[macro_use]
	mod macros {
		#[macro_export]
		macro_rules! fscanln {
		    ($reader:expr, $($i:expr), +) => {{
		        #[allow(unused_imports)]
		        use std::io::BufRead;
		        let mut iter = std::iter::repeat_with(|| {
		            let mut buf = String::new();
		            $reader.read_line(&mut buf).unwrap();
		            buf.split_whitespace()
		                .map(|x| x.to_string())
		                .collect::<Vec<_>>()
		        })
		        .flatten();
		        $(
		            $i = iter.next().unwrap().parse().unwrap();
		        )*
		    }};
		
		    ( $reader:expr, $($i:expr), +, ?) => {{
		        #[allow(unused_imports)]
		        use std::io::BufRead;
		        let mut iter = std::iter::repeat_with(|| {
		            let mut buf = String::new();
		            $reader.read_line(&mut buf).unwrap();
		            buf.split_whitespace()
		                .map(|x| x.to_string())
		                .collect::<Vec<_>>()
		        })
		        .find(|x| !x.is_empty())
		        .unwrap()
		        .into_iter();
		        $(
		            if let Some(val) = iter.next() {
		                $i = val.parse().unwrap();
		            } else {
		                $i = Default::default();
		            }
		        )*
		    }};
		
		    ($reader:expr, $coll:expr ; $n:expr) => {{
		        #[allow(unused_imports)]
		        use std::io::BufRead;
		        $coll = std::iter::once(Default::default())
		                .chain(
		                    std::iter::repeat_with(|| {
		                        let mut buf = String::new();
		                        $reader.read_line(&mut buf).unwrap();
		                        buf.split_whitespace()
		                            .map(|x| x.parse().unwrap())
		                            .collect::<Vec<_>>()
		                    })
		                    .flatten()
		                    .take($n),
		                )
		                .collect();
		    }};
		
		    ($reader:expr, $coll:expr ;) => {{
		        #[allow(unused_imports)]
		        use std::io::BufRead;
		        let mut buf = String::new();
		        $reader.read_line(&mut buf).unwrap();
		        $coll = std::iter::once(Default::default())
		                .chain(buf.split_whitespace().map(|x| x.parse().unwrap()))
		                .collect();
		    }};
		}
		
		#[macro_export]
		macro_rules! scanln {
		    ($($i:expr), +) => {
		        $crate::fscanln!(std::io::stdin(), $($i), +);
		    };
		
		    ($($i:expr), +, ?) => {
		        $crate::fscanln!(std::io::stdin(), $($i), +, ?);
		    };
		
		    ($coll:expr ; $n:expr) => {
		        $crate::fscanln!(std::io::stdin(), $coll ; $n);
		    };
		
		    ($coll:expr ;) => {
		        $crate::fscanln!(std::io::stdin(), $coll ;);
		    };
		}
	}
}
