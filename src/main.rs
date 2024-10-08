#![feature(min_specialization)]
use phys_calc::math::*;
use std::marker::PhantomData;
use std::ops::{Add, Mul};
// use types::type_level_math::*;

trait LengthUnit: Copy {}
trait TimeUnit: Copy {}

#[allow(non_camel_case_types)]
type number = f64;

pub trait Unit: Copy + Clone + Sized {
    fn mult() -> number;
}

#[derive(Copy, Clone, Debug)]
struct Second;
impl TimeUnit for Second {}
impl EqualsOrZero for Second {
    type SelfType = Self;
}
impl EqualsOrZero<None> for Second {
    type SelfType = Self;
}

#[derive(Copy, Clone, Debug)]
struct Meter;
impl LengthUnit for Meter {}
impl EqualsOrZero for Meter {
    type SelfType = Self;
}
impl EqualsOrZero<None> for Meter {
    type SelfType = Self;
}

#[derive(Copy, Clone, Debug)]
struct Kilometer;
impl LengthUnit for Kilometer {}
impl EqualsOrZero for Kilometer {
    type SelfType = Self;
}
impl EqualsOrZero<None> for Kilometer {
    type SelfType = Self;
}

#[derive(Copy, Clone, Debug)]
struct None;
impl LengthUnit for None {}
impl TimeUnit for None {}
impl<T> EqualsOrZero<T> for None {
    type SelfType = T;
}

#[derive(Copy, Clone, Debug)]
struct Length<L: LengthUnit> {
    inner: number,
    types: PhantomData<L>,
}

#[derive(Copy, Clone, Debug)]
struct Time<T: TimeUnit> {
    inner: number,
    types: PhantomData<T>,
}

struct Speed<L: LengthUnit, T: TimeUnit> {
    types: PhantomData<(L, T)>,
    inner: number,
}

struct Area<L: LengthUnit> {
    types: PhantomData<L>,
    inner: number,
}

trait TimeTrait {}
struct TimePower<T: Number, U: TimeUnit>(PhantomData<(T, U)>);
trait LengthTrait {}
struct LengthPower<L: Number, U: LengthUnit>(PhantomData<(L, U)>);

impl<T: Number, U: TimeUnit> TimeTrait for TimePower<T, U> {}
impl<L: Number, U: LengthUnit> LengthTrait for LengthPower<L, U> {}

struct Derived<L: LengthTrait, T: TimeTrait> {
    types: PhantomData<(T, L)>,
    inner: number,
}

trait UnitToDerived {
    type LP: Number;
    type TP: Number;

    type LU: LengthUnit;
    type TU: TimeUnit;
    // type LP: LengthTrait;
    // type TP: TimeTrait;
    fn to_derived(self) -> Derived<LengthPower<Self::LP, Self::LU>, TimePower<Self::TP, Self::TU>>;
}

trait DerivedToUnit {
    type Output;
    type LU: LengthUnit;
    type TU: TimeUnit;
    fn to_unit(self) -> Self::Output;
}

