#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pedas::sync::list::List;
use std::vec::Vec;

fn std_vec_push(c: &mut Criterion) {
    let limit = 10_000;

    c.bench_function("std vec push", move |b| {
        b.iter(|| {
            let mut vec = Vec::new();

            for i in 0..limit {
                vec.push(i);
            }

            vec
        })
    });
}

fn pedas_list_add(c: &mut Criterion) {
    let limit = 10_000;

    c.bench_function("pedas list add", move |b| {
        b.iter(|| {
            let mut list: List<usize> = List::empty();

            for i in 0..limit {
                list = list.add(i);
            }

            list
        })
    });
}

criterion_group!(
    benches,
    pedas_list_add,
    std_vec_push,
);
criterion_main!(benches);
