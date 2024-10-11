use crate::base::unit_traits::*;
use crate::math::{number, EqualsOrZero};

pub trait Unit: Copy + Clone + Sized {
    // fn mult() -> number;
    // fn unit() -> &'static str;
}

#[derive(Copy, Clone, Debug)]
pub struct None;
impl<T> EqualsOrZero<T> for None {
    type SelfType = T;
}

impl Unit for None {}
impl LengthUnit for None {
    fn unit() -> &'static str {
        panic!("`None` is not a regular unit!")
    }

    fn mult() -> number {
        panic!("`None` is not a regular unit!")
    }
}
impl TimeUnit for None {
    fn mult() -> number {
        panic!("`None` is not a regular unit!")
    }

    fn unit() -> &'static str {
        panic!("`None` is not a regular unit!")
    }
}
