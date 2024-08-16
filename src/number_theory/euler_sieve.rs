pub fn euler_sieve<T, F>(n: usize) -> (Vec<usize>, Vec<T>)
where
    T: Default + Clone,
    F: MulFunc<T>,
{
    let mut is_prime = vec![true; n + 1];
    let mut primes = vec![];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut f = vec![T::default(); n + 1];
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            f[i] = F::P(i);
        }
        for &p in &primes {
            if i * p > n {
                break;
            }
            is_prime[i * p] = false;
            if i % p == 0 {
                f[i * p] = F::DERIVE_DIVIDES(p, i, &f);
                break;
            }
            f[i * p] = F::DERIVE_COPRIME(p, i, &f);
        }
    }
    (primes, f)
}

/// The derivation that defines a multiplicative function.
/// For a multiplicative function `f`, it satisfies:
/// - `f(1) = f.ONE`
/// - `f(x\cdot y) = f(x) \cdot f(y)`.
pub trait MulFunc<T = usize> {
    /// The value of the multiplicative function at _1_.
    const ONE: T;
    /// Given a prime number `p`, calculate the value of the multiplicative function at `p`.
    const P: fn(p: usize) -> T;
    /// Given a prime number `p`, a positive integer `x`,
    /// and a vector `f` of multiplicative function values,
    /// such that `p` divides `x` i.e. `p|x`,
    /// calculate the value of the multiplicative function at `p \cdot x`.
    const DERIVE_DIVIDES: fn(p: usize, x: usize, f: &Vec<T>) -> T;
    /// Given a prime number `p`, a positive integer `x`,
    /// and a vector `f` of multiplicative function values,
    /// such that `p` is a prime and `p` does not divide `x` i.e. `p\not| x`,
    /// calculate the value of the multiplicative function at `p \cdot x`.
    const DERIVE_COPRIME: fn(p: usize, x: usize, f: &Vec<T>) -> T;
}

pub struct EulerPhi;

impl MulFunc for EulerPhi {
    const ONE: usize = 1;
    const P: fn(usize) -> usize = |p| p - 1;
    const DERIVE_DIVIDES: fn(usize, usize, &Vec<usize>) -> usize = |p, x, f| f[x] * p;
    const DERIVE_COPRIME: fn(usize, usize, &Vec<usize>) -> usize = |p, x, f| f[x] * f[p];
}

impl MulFunc<()> for () {
    const ONE: () = ();
    const P: fn(usize) -> () = |_| ();
    const DERIVE_DIVIDES: fn(usize, usize, &Vec<()>) -> () = |_, _, _| ();
    const DERIVE_COPRIME: fn(usize, usize, &Vec<()>) -> () = |_, _, _| ();
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::fscanln;
    use crate::number_theory::euler_sieve;

    #[test]
    fn luogu_p3383() {
        let mut reader = BufReader::new(Cursor::new(
            r#"
100 5
1
2
3
4
5
"#,
        ));
        let (n, q): (usize, usize);
        fscanln!(reader, n, q);
        let (p, _) = euler_sieve::<(), ()>(n);
        let mut ans = vec![];
        for _ in 0..q {
            let x: usize;
            fscanln!(reader, x);
            ans.push(p[x - 1]);
        }
        assert_eq!(ans, vec![2, 3, 5, 7, 11]);
    }
}
