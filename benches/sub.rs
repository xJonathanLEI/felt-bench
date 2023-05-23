use std::ops::Sub;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
            "sub | starknet-ff - xJonathanLEI/starknet-rs@a6cbfa3",
            |b| {
                b.iter(|| {
                    black_box(num_1.sub(num_2));
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

        c.bench_function("sub | cairo-felt - lambdaclass/cairo-rs@e173ec9", |b| {
            b.iter(|| {
                // No choice but to clone here. See the `sub_assign` bench for the clone-less
                // version
                black_box(num_1.clone().sub(&num_2));
            });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
