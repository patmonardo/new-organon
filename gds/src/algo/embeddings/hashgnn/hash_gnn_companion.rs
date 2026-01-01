use crate::collections::BitSet;
use crate::core::utils::paged::HugeAtomicBitSet;
use rand::Rng;

use super::hash_gnn::MinAndArgmin;

/// Helpers mirroring Java's `HashGNNCompanion`.
pub struct HashGNNCompanion;

impl HashGNNCompanion {
    pub fn hash_argmin_atomic(
        bitset: &HugeAtomicBitSet,
        hashes: &[i32],
        result: &mut MinAndArgmin,
        temp: &mut MinAndArgmin,
    ) {
        temp.arg_min = -1;
        temp.min = i32::MAX;

        bitset.for_each_set_bit(|bit| {
            let hash = hashes[bit];
            if hash < temp.min {
                temp.min = hash;
                temp.arg_min = bit as i32;
            }
        });

        result.min = temp.min;
        result.arg_min = temp.arg_min;
    }

    pub fn hash_argmin(bitset: &BitSet, hashes: &[i32], result: &mut MinAndArgmin) {
        let mut arg_min = -1;
        let mut min_hash = i32::MAX;
        let mut bit = bitset.next_set_bit(0);
        while let Some(b) = bit {
            let hash = hashes[b];
            if hash < min_hash {
                min_hash = hash;
                arg_min = b as i32;
            }
            bit = bitset.next_set_bit(b + 1);
        }

        result.min = min_hash;
        result.arg_min = arg_min;
    }
}

/// Next prime number >= n (simple, deterministic).
///
/// This is a pragmatic replacement for Apache Commons `Primes.nextPrime`.
pub fn next_prime(mut n: i32) -> i32 {
    if n <= 2 {
        return 2;
    }
    if n % 2 == 0 {
        n += 1;
    }
    while !is_prime(n) {
        n += 2;
    }
    n
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5i32;
    while (i as i64) * (i as i64) <= n as i64 {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Java: `record HashTriple(int a,int b,int c)`
#[derive(Debug, Clone, Copy)]
pub struct HashTriple {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

impl HashTriple {
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        let c = next_prime(rng.gen_range(1..i32::MAX));
        Self::generate_with_c(rng, c)
    }

    pub fn generate_with_c<R: Rng>(rng: &mut R, c: i32) -> Self {
        let a = rng.gen_range(1..c);
        let b = rng.gen_range(1..c);
        Self { a, b, c }
    }

    pub fn compute_hashes_from_triple(embedding_dimension: usize, triple: HashTriple) -> Vec<i32> {
        let mut out = vec![0i32; embedding_dimension];
        for (i, out_val) in out.iter_mut().enumerate().take(embedding_dimension) {
            *out_val =
                (((i as i64) * (triple.a as i64) + (triple.b as i64)) % (triple.c as i64)) as i32;
        }
        out
    }
}
