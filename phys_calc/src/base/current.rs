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

pub trait CurrentUnit: Unit {
    fn to_base(s: Current<Self>) -> Current<Ampere> {
        Current {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: Current<Ampere>) -> Current<Self> {
        Current {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn unit() -> &'static str;
    fn mult() -> number;
}
impl_math!(Current<C: CurrentUnit>);

#[derive(Copy, Clone, Debug)]
pub struct Current<C: CurrentUnit> {
    pub inner: number,
    pub types: PhantomData<C>,
}

impl<C: CurrentUnit> Current<C> {
    pub fn convert<D: CurrentUnit>(self) -> Current<D> {
        D::from_base(C::to_base(self))
    }
}

impl<C> Display for Current<C>
where
    C: CurrentUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, C::unit())
    }
}

impl<C> LowerExp for Current<C>
where
    C: CurrentUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, C::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-9)]
#[unit("nA")]
#[unit_impl(CurrentUnit)]
pub struct Nanoampere;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-6)]
#[unit("μA")]
#[unit_impl(CurrentUnit)]
pub struct Microampere;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-3)]
#[unit("mA")]
#[unit_impl(CurrentUnit)]
pub struct Milliampere;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1.)]
#[unit("A")]
#[unit_impl(CurrentUnit)]
pub struct Ampere;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e3)]
#[unit("kA")]
#[unit_impl(CurrentUnit)]
pub struct Kiloampere;

unspecialize!(Current<C: CurrentUnit>);
impl_derived_conversions!(Current<C: CurrentUnit>, Current: One, C);
