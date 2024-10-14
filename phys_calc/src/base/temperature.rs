use std::fmt::Display;
use std::fmt::LowerExp;
use std::marker::PhantomData;

use super::unit_traits::*;
use crate::math::*;
use crate::math_helpers::*;
use crate::math_impl::impl_math;
use crate::unit::None;
use crate::Unit;

pub trait TempUnit: Unit {
    fn to_base(s: Temperature<Self>) -> Temperature<Kelvin>;
    fn from_base(s: Temperature<Kelvin>) -> Temperature<Self>;
    fn unit() -> &'static str;
    fn mult() -> number;
}

impl_math!(Temperature<M: TempUnit>);

#[derive(Copy, Clone, Debug)]
pub struct Temperature<M: TempUnit> {
    pub inner: number,
    pub types: PhantomData<M>,
}

impl<M: TempUnit> Temperature<M> {
    pub fn convert<D: TempUnit>(self) -> Temperature<D> {
        D::from_base(M::to_base(self))
    }
}

impl<M> Display for Temperature<M>
where
    M: TempUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, M::unit())
    }
}

impl<M> LowerExp for Temperature<M>
where
    M: TempUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, M::unit())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Kelvin;

impl Unit for Kelvin {}
impl TempUnit for Kelvin {
    fn to_base(s: Temperature<Self>) -> Temperature<Kelvin> {
        s
    }

    fn from_base(s: Temperature<Kelvin>) -> Temperature<Self> {
        s
    }

    fn unit() -> &'static str {
        "K"
    }
    fn mult() -> number {
        1.
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Celsius;

impl Unit for Celsius {}
impl TempUnit for Celsius {
    fn to_base(s: Temperature<Self>) -> Temperature<Kelvin> {
        // No need to check, due to invariant for Temperature.
        Temperature {
            inner: s.inner + 273.15,
            types: PhantomData,
        }
    }

    fn from_base(s: Temperature<Kelvin>) -> Temperature<Self> {
        // No need to check, due to invariant for Temperature.
        Temperature {
            inner: s.inner - 273.15,
            types: PhantomData,
        }
    }

    fn unit() -> &'static str {
        "°C"
    }

    fn mult() -> number {
        1.
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Fahrenheit;

impl Unit for Fahrenheit {}
impl TempUnit for Fahrenheit {
    fn to_base(s: Temperature<Self>) -> Temperature<Kelvin> {
        // No need to check, due to invariant for Temperature.
        Temperature {
            inner: (s.inner - 32.) * (5. / 9.) + 273.15,
            types: PhantomData,
        }
    }

    fn from_base(s: Temperature<Kelvin>) -> Temperature<Self> {
        Temperature {
            inner: (s.inner - 273.15) * (9. / 5.) + 32.,
            types: PhantomData,
        }
    }

    fn unit() -> &'static str {
        "°F"
    }
    fn mult() -> number {
        1.8
    }
}

// impl<T: TimeUnit, L: LengthUnit> DerivedToUnit
//     for Derived<LengthPower<Zero, L>, TimePower<One, T>>
// {
//     type Output = Mass<T>;
//     type LU = L;
//     type TU = T;

//     fn to_unit(self) -> Self::Output {
//         Mass {
//             inner: self.inner,
//             types: PhantomData,
//         }
//     }
// }

// impl<T: TimeUnit> UnitToDerived for Mass<T> {
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
