#[derive(Debug)]
pub struct SegmentTree {
    sum: Vec<i64>,
    tag: Vec<i64>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        Self {
            sum: vec![0; n<<2],
            tag: vec![0; n<<2],
        }
    }

    fn pushup(&mut self, x: usize) {
        let SegmentTree { sum, .. } = self;
        sum[x] = sum[x<<1] + sum[x<<1|1];
    }

    fn pushdown(&mut self, x: usize, l: usize, r: usize) {
        let SegmentTree { sum, tag } = self;
        let m = (l+r)>>1;
        sum[x<<1] += tag[x]*(m-l+1) as i64;
        tag[x<<1] += tag[x];
        sum[x<<1|1] += tag[x]*(r-m) as i64;
        tag[x<<1|1] += tag[x];
        tag[x] = 0;
    }

    pub fn build(&mut self, x: usize, l: usize, r: usize, a: &Vec<i64>) {
        let SegmentTree { sum, .. } = self;
        if l == r { sum[x] = a[l-1] }
        else {
            let m = (l+r)>>1;
            self.build(x<<1, l, m, a);
            self.build(x<<1|1, m+1, r, a);
            self.pushup(x);
        }
    }

    pub fn modify(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize, del: i64) {
        if ql <= l && r <= qr {
            let SegmentTree { sum, tag } = self;
            sum[x] += del*(r-l+1) as i64;
            tag[x] += del;
        } else {
            self.pushdown(x, l, r);
            let m = (l+r)>>1;
            if ql <= m { self.modify(x<<1, l, m, ql, qr, del) }
            if m < qr { self.modify(x<<1|1, m+1, r, ql, qr, del) }
            // let SegmentTree { sum, tag } = self;
            // sum[x] = tag[x]*(r-l+1) as i64 + sum[x<<1] + sum[x<<1|1];
            self.pushup(x);
        }
    }

    pub fn query(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql <= l && r <= qr { self.sum[x] }
        else {
            self.pushdown(x, l, r);
            let m = (l+r)>>1;
            (if ql <= m { self.query(x<<1, l, m, ql, qr) } else { 0 })
            + (if m < qr { self.query(x<<1|1, m+1, r, ql, qr) } else { 0 })
            // + tag[x] * (cmp::min(qr,r)-cmp::max(ql,l)+1) as i64
        }
    }
}