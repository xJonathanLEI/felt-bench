use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // starknet-ff
    {
        use starknet_ff::FieldElement;

        let num = FieldElement::from_hex_be(
            "0x03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        )
        .unwrap();

        c.bench_function(
            "invert | starknet-ff - xJonathanLEI/starknet-rs@a6cbfa3",
            |b| {
                b.iter(|| {
                    black_box(num.invert().unwrap());
                });
            },
        );
    }

    // cairo-felt - no invert method available

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

        c.bench_function("invert | stark_curve - eqlabs/pathfinder@fccef91", |b| {
            b.iter(|| {
                black_box(num.invert().unwrap());
            });
        });
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

        c.bench_function(
            "invert | lambdaworks-math - lambdaclass/lambdaworks@75423a1",
            |b| {
                b.iter(|| {
                    black_box(num.inv());
                });
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
