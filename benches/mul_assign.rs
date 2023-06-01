use std::ops::MulAssign;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // starknet-ff
    {
        use starknet_ff::FieldElement;

        let mut num_1 = FieldElement::from_hex_be(
            "0x03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        )
        .unwrap();
        let num_2 = FieldElement::from_hex_be(
            "0x0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        )
        .unwrap();

        c.bench_function(
            "mul_assign | starknet-ff - xJonathanLEI/starknet-rs@a6cbfa3",
            |b| {
                b.iter(|| {
                    #[allow(clippy::unit_arg)]
                    black_box(num_1.mul_assign(black_box(num_2)));
                });
            },
        );
    }

    // cairo-felt
    {
        use cairo_felt::Felt252;

        let mut num_1 = Felt252::from_bytes_be(
            &hex::decode("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb")
                .unwrap(),
        );
        let num_2 = Felt252::from_bytes_be(
            &hex::decode("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a")
                .unwrap(),
        );

        c.bench_function(
            "mul_assign | cairo-felt - lambdaclass/cairo-rs@5db2e65",
            |b| {
                b.iter(|| {
                    #[allow(clippy::unit_arg)]
                    black_box(num_1.mul_assign(black_box(&num_2)));
                });
            },
        );
    }

    // stark_curve
    {
        use stark_curve::FieldElement;
        use stark_hash::Felt;

        let mut num_1 = FieldElement::from(
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
            "mul_assign | stark_curve - eqlabs/pathfinder@5b131c5",
            |b| {
                b.iter(|| {
                    #[allow(clippy::unit_arg)]
                    black_box(num_1.mul_assign(black_box(&num_2)));
                });
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
