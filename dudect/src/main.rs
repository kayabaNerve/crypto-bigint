//! Fuzz tests for `crypto-bigint`.
//!
//! These use the `dudect_bencher` crate to check certain operations for
//! constant-time behavior.

use crypto_bigint::{NonZero, Random, U128, U256};
use dudect_bencher::{ctbench_main, BenchRng, Class, CtRunner};

use crypto_bigint::{Random, U128, U256, NonZero};
use dudect_bencher::{ctbench_main, BenchRng, Class, CtRunner};

/// Check `UInt::add` for constant-time operation.
fn add(runner: &mut CtRunner, mut rng: &mut BenchRng) {
    const ITERATIONS_OUTER: usize = 10_000;
    const ITERATIONS_INNER: usize = 10_000;

    let mut inputs = vec![];
    for _ in 0..ITERATIONS_OUTER {
        inputs.push((Class::Left, (U256::random(&mut rng), U256::random(&mut rng))));
    }
    for _ in 0..ITERATIONS_OUTER {
        inputs.push((Class::Right, (U256::ZERO, U256::ZERO)));
    }

    for (class, (a, b)) in inputs {
        runner.run_one(class, || {
            for _ in 0..ITERATIONS_INNER {
                core::hint::black_box(a.wrapping_add(&b));
            }
        })
    }
}

/// Check `UInt::rem` for constant-time operation.
fn rem(runner: &mut CtRunner, mut rng: &mut BenchRng) {
    const ITERATIONS_OUTER: usize = 10_000;
    const ITERATIONS_INNER: usize = 10_000;

    // Random modulus (256-bit)
    let modulus = NonZero::new(U256::random(&mut rng)).unwrap_or(NonZero::<U256>::ONE);

    // Precomputing the inputs appears to eliminate some noise
    let mut inputs = vec![];

    // 128-bit random example
    for _ in 0..ITERATIONS_OUTER {
        inputs.push((Class::Left, U128::ZERO.concat(&U128::random(&mut rng))));
    }

    // 256-bit random example
    for _ in 0..ITERATIONS_OUTER {
        inputs.push((Class::Right, U256::random(&mut rng)));
    }

    for (class, input) in inputs {
        runner.run_one(class, || {
            for _ in 0..ITERATIONS_INNER {
                core::hint::black_box(input.rem(&modulus));
            }
        })
    }
}

ctbench_main!(add, rem);