impl<T: TimeUnit, L: LengthUnit> DerivedToUnit
    for Derived<LengthPower<One, L>, TimePower<Zero, T>>
{
    type Output = Length<L>;
    type LU = L;
    type TU = T;

    fn to_unit(self /*, types: PhantomData<L>*/) -> Self::Output {
        Length {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl<L: LengthUnit> UnitToDerived for Length<L> {
    type LP = One;
    type TP = Zero;

    type LU = L;
    type TU = None;
    fn to_derived(self) -> Derived<LengthPower<Self::LP, Self::LU>, TimePower<Self::TP, Self::TU>> {
        Derived {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl<T: TimeUnit, L: LengthUnit> DerivedToUnit
    for Derived<LengthPower<Zero, L>, TimePower<One, T>>
{
    type Output = Time<T>;

    type LU = L;
    type TU = T;

    fn to_unit(self /*, types: PhantomData<L>*/) -> Self::Output {
        Time {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl<T: TimeUnit> UnitToDerived for Time<T> {
    type LP = Zero;
    type TP = One;

    type LU = None;
    type TU = T;

    fn to_derived(self) -> Derived<LengthPower<Self::LP, Self::LU>, TimePower<Self::TP, Self::TU>> {
        Derived {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl<T: TimeUnit, L: LengthUnit> DerivedToUnit
    for Derived<LengthPower<One, L>, TimePower<NegOne, T>>
{
    type Output = Speed<L, T>;

    type LU = L;
    type TU = T;

    fn to_unit(self /*, types: PhantomData<L>*/) -> Self::Output {
        Speed {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl<L: LengthUnit, T: TimeUnit> UnitToDerived for Speed<L, T> {
    type LP = One;
    type TP = NegOne;

    type TU = T;
    type LU = L;

    fn to_derived(self) -> Derived<LengthPower<One, Self::LU>, TimePower<NegOne, Self::TU>> {
        Derived {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl<T: TimeUnit, L: LengthUnit> DerivedToUnit
    for Derived<LengthPower<Two, L>, TimePower<Zero, T>>
{
    type Output = Area<L>;

    type LU = L;
    type TU = T;

    fn to_unit(self /*, types: PhantomData<L>*/) -> Self::Output {
        Area {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl<L: LengthUnit> UnitToDerived for Area<L> {
    type LP = Two;
    type TP = Zero;

    type TU = None;
    type LU = L;

    fn to_derived(self) -> Derived<LengthPower<Two, Self::LU>, TimePower<Zero, Self::TU>> {
        Derived {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

trait MulHelper<T> {
    type Output;
    fn multiply(self, rhs: T) -> Self::Output;
}

trait DivHelper<T> {
    type Output;
    fn divide(self, rhs: T) -> Self::Output;
}

impl<T, U> MulHelper<U> for T
where
    T: UnitToDerived,
    U: UnitToDerived,

    T::LU: EqualsOrZero<U::LU>,
    T::TU: EqualsOrZero<U::TU>,

    <T::LU as EqualsOrZero<U::LU>>::SelfType: LengthUnit,
    <T::TU as EqualsOrZero<U::TU>>::SelfType: TimeUnit,

    T::LP: Add<U::LP>,
    <T::LP as Add<U::LP>>::Output: Number,
    <T::TP as Add<U::TP>>::Output: Number,
    T::TP: Add<U::TP>,
    Derived<
        LengthPower<<T::LP as Add<U::LP>>::Output, <T::LU as EqualsOrZero<U::LU>>::SelfType>,
        TimePower<<T::TP as Add<U::TP>>::Output, <T::TU as EqualsOrZero<U::TU>>::SelfType>,
    >: DerivedToUnit,
{
    type Output = <Derived<
        LengthPower<<T::LP as Add<U::LP>>::Output, <T::LU as EqualsOrZero<U::LU>>::SelfType>,
        TimePower<<T::TP as Add<U::TP>>::Output, <T::TU as EqualsOrZero<U::TU>>::SelfType>,
    > as DerivedToUnit>::Output;

    fn multiply(self, rhs: U) -> Self::Output {
        Derived {
            types: PhantomData,
            inner: self.to_derived().inner * rhs.to_derived().inner,
        }
        .to_unit()
    }
}

impl<T, U> DivHelper<U> for T
where
    T: UnitToDerived,
    U: UnitToDerived,

    T::LU: EqualsOrZero<U::LU>,
    T::TU: EqualsOrZero<U::TU>,

    <T::LU as EqualsOrZero<U::LU>>::SelfType: LengthUnit,
    <T::TU as EqualsOrZero<U::TU>>::SelfType: TimeUnit,

    T::TP: Number,
    U::TP: Number,
    T::LP: Add<<U::LP as Number>::Neg>,
    <T::LP as Add<<U::LP as Number>::Neg>>::Output: Number,
    T::TP: Add<<U::TP as Number>::Neg>,
    <T::TP as Add<<U::TP as Number>::Neg>>::Output: Number,
    Derived<
        LengthPower<
            <T::LP as Add<<U::LP as Number>::Neg>>::Output,
            <T::LU as EqualsOrZero<U::LU>>::SelfType,
        >,
        TimePower<
            <T::TP as Add<<U::TP as Number>::Neg>>::Output,
            <T::TU as EqualsOrZero<U::TU>>::SelfType,
        >,
    >: DerivedToUnit,
{
    type Output = <Derived<
        LengthPower<
            <T::LP as Add<<U::LP as Number>::Neg>>::Output,
            <T::LU as EqualsOrZero<U::LU>>::SelfType,
        >,
        TimePower<
            <T::TP as Add<<U::TP as Number>::Neg>>::Output,
            <T::TU as EqualsOrZero<U::TU>>::SelfType,
        >,
    > as DerivedToUnit>::Output;

    fn divide(self, rhs: U) -> Self::Output {
        Derived {
            types: PhantomData,
            inner: self.to_derived().inner / rhs.to_derived().inner,
        }
        .to_unit()
    }
}

fn main() {
    let meter = Length {
        types: PhantomData::<Meter>,
        inner: 5.,
    };
    let sec = Time {
        types: PhantomData::<Second>,
        inner: 2.,
    };

    let kilometer = Length {
        types: PhantomData::<Kilometer>,
        inner: 5.,
    };

    let a = meter * (meter);
    let a2 = kilometer.multiply(kilometer);
    let v = kilometer.divide(sec);
    // let a: Derived<LengthPower<One>, TimePower<Zero>> = meter.to_derived();
    // let b = sec.to_derived();
    let x = meter.divide(sec);
    println!("{}", x.inner);
    // let y = x.mul(sec);
}

impl<T, L: LengthUnit> Mul<T> for Length<L>
where
    Self: MulHelper<T>,
{
    type Output = <Self as MulHelper<T>>::Output;

    fn mul(self, rhs: T) -> Self::Output {
        self.multiply(rhs)
    }
}
