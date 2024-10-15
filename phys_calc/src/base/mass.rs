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

pub trait MassUnit: Unit {
    fn to_base(s: Mass<Self>) -> Mass<Kilogram> {
        Mass {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: Mass<Kilogram>) -> Mass<Self> {
        Mass {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn mult() -> number;
    fn unit() -> &'static str;
}

impl_math!(Mass<M: MassUnit>);

#[derive(Copy, Clone, Debug)]
pub struct Mass<M: MassUnit> {
    pub inner: number,
    pub types: PhantomData<M>,
}

impl<M: MassUnit> Mass<M> {
    pub fn convert<D: MassUnit>(self) -> Mass<D> {
        D::from_base(M::to_base(self))
    }
}

impl<M> Display for Mass<M>
where
    M: MassUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, M::unit())
    }
}

impl<M> LowerExp for Mass<M>
where
    M: MassUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, M::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-15)]
#[unit("pg")]
#[unit_impl(MassUnit)]
pub struct Picogram;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-12)]
#[unit("ng")]
#[unit_impl(MassUnit)]
pub struct Nanogram;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-9)]
#[unit("μg")]
#[unit_impl(MassUnit)]
pub struct Microgram;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-6)]
#[unit("mg")]
#[unit_impl(MassUnit)]
pub struct Milligram;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-3)]
#[unit("g")]
#[unit_impl(MassUnit)]
pub struct Gram;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1.)]
#[unit("kg")]
#[unit_impl(MassUnit)]
pub struct Kilogram;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1000.)]
#[unit("t")]
#[unit_impl(MassUnit)]
pub struct Tonne;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.0283495)]
#[unit("oz")]
#[unit_impl(MassUnit)]
pub struct Ounce;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.453592)]
#[unit("lb")]
#[unit_impl(MassUnit)]
pub struct Pound;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(907.185)] // 365.25 / 12 * 86400
#[unit("shortton")]
#[unit_impl(MassUnit)]
pub struct ShortTon;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1016.05)] // 365.25 / 12 * 86400
#[unit("longton")]
#[unit_impl(MassUnit)]
pub struct LongTon;

unspecialize!(Mass<M: MassUnit>);
impl_derived_conversions!(Mass<M: MassUnit>, Mass: One, M);
