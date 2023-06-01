use std::ops::Mul;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use felt_bench::*;

const BENCHMARK_NAME: &str = "mul";

pub fn criterion_benchmark(c: &mut Criterion) {
    // starknet-ff
    {
        use starknet_ff::FieldElement;

        let num_1 = FieldElement::from_hex_be(
            "0x03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        )
        .unwrap();
        let num_2 = FieldElement::from_hex_be(
            "0x0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        )
        .unwrap();

        c.bench_function(
            &format!(
                "{} | {} - {}",
                BENCHMARK_NAME, IMPL_STARKNET_FF.name, IMPL_STARKNET_FF.source,
            ),
            |b| {
                b.iter(|| {
                    black_box(black_box(&num_1).mul(black_box(num_2)));
                });
            },
        );
    }

    // cairo-felt
    {
        use cairo_felt::Felt252;

        let num_1 = Felt252::from_bytes_be(
            &hex::decode("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb")
                .unwrap(),
        );
        let num_2 = Felt252::from_bytes_be(
            &hex::decode("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a")
                .unwrap(),
        );

        c.bench_function(
            &format!(
                "{} | {} - {}",
                BENCHMARK_NAME, IMPL_CAIRO_FELT.name, IMPL_CAIRO_FELT.source,
            ),
            |b| {
                b.iter(|| {
                    // No choice but to clone here. See the `sub_assign` bench for the clone-less
                    // version
                    black_box(black_box(&num_1).mul(black_box(&num_2)));
                });
            },
        );
    }

    // stark_curve
    {
        use stark_curve::FieldElement;
        use stark_hash::Felt;

        let num_1 = FieldElement::from(
            Felt::from_be_slice(
                &hex::decode("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb")
                    .unwrap(),
            )
            .unwrap(),
        );
        let num_2 = FieldElement::from(
            Felt::from_be_slice(
                &hex::decode("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a")
                    .unwrap(),
            )
            .unwrap(),
        );

        c.bench_function(
            &format!(
                "{} | {} - {}",
                BENCHMARK_NAME, IMPL_STARK_CURVE.name, IMPL_STARK_CURVE.source,
            ),
            |b| {
                b.iter(|| {
                    black_box(black_box(num_1).mul(black_box(num_2)));
                });
            },
        );
    }

    // lambdaworks-math
    {
        use lambdaworks_math::field::{
            element::FieldElement, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField,
        };

        let num_1 = FieldElement::<Stark252PrimeField>::from_hex(
            "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        )
        .unwrap();
        let num_2 = FieldElement::<Stark252PrimeField>::from_hex(
            "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        )
        .unwrap();

        c.bench_function(
            &format!(
                "{} | {} - {}",
                BENCHMARK_NAME, IMPL_LAMBDAWORKS_MATH.name, IMPL_LAMBDAWORKS_MATH.source,
            ),
            |b| {
                b.iter(|| {
                    black_box(black_box(&num_1).mul(black_box(&num_2)));
                });
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
