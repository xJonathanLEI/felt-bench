use criterion::{black_box, criterion_group, criterion_main, Criterion};
use felt_bench::*;

const BENCHMARK_NAME: &str = "sqrt";

pub fn criterion_benchmark(c: &mut Criterion) {
    // starknet-ff
    {
        use starknet_ff::FieldElement;

        let num = FieldElement::from_hex_be(
            "0x03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        )
        .unwrap();
        let num = num * num;

        c.bench_function(
            &format!(
                "{} | {} - {}",
                BENCHMARK_NAME, IMPL_STARKNET_FF.name, IMPL_STARKNET_FF.source,
            ),
            |b| {
                b.iter(|| {
                    black_box(black_box(&num).sqrt());
                });
            },
        );
    }

    // cairo-felt
    {
        use cairo_felt::Felt252;

        let num = Felt252::from_bytes_be(
            &hex::decode("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb")
                .unwrap(),
        );
        let num = &num * &num;

        c.bench_function(
            &format!(
                "{} | {} - {}",
                BENCHMARK_NAME, IMPL_CAIRO_FELT.name, IMPL_CAIRO_FELT.source,
            ),
            |b| {
                b.iter(|| {
                    black_box(black_box(&num).sqrt());
                });
            },
        );
    }

    // stark_curve
    {
        use stark_curve::ff::Field;
        use stark_curve::FieldElement;
        use stark_hash::Felt;

        let num = FieldElement::from(
            Felt::from_be_slice(
                &hex::decode("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb")
                    .unwrap(),
            )
            .unwrap(),
        );
        #[allow(clippy::op_ref)]
        let num = num * &num;

        c.bench_function(
            &format!(
                "{} | {} - {}",
                BENCHMARK_NAME, IMPL_STARK_CURVE.name, IMPL_STARK_CURVE.source,
            ),
            |b| {
                b.iter(|| {
                    black_box(black_box(&num).sqrt());
                });
            },
        );
    }

    // lambdaworks-math
    {
        use lambdaworks_math::field::{
            element::FieldElement, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField,
        };

        let num = FieldElement::<Stark252PrimeField>::from_hex(
            "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        )
        .unwrap();
        let num = &num * &num;

        c.bench_function(
            &format!(
                "{} | {} - {}",
                BENCHMARK_NAME, IMPL_LAMBDAWORKS_MATH.name, IMPL_LAMBDAWORKS_MATH.source,
            ),
            |b| {
                b.iter(|| {
                    black_box(black_box(&num).sqrt());
                });
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
