use std::fmt::Display;
use std::fmt::LowerExp;
use std::marker::PhantomData;

use phys_calc_macros::impl_derived_conversions;
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
use crate::specialization::False;
use crate::specialization::Specialized;
use crate::unit::None;
use crate::unspecialize;
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

unspecialize!(Angle<A: AngleUnit>);
impl_derived_conversions!(Angle<A: AngleUnit>, Angle: One, A);

// impl<
//         T: TimeUnit,
//         L: LengthUnit,
//         TMP: TempUnit,
//         M: MassUnit,
//         C: CurrentUnit,
//         LI: LuminousIntensityUnit,
//         Q: QuantityUnit,
//         A: AngleUnit,
//         S: SolidAngleUnit,
//         D: DigitalInformationUnit,
//     > DerivedToUnit
//     for Derived<
//         LengthPower<Zero, L>,
//         TimePower<Zero, T>,
//         TemperaturePower<Zero, TMP>,
//         MassPower<Zero, M>,
//         CurrentPower<Zero, C>,
//         LuminousIntensityPower<Zero, LI>,
//         QuantityPower<Zero, Q>,
//         AnglePower<One, A>,
//         SolidAnglePower<Zero, S>,
//         DigitalInformationPower<Zero, D>,
//     >
// where
//     Angle<A>: Specialized<Bool = False>,
// {
//     type Output = Angle<A>;

//     fn to_unit(self) -> Self::Output {
//         Angle {
//             inner: self.inner,
//             types: PhantomData,
//         }
//     }
// }

// impl<A: AngleUnit> UnitToDerived for Angle<A> {
//     type LengthExp = Zero;
//     type TimeExp = Zero;
//     type TempExp = Zero;
//     type MassExp = Zero;
//     type CurrentExp = Zero;
//     type LuminousIntensityExp = Zero;
//     type QuantityExp = Zero;
//     type AngleExp = One;
//     type SolidAngleExp = Zero;
//     type DigitalInformationExp = Zero;

//     type LengthUnit = None;
//     type TimeUnit = None;
//     type TempUnit = None;
//     type MassUnit = None;
//     type CurrentUnit = None;
//     type LuminousIntensityUnit = None;
//     type QuantityUnit = None;
//     type AngleUnit = A;
//     type SolidAngleUnit = None;
//     type DigitalInformationUnit = None;
//     fn to_derived(
//         self,
//     ) -> Derived<
//         LengthPower<Self::LengthExp, Self::LengthUnit>,
//         TimePower<Self::TimeExp, Self::TimeUnit>,
//         TemperaturePower<Self::TempExp, Self::TempUnit>,
//         MassPower<Self::MassExp, Self::MassUnit>,
//         CurrentPower<Self::CurrentExp, Self::CurrentUnit>,
//         LuminousIntensityPower<Self::LuminousIntensityExp, Self::LuminousIntensityUnit>,
//         QuantityPower<Self::QuantityExp, Self::QuantityUnit>,
//         AnglePower<Self::AngleExp, Self::AngleUnit>,
//         SolidAnglePower<Self::SolidAngleExp, Self::SolidAngleUnit>,
//         DigitalInformationPower<Self::DigitalInformationExp, Self::DigitalInformationUnit>,
//     > {
//         Derived {
//             inner: self.inner,
//             types: PhantomData,
//         }
//     }
// }
