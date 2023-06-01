use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // cairo-felt
    {
        use cairo_felt::Felt252;
        use num_traits::pow::Pow;

        let num_1 = Felt252::from_bytes_be(
            &hex::decode("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb")
                .unwrap(),
        );
        let num_2 = Felt252::from(u32::MAX);

        c.bench_function("pow | cairo-felt - lambdaclass/cairo-rs@5db2e65", |b| {
            b.iter(|| {
                black_box(black_box(&num_1).pow(black_box(&num_2)));
            });
        });
    }

    // stark_curve
    {
        use stark_curve::ff::Field;
        use stark_curve::FieldElement;
        use stark_hash::Felt;

        let num_1 = FieldElement::from(
            Felt::from_be_slice(
                &hex::decode("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb")
                    .unwrap(),
            )
            .unwrap(),
        );
        let num_2 = [u32::MAX as u64];

        c.bench_function("pow | stark_curve - eqlabs/pathfinder@5b131c5", |b| {
            b.iter(|| {
                black_box(black_box(&num_1).pow_vartime(black_box(&num_2)));
            });
        });
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
        let num_2 = u32::MAX;

        c.bench_function(
            "pow | lambdaworks-math - lambdaclass/lambdaworks@46dd588",
            |b| {
                b.iter(|| {
                    black_box(black_box(&num_1).pow(black_box(num_2)));
                });
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
