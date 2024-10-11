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

pub trait QuantityUnit: Unit {
    fn to_base(s: Quantity<Self>) -> Quantity<Single> {
        Quantity {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: Quantity<Single>) -> Quantity<Self> {
        Quantity {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn mult() -> number;
    fn unit() -> &'static str;
}

impl_math!(Quantity<Q: QuantityUnit>);

#[derive(Copy, Clone, Debug)]
pub struct Quantity<T: QuantityUnit> {
    pub inner: number,
    pub types: PhantomData<T>,
}

impl<T: QuantityUnit> Quantity<T> {
    pub fn convert<D: QuantityUnit>(self) -> Quantity<D> {
        D::from_base(T::to_base(self))
    }
}

impl<T> Display for Quantity<T>
where
    T: QuantityUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, T::unit())
    }
}

impl<T> LowerExp for Quantity<T>
where
    T: QuantityUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, T::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1.)]
#[unit("")]
#[unit_impl(QuantityUnit)]
pub struct Single;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(6.022e23)]
#[unit("mol")]
#[unit_impl(QuantityUnit)]
pub struct Mole;

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
