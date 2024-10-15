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

pub trait DigitalInformationUnit: Unit {
    fn to_base(s: DigitalInformation<Self>) -> DigitalInformation<Byte> {
        DigitalInformation {
            inner: s.inner * Self::mult(),
            types: PhantomData,
        }
    }
    fn from_base(s: DigitalInformation<Byte>) -> DigitalInformation<Self> {
        DigitalInformation {
            inner: s.inner / Self::mult(),
            types: PhantomData,
        }
    }
    fn unit() -> &'static str;
    fn mult() -> number;
}
impl_math!(DigitalInformation<D: DigitalInformationUnit>);

#[derive(Copy, Clone, Debug)]
pub struct DigitalInformation<D: DigitalInformationUnit> {
    pub inner: number,
    pub types: PhantomData<D>,
}

impl<R: DigitalInformationUnit> DigitalInformation<R> {
    pub fn convert<D: DigitalInformationUnit>(self) -> DigitalInformation<D> {
        D::from_base(R::to_base(self))
    }
}

impl<R> Display for DigitalInformation<R>
where
    R: DigitalInformationUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.inner, R::unit())
    }
}

impl<R> LowerExp for DigitalInformation<R>
where
    R: DigitalInformationUnit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{}", self.inner, R::unit())
    }
}

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1)]
#[unit("bit")]
#[unit_impl(DigitalInformationUnit)]
pub struct Bit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8)]
#[unit("B")]
#[unit_impl(DigitalInformationUnit)]
pub struct Byte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1000)]
#[unit("Kb")]
#[unit_impl(DigitalInformationUnit)]
pub struct Kilobit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1024)]
#[unit("Kib")]
#[unit_impl(DigitalInformationUnit)]
pub struct Kibibit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1000)]
#[unit("KB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Kilobyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1024)]
#[unit("KiB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Kibibyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1000*1000)]
#[unit("Mb")]
#[unit_impl(DigitalInformationUnit)]
pub struct Megabit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1024*1024)]
#[unit("Mib")]
#[unit_impl(DigitalInformationUnit)]
pub struct Mebibit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1000*1000)]
#[unit("MB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Megabyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1024*1024)]
#[unit("MiB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Mebibyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1000*1000*1000)]
#[unit("Gb")]
#[unit_impl(DigitalInformationUnit)]
pub struct Gigabit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1024*1024*1024)]
#[unit("Gib")]
#[unit_impl(DigitalInformationUnit)]
pub struct Gibibit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1000*1000*1000u64)]
#[unit("GB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Gigabyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1024*1024*1024u64)]
#[unit("GiB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Gibibyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1000*1000*1000*1000u64)]
#[unit("Tb")]
#[unit_impl(DigitalInformationUnit)]
pub struct Terabit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1024*1024*1024*1024u64)]
#[unit("Tib")]
#[unit_impl(DigitalInformationUnit)]
pub struct Tebibit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1000*1000*1000*1000u64)]
#[unit("TB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Terabyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1024*1024*1024*1024u64)]
#[unit("TiB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Tebibyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1000*1000*1000*1000*1000u64)]
#[unit("Pb")]
#[unit_impl(DigitalInformationUnit)]
pub struct Petabit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1024*1024*1024*1024*1024u64)]
#[unit("Pib")]
#[unit_impl(DigitalInformationUnit)]
pub struct Pebibit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1000*1000*1000*1000*1000u64)]
#[unit("PB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Petabyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1024*1024*1024*1024*1024u64)]
#[unit("PiB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Pebibyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1000*1000*1000*1000*1000*1000u64)]
#[unit("Eb")]
#[unit_impl(DigitalInformationUnit)]
pub struct Exabit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(1024*1024*1024*1024*1024*1024u64)]
#[unit("Eib")]
#[unit_impl(DigitalInformationUnit)]
pub struct Exbibit;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1000*1000*1000*1000*1000*1000u64)]
#[unit("EB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Exabyte;

#[derive(Copy, Clone, Debug, Unit)]
#[multiplier(8*1024*1024*1024*1024*1024*1024u64)]
#[unit("EiB")]
#[unit_impl(DigitalInformationUnit)]
pub struct Exbibyte;

unspecialize!(DigitalInformation<D: DigitalInformationUnit>);
impl_derived_conversions!(DigitalInformation<D: DigitalInformationUnit>, DigitalInformation: One, D);
