#![feature(test)]
extern crate test;

use base_mod_internals::math::scalar::aliases::UsfOrNormalScalar;
use base_mod_internals::math::scalar::shared::ScalarCoreOps;
use base_mod_internals::math::scalar::usf::{UsfScalar, UsfScalarConstants};
use num_integer::Roots;
use test::{Bencher, black_box};

fn my_function() {
    let mut x: i32 = 0;
    for i in 0..1000 {
        x += i;
        x = x.sqrt();
        x = black_box(x);
    }
    black_box(x);
}

fn my_complex_function() {
    let pos_sample = <UsfScalar as UsfScalarConstants>::POSITIVE_ONE_DECILLION;
    let neg_sample = <UsfScalar as UsfScalarConstants>::NEGATIVE_ONE_DECILLION;
    let value = pos_sample.add(UsfOrNormalScalar::A(neg_sample.clone()));
    black_box((pos_sample, neg_sample, value));
}

fn my_simple_function() {
    // black_box(black_box(4223372036854775807_i64) + black_box(4223372036854775807_i64));
    black_box(black_box(4223372036854775807_i64).nth_root(3));
}

fn do_something() {}

#[bench]
fn math_scalar_bench(b: &mut Bencher) {
    b.iter(|| {
        black_box(my_complex_function());
    });
}
