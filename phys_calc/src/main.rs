use std::marker::PhantomData;

use phys_calc::base::length::{Kilometer, Length, Meter};

fn main() {
    function(5., 10.);
    function(10., 5.);
    function(10000., 5.);
}

#[inline(never)]
fn function(x: f32, y: f32) -> f32 {
    std::hint::black_box(x) + std::hint::black_box(y)
}

#[inline(never)]
fn function2(x: f32, y: f32) -> f32 {
    std::hint::black_box(x) + std::hint::black_box(y)
}
