#![feature(min_specialization)]
use phys_calc::math::*;
use std::marker::PhantomData;
use std::ops::{Add, Mul};
// use types::type_level_math::*;

trait LengthUnit: Copy {}
trait TimeUnit: Copy {}

#[derive(Copy, Clone, Debug)]
struct Second;
impl TimeUnit for Second {}
impl Equals for Second {
    type SelfType = Self;
}
impl Equals<None> for Second {
    type SelfType = Self;
}

#[derive(Copy, Clone, Debug)]
struct Meter;
impl LengthUnit for Meter {}
impl Equals for Meter {
    type SelfType = Self;
}
impl Equals<None> for Meter {
    type SelfType = Self;
}

#[derive(Copy, Clone, Debug)]
struct Kilometer;
impl LengthUnit for Kilometer {}
impl Equals for Kilometer {
    type SelfType = Self;
}
impl Equals<None> for Kilometer {
    type SelfType = Self;
}

#[derive(Copy, Clone, Debug)]
struct None;
impl LengthUnit for None {}
impl TimeUnit for None {}
impl<T> Equals<T> for None {
    type SelfType = T;
}

#[derive(Copy, Clone, Debug)]
struct Length<L: LengthUnit> {
    inner: f32,
    types: PhantomData<L>,
}

#[derive(Copy, Clone, Debug)]
struct Time<T: TimeUnit> {
    inner: f32,
    types: PhantomData<T>,
}

struct Speed<L: LengthUnit, T: TimeUnit> {
    types: PhantomData<(L, T)>,
    inner: f32,
}

struct Area<L: LengthUnit> {
    types: PhantomData<L>,
    inner: f32,
}

trait TimeTrait {}
struct TimePower<T: Number, U: TimeUnit>(PhantomData<(T, U)>);
trait LengthTrait {}
struct LengthPower<L: Number, U: LengthUnit>(PhantomData<(L, U)>);

impl<T: Number, U: TimeUnit> TimeTrait for TimePower<T, U> {}
impl<L: Number, U: LengthUnit> LengthTrait for LengthPower<L, U> {}

struct Derived<L: LengthTrait, T: TimeTrait> {
    types: PhantomData<(T, L)>,
    inner: f32,
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
    fn mul(self, rhs: T) -> Self::Output;
}

trait DivHelper<T> {
    type Output;
    fn div(self, rhs: T) -> Self::Output;
}

impl<T, U> MulHelper<U> for T
where
    T: UnitToDerived,
    U: UnitToDerived,

    T::LU: Equals<U::LU>,
    T::TU: Equals<U::TU>,

    <T::LU as Equals<U::LU>>::SelfType: LengthUnit,
    <T::TU as Equals<U::TU>>::SelfType: TimeUnit,

    T::LP: Add<U::LP>,
    <T::LP as Add<U::LP>>::Output: Number,
    <T::TP as Add<U::TP>>::Output: Number,
    T::TP: Add<U::TP>,
    Derived<
        LengthPower<<T::LP as Add<U::LP>>::Output, <T::LU as Equals<U::LU>>::SelfType>,
        TimePower<<T::TP as Add<U::TP>>::Output, <T::TU as Equals<U::TU>>::SelfType>,
    >: DerivedToUnit,
{
    type Output = <Derived<
        LengthPower<<T::LP as Add<U::LP>>::Output, <T::LU as Equals<U::LU>>::SelfType>,
        TimePower<<T::TP as Add<U::TP>>::Output, <T::TU as Equals<U::TU>>::SelfType>,
    > as DerivedToUnit>::Output;

    fn mul(self, rhs: U) -> Self::Output {
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

    T::LU: Equals<U::LU>,
    T::TU: Equals<U::TU>,

    <T::LU as Equals<U::LU>>::SelfType: LengthUnit,
    <T::TU as Equals<U::TU>>::SelfType: TimeUnit,

    T::TP: Number,
    U::TP: Number,
    T::LP: Add<<U::LP as Number>::Neg>,
    <T::LP as Add<<U::LP as Number>::Neg>>::Output: Number,
    T::TP: Add<<U::TP as Number>::Neg>,
    <T::TP as Add<<U::TP as Number>::Neg>>::Output: Number,
    Derived<
        LengthPower<
            <T::LP as Add<<U::LP as Number>::Neg>>::Output,
            <T::LU as Equals<U::LU>>::SelfType,
        >,
        TimePower<
            <T::TP as Add<<U::TP as Number>::Neg>>::Output,
            <T::TU as Equals<U::TU>>::SelfType,
        >,
    >: DerivedToUnit,
{
    type Output = <Derived<
        LengthPower<
            <T::LP as Add<<U::LP as Number>::Neg>>::Output,
            <T::LU as Equals<U::LU>>::SelfType,
        >,
        TimePower<
            <T::TP as Add<<U::TP as Number>::Neg>>::Output,
            <T::TU as Equals<U::TU>>::SelfType,
        >,
    > as DerivedToUnit>::Output;

    fn div(self, rhs: U) -> Self::Output {
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

    let a = meter.mul(meter);
    let a2 = kilometer.mul(kilometer);
    let v = kilometer.div(sec);
    // let a: Derived<LengthPower<One>, TimePower<Zero>> = meter.to_derived();
    // let b = sec.to_derived();
    let x = meter.div(sec);
    println!("{}", x.inner);
    // let y = x.mul(sec);
}
