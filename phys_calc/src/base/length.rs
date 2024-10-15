use std::fmt::Display;
use std::fmt::LowerExp;
use std::marker::PhantomData;

use phys_calc_macros::impl_derived_conversions;
use phys_calc_macros::Unit;

use super::unit_traits::*;
use crate::math::*;
use crate::math_helpers::*;
use crate::math_impl::impl_math;
use crate::unit::None;
use crate::unspecialize;
use crate::Unit;

pub trait LengthUnit: Unit {
    fn to_base(s: Length<Self>) -> Length<Meter> {
        Length {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: Length<Meter>) -> Length<Self> {
        Length {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn unit() -> &'static str;
    fn mult() -> number;
}
impl_math!(Length<L: LengthUnit>);

#[derive(Copy, Clone, Debug)]
pub struct Length<L: LengthUnit> {
    pub inner: number,
    pub types: PhantomData<L>,
}

impl<L: LengthUnit> Length<L> {
    pub fn convert<D: LengthUnit>(self) -> Length<D> {
        D::from_base(L::to_base(self))
    }
}

impl<L> Display for Length<L>
where
    L: LengthUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, L::unit())
    }
}

impl<L> LowerExp for Length<L>
where
    L: LengthUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, L::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-12)]
#[unit("pm")]
#[unit_impl(LengthUnit)]
pub struct Picometer;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-9)]
#[unit("nm")]
#[unit_impl(LengthUnit)]
pub struct Nanometer;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-6)]
#[unit("μm")]
#[unit_impl(LengthUnit)]
pub struct Micrometer;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-3)]
#[unit("mm")]
#[unit_impl(LengthUnit)]
pub struct Millimeter;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.01)]
#[unit("cm")]
#[unit_impl(LengthUnit)]
pub struct Centimeter;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.1)]
#[unit("dm")]
#[unit_impl(LengthUnit)]
pub struct Decimeter;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1.)]
#[unit("m")]
#[unit_impl(LengthUnit)]
pub struct Meter;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1000.)]
#[unit("km")]
#[unit_impl(LengthUnit)]
pub struct Kilometer;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1.495978707e11)]
#[unit("au")]
#[unit_impl(LengthUnit)]
pub struct AstronomicalUnit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(9.4607304725808e15)]
#[unit("ly")]
#[unit_impl(LengthUnit)]
pub struct Lightyear;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(3.085677581e16)]
#[unit("pc")]
#[unit_impl(LengthUnit)]
pub struct Parsec;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.0254)]
#[unit("in")]
#[unit_impl(LengthUnit)]
pub struct Inch;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.3048)]
#[unit("ft")]
#[unit_impl(LengthUnit)]
pub struct Foot;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.9144)]
#[unit("yd")]
#[unit_impl(LengthUnit)]
pub struct Yard;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1609.34)]
#[unit("mi")]
#[unit_impl(LengthUnit)]
pub struct Mile;

unspecialize!(Length<L: LengthUnit>);
impl_derived_conversions!(Length<L: LengthUnit>, Length: One, L);
