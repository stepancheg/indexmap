#![feature(bench_black_box)]

use indexmap::IndexMap;
use std::hint::black_box;

fn main() {
    for _ in 0..10000 {
        let mut m = IndexMap::with_capacity(10000);
        for i in 0..10000 {
            m.insert_unique_unchecked(i, i);
            black_box(&mut m);
        }
    }
}