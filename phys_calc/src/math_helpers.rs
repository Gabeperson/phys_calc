use std::marker::PhantomData;
use std::ops::Add;

use crate::base::unit_traits::*;
use crate::math::*;

pub trait MulHelper<T> {
    type Output;
    fn multiply(self, rhs: T) -> Self::Output;
}

pub trait DivHelper<T> {
    type Output;
    fn divide(self, rhs: T) -> Self::Output;
}

pub trait TimeTrait {}
pub struct TimePower<T: Number, U: TimeUnit>(PhantomData<(T, U)>);
impl<T: Number, U: TimeUnit> TimeTrait for TimePower<T, U> {}

pub trait LengthTrait {}
pub struct LengthPower<L: Number, U: LengthUnit>(PhantomData<(L, U)>);
impl<L: Number, U: LengthUnit> LengthTrait for LengthPower<L, U> {}

pub trait TemperatureTrait {}
pub struct TemperaturePower<TMP: Number, U: TempUnit>(PhantomData<(TMP, U)>);
impl<TMP: Number, U: TempUnit> TemperatureTrait for TemperaturePower<TMP, U> {}

pub trait MassTrait {}
pub struct MassPower<M: Number, U: TempUnit>(PhantomData<(M, U)>);
impl<M: Number, U: TempUnit> MassTrait for MassPower<M, U> {}

pub trait CurrentTrait {}
pub struct CurrentPower<M: Number, U: TempUnit>(PhantomData<(M, U)>);
impl<C: Number, U: TempUnit> CurrentTrait for CurrentPower<C, U> {}

pub trait LuminousIntensityTrait {}
pub struct LuminousIntensityPower<M: Number, U: TempUnit>(PhantomData<(M, U)>);
impl<LI: Number, U: TempUnit> LuminousIntensityTrait for LuminousIntensityPower<LI, U> {}

pub trait QuantityTrait {}
pub struct QuantityPower<M: Number, U: TempUnit>(PhantomData<(M, U)>);
impl<Q: Number, U: TempUnit> QuantityTrait for QuantityPower<Q, U> {}

pub trait AngleTrait {}
pub struct AnglePower<M: Number, U: TempUnit>(PhantomData<(M, U)>);
impl<A: Number, U: TempUnit> AngleTrait for AnglePower<A, U> {}

pub trait SolidAngleTrait {}
pub struct SolidAnglePower<M: Number, U: TempUnit>(PhantomData<(M, U)>);
impl<S: Number, U: TempUnit> SolidAngleTrait for SolidAnglePower<S, U> {}

pub trait DigitalInformationTrait {}
pub struct DigitalInformationPower<M: Number, U: TempUnit>(PhantomData<(M, U)>);
impl<D: Number, U: TempUnit> DigitalInformationTrait for DigitalInformationPower<D, U> {}

pub struct Derived<L: LengthTrait, T: TimeTrait> {
    pub(crate) types: PhantomData<(T, L)>,
    pub inner: number,
}

pub trait UnitToDerived {
    type LengthExp: Number;
    type TimeExp: Number;

    type LengthUnit: LengthUnit;
    type TimeUnit: TimeUnit;
    // type LP: LengthTrait;
    // type TP: TimeTrait;

    // I can't think of any actually good way to make this simpler
    #[allow(clippy::type_complexity)]
    fn to_derived(
        self,
    ) -> Derived<
        LengthPower<Self::LengthExp, Self::LengthUnit>,
        TimePower<Self::TimeExp, Self::TimeUnit>,
    >;
}

pub trait DerivedToUnit {
    type Output;
    type LU: LengthUnit;
    type TU: TimeUnit;
    fn to_unit(self) -> Self::Output;
}

impl<T, U> MulHelper<U> for T
where
    T: UnitToDerived,
    U: UnitToDerived,

    T::LengthUnit: EqualsOrZero<U::LengthUnit>,
    T::TimeUnit: EqualsOrZero<U::TimeUnit>,

    <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType: LengthUnit,
    <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType: TimeUnit,

    T::LengthExp: Add<U::LengthExp>,
    <T::LengthExp as Add<U::LengthExp>>::Output: Number,
    <T::TimeExp as Add<U::TimeExp>>::Output: Number,
    T::TimeExp: Add<U::TimeExp>,
    Derived<
        LengthPower<
            <T::LengthExp as Add<U::LengthExp>>::Output,
            <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType,
        >,
        TimePower<
            <T::TimeExp as Add<U::TimeExp>>::Output,
            <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType,
        >,
    >: DerivedToUnit,
{
    type Output = <Derived<
        LengthPower<
            <T::LengthExp as Add<U::LengthExp>>::Output,
            <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType,
        >,
        TimePower<
            <T::TimeExp as Add<U::TimeExp>>::Output,
            <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType,
        >,
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

    T::LengthUnit: EqualsOrZero<U::LengthUnit>,
    T::TimeUnit: EqualsOrZero<U::TimeUnit>,

    <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType: LengthUnit,
    <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType: TimeUnit,

    T::TimeExp: Number,
    U::TimeExp: Number,
    T::LengthExp: Add<<U::LengthExp as Number>::Neg>,
    <T::LengthExp as Add<<U::LengthExp as Number>::Neg>>::Output: Number,
    T::TimeExp: Add<<U::TimeExp as Number>::Neg>,
    <T::TimeExp as Add<<U::TimeExp as Number>::Neg>>::Output: Number,
    Derived<
        LengthPower<
            <T::LengthExp as Add<<U::LengthExp as Number>::Neg>>::Output,
            <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType,
        >,
        TimePower<
            <T::TimeExp as Add<<U::TimeExp as Number>::Neg>>::Output,
            <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType,
        >,
    >: DerivedToUnit,
{
    type Output = <Derived<
        LengthPower<
            <T::LengthExp as Add<<U::LengthExp as Number>::Neg>>::Output,
            <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType,
        >,
        TimePower<
            <T::TimeExp as Add<<U::TimeExp as Number>::Neg>>::Output,
            <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType,
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
