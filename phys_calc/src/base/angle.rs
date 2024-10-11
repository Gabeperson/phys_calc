use std::fmt::Display;
use std::fmt::LowerExp;
use std::marker::PhantomData;

use phys_calc_macros::Unit;

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

use super::unit_traits::*;
use crate::math::*;
use crate::math_helpers::*;
use crate::math_impl::impl_math;
use crate::unit::None;
use crate::Unit;

pub trait AngleUnit: Unit {
    fn to_base(s: Angle<Self>) -> Angle<Radian> {
        Angle {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: Angle<Radian>) -> Angle<Self> {
        Angle {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn mult() -> number;
    fn unit() -> &'static str;
}

impl_math!(Angle<A: AngleUnit>);

#[derive(Copy, Clone, Debug)]
pub struct Angle<A: AngleUnit> {
    pub inner: number,
    pub types: PhantomData<A>,
}

impl<A: AngleUnit> Angle<A> {
    pub fn convert<D: AngleUnit>(self) -> Angle<D> {
        D::from_base(A::to_base(self))
    }
}

impl<A> Display for Angle<A>
where
    A: AngleUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, A::unit())
    }
}

impl<A> LowerExp for Angle<A>
where
    A: AngleUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, A::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(number::pi() / 180. / 60. / 60.)]
#[unit("″")]
#[unit_impl(AngleUnit)]
pub struct Arcsecond;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(number::pi() / 180. / 60.)]
#[unit("′")]
#[unit_impl(AngleUnit)]
pub struct Arcminute;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(number::pi() / 180.)]
#[unit("°")]
#[unit_impl(AngleUnit)]
pub struct Degree;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-3)]
#[unit("mrad")]
#[unit_impl(AngleUnit)]
pub struct MilliRadian;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1)]
#[unit("rad")]
#[unit_impl(AngleUnit)]
pub struct Radian;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(number::pi() / 200.)]
#[unit("grad")]
#[unit_impl(AngleUnit)]
pub struct Gradian;

// impl<T: TimeUnit, L: LengthUnit> DerivedToUnit
//     for Derived<LengthPower<Zero, L>, TimePower<One, T>>
// {
//     type Output = Time<T>;
//     type LU = L;
//     type TU = T;

//     fn to_unit(self) -> Self::Output {
//         Time {
//             inner: self.inner,
//             types: PhantomData,
//         }
//     }
// }

// impl<T: TimeUnit> UnitToDerived for Time<T> {
//     type LP = Zero;
//     type TP = One;

//     type LU = None;
//     type TU = T;
//     fn to_derived(self) -> Derived<LengthPower<Self::LP, Self::LU>, TimePower<Self::TP, Self::TU>> {
//         Derived {
//             inner: self.inner,
//             types: PhantomData,
//         }
//     }
// }
