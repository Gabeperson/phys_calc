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

pub trait LuminousIntensityUnit: Unit {
    fn to_base(s: LuminousIntensity<Self>) -> LuminousIntensity<Candela> {
        LuminousIntensity {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: LuminousIntensity<Candela>) -> LuminousIntensity<Self> {
        LuminousIntensity {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn unit() -> &'static str;
    fn mult() -> number;
}
impl_math!(LuminousIntensity<L: LuminousIntensityUnit>);

#[derive(Copy, Clone, Debug)]
pub struct LuminousIntensity<L: LuminousIntensityUnit> {
    pub inner: number,
    pub types: PhantomData<L>,
}

impl<L: LuminousIntensityUnit> LuminousIntensity<L> {
    pub fn convert<D: LuminousIntensityUnit>(self) -> LuminousIntensity<D> {
        D::from_base(L::to_base(self))
    }
}

impl<L> Display for LuminousIntensity<L>
where
    L: LuminousIntensityUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, L::unit())
    }
}

impl<L> LowerExp for LuminousIntensity<L>
where
    L: LuminousIntensityUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, L::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1.)]
#[unit("cd")]
#[unit_impl(LuminousIntensityUnit)]
pub struct Candela;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.981)]
#[unit("cp")]
#[unit_impl(LuminousIntensityUnit)]
pub struct CandlePower;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(0.920)]
#[unit("HK")]
#[unit_impl(LuminousIntensityUnit)]
pub struct HefnerLamp;

// impl<T: TimeUnit, L: LengthUnit> DerivedToUnit
//     for Derived<LengthPower<One, L>, TimePower<Zero, T>>
// {
//     type Output = Length<L>;
//     type LU = L;
//     type TU = T;

//     fn to_unit(self) -> Self::Output {
//         Length {
//             inner: self.inner,
//             types: PhantomData,
//         }
//     }
// }

// impl<L: LengthUnit> UnitToDerived for Length<L> {
//     type LP = One;
//     type TP = Zero;

//     type LU = L;
//     type TU = None;
//     fn to_derived(self) -> Derived<LengthPower<Self::LP, Self::LU>, TimePower<Self::TP, Self::TU>> {
//         Derived {
//             inner: self.inner,
//             types: PhantomData,
//         }
//     }
// }
