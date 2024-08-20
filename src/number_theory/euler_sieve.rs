/// Given a positive integer `n`, calculate the prime numbers less than or equal to `n`
/// in `O(n)` time complexity.
///
/// For more information, see [Linear Sieve](https://cp-algorithms.com/algebra/prime-sieve-linear.html)
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

/// The derivation that defines a multiplicative function.
/// For a multiplicative function `f`, it satisfies:
/// - `f(1) = f.ONE`
/// - `f(x\cdot y) = f(x) \cdot f(y)`.
///
/// Note that this trait is designed for the `euler_sieve` function.
/// Some may not require the above properties.
pub trait MulFunc<T = usize> {
    /// The value of the multiplicative function at _1_.
    const ONE: T;
    /// Given the `index` th prime number `p`,
    /// calculate the value of the multiplicative function at `p`.
    const P: fn(p: usize, index: usize) -> T;
    /// Given a prime number `p`, a positive integer `x`,
    /// and a function `f` of multiplicative function values,
    /// such that `p` divides `x` i.e. `p|x`,
    /// calculate the value of the multiplicative function at `p \cdot x`.
    const DERIVE_DIVIDES: fn(p: usize, x: usize, f: &dyn Fn(usize) -> T) -> T;
    /// Given a prime number `p`, a positive integer `x`,
    /// and a function `f` of multiplicative function values,
    /// such that `p` is a prime and `p` does not divide `x` i.e. `p\not| x`,
    /// calculate the value of the multiplicative function at `p \cdot x`.
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
    const P: fn(usize, usize) = |_, _| ();
    const DERIVE_DIVIDES: fn(usize, usize, &dyn Fn(usize)) = |_, _, _| ();
    const DERIVE_COPRIME: fn(usize, usize, &dyn Fn(usize)) = |_, _, _| ();
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
