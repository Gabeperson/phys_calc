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

trait Pi {
    fn pi() -> Self;
}

impl Pi for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }
}

impl Pi for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }
}

pub trait SolidAngleUnit: Unit {
    fn to_base(s: SolidAngle<Self>) -> SolidAngle<Steradian> {
        SolidAngle {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: SolidAngle<Steradian>) -> SolidAngle<Self> {
        SolidAngle {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn unit() -> &'static str;
    fn mult() -> number;
}

impl_math!(SolidAngle<S: SolidAngleUnit>);

#[derive(Copy, Clone, Debug)]
pub struct SolidAngle<S: SolidAngleUnit> {
    pub inner: number,
    pub types: PhantomData<S>,
}

impl<S: SolidAngleUnit> SolidAngle<S> {
    pub fn convert<D: SolidAngleUnit>(self) -> SolidAngle<D> {
        D::from_base(S::to_base(self))
    }
}

impl<S> Display for SolidAngle<S>
where
    S: SolidAngleUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, S::unit())
    }
}

impl<S> LowerExp for SolidAngle<S>
where
    S: SolidAngleUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, S::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier((number::pi() / (60.*60.*180.)).powi(2))]
#[unit("arcsec²")]
#[unit_impl(SolidAngleUnit)]
pub struct SquareSecond;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier((number::pi() / (60.*180.)).powi(2))]
#[unit("arcmin²")]
#[unit_impl(SolidAngleUnit)]
pub struct SquareMinute;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier((number::pi() / 180.).powi(2))]
#[unit("deg²")]
#[unit_impl(SolidAngleUnit)]
pub struct SquareDegree;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1)]
#[unit("sr")]
#[unit_impl(SolidAngleUnit)]
pub struct Steradian;

unspecialize!(SolidAngle<S: SolidAngleUnit>);
impl_derived_conversions!(SolidAngle<S: SolidAngleUnit>, SolidAngle: One, S);
