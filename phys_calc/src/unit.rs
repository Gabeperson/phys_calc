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

macro_rules! impl_none {
    ($ident:ident) => {
        impl $ident for None {
            fn mult() -> number {
                panic!("`None` is not a regular unit!")
            }

            fn unit() -> &'static str {
                panic!("`None` is not a regular unit!")
            }
        }
    };
}

impl Unit for None {}
impl_none!(LengthUnit);
impl_none!(TimeUnit);
impl_none!(CurrentUnit);
impl_none!(LuminousIntensityUnit);
impl_none!(MassUnit);
impl_none!(QuantityUnit);
impl_none!(AngleUnit);
impl_none!(DigitalInformationUnit);
impl_none!(SolidAngleUnit);

impl TempUnit for None {
    fn to_base(
        _s: crate::base::temperature::Temperature<Self>,
    ) -> crate::base::temperature::Temperature<crate::base::temperature::Kelvin> {
        panic!("`None` is not a regular unit!")
    }

    fn from_base(
        _s: crate::base::temperature::Temperature<crate::base::temperature::Kelvin>,
    ) -> crate::base::temperature::Temperature<Self> {
        panic!("`None` is not a regular unit!")
    }

    fn unit() -> &'static str {
        panic!("`None` is not a regular unit!")
    }

    fn mult() -> number {
        panic!("`None` is not a regular unit!")
    }
}
