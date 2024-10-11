use std::fmt::Display;
use std::fmt::LowerExp;
use std::marker::PhantomData;

use phys_calc_macros::Unit;

use super::unit_traits::*;
use crate::math::*;
use crate::math_helpers::*;
use crate::math_impl::impl_math;
use crate::unit::None;
use crate::Unit;

pub trait TimeUnit: Unit {
    fn to_base(s: Time<Self>) -> Time<Second> {
        Time {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: Time<Second>) -> Time<Self> {
        Time {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn mult() -> number;
    fn unit() -> &'static str;
}

impl_math!(Time<T: TimeUnit>);

#[derive(Copy, Clone, Debug)]
pub struct Time<T: TimeUnit> {
    pub inner: number,
    pub types: PhantomData<T>,
}

impl<T: TimeUnit> Time<T> {
    pub fn convert<D: TimeUnit>(self) -> Time<D> {
        D::from_base(T::to_base(self))
    }
}

impl<T> Display for Time<T>
where
    T: TimeUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, T::unit())
    }
}

impl<T> LowerExp for Time<T>
where
    T: TimeUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, T::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-12)]
#[unit("ps")]
#[unit_impl(TimeUnit)]
pub struct Picosecond;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-9)]
#[unit("ns")]
#[unit_impl(TimeUnit)]
pub struct Nanosecond;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-6)]
#[unit("μs")]
#[unit_impl(TimeUnit)]
pub struct Microsecond;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1e-3)]
#[unit("ms")]
#[unit_impl(TimeUnit)]
pub struct Millisecond;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1.)]
#[unit("s")]
#[unit_impl(TimeUnit)]
pub struct Second;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(60.)]
#[unit("min")]
#[unit_impl(TimeUnit)]
pub struct Minute;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(3600.)]
#[unit("hr")]
#[unit_impl(TimeUnit)]
pub struct Hour;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(86400.)]
#[unit("day")]
#[unit_impl(TimeUnit)]
pub struct Day;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(2629800.)] // 365.25 / 12 * 86400
#[unit("month")]
#[unit_impl(TimeUnit)]
pub struct Month;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(31557600.)]
#[unit("yr")]
#[unit_impl(TimeUnit)]
pub struct Year;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(4.348e17)]
#[unit("AgeOfUniverse")]
#[unit_impl(TimeUnit)]
pub struct AgeOfUniverse;

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
//     type LengthExp = Zero;
//     type TimeExp = One;

//     type LengthUnit = None;
//     type TimeUnit = T;
//     fn to_derived(
//         self,
//     ) -> Derived<
//         LengthPower<Self::LengthExp, Self::LengthUnit>,
//         TimePower<Self::TimeExp, Self::TimeUnit>,
//     > {
//         Derived {
//             inner: self.inner,
//             types: PhantomData,
//         }
//     }
// }
