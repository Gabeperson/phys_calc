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

pub trait TempTrait {}
pub struct TempPower<TMP: Number, U: TempUnit>(PhantomData<(TMP, U)>);
impl<TMP: Number, U: TempUnit> TempTrait for TempPower<TMP, U> {}

pub trait MassTrait {}
pub struct MassPower<M: Number, U: MassUnit>(PhantomData<(M, U)>);
impl<M: Number, U: MassUnit> MassTrait for MassPower<M, U> {}

pub trait CurrentTrait {}
pub struct CurrentPower<M: Number, U: CurrentUnit>(PhantomData<(M, U)>);
impl<C: Number, U: CurrentUnit> CurrentTrait for CurrentPower<C, U> {}

pub trait LuminousIntensityTrait {}
pub struct LuminousIntensityPower<M: Number, U: LuminousIntensityUnit>(PhantomData<(M, U)>);
impl<LI: Number, U: LuminousIntensityUnit> LuminousIntensityTrait
    for LuminousIntensityPower<LI, U>
{
}

pub trait QuantityTrait {}
pub struct QuantityPower<M: Number, U: QuantityUnit>(PhantomData<(M, U)>);
impl<Q: Number, U: QuantityUnit> QuantityTrait for QuantityPower<Q, U> {}

pub trait AngleTrait {}
pub struct AnglePower<M: Number, U: AngleUnit>(PhantomData<(M, U)>);
impl<A: Number, U: AngleUnit> AngleTrait for AnglePower<A, U> {}

pub trait SolidAngleTrait {}
pub struct SolidAnglePower<M: Number, U: SolidAngleUnit>(PhantomData<(M, U)>);
impl<S: Number, U: SolidAngleUnit> SolidAngleTrait for SolidAnglePower<S, U> {}

pub trait DigitalInformationTrait {}
pub struct DigitalInformationPower<M: Number, U: DigitalInformationUnit>(PhantomData<(M, U)>);
impl<D: Number, U: DigitalInformationUnit> DigitalInformationTrait
    for DigitalInformationPower<D, U>
{
}

pub struct Derived<
    L: LengthTrait,
    T: TimeTrait,
    TMP: TempTrait,
    M: MassTrait,
    C: CurrentTrait,
    LI: LuminousIntensityTrait,
    Q: QuantityTrait,
    A: AngleTrait,
    S: SolidAngleTrait,
    D: DigitalInformationTrait,
> {
    // I can't think of any actually good way to make this simpler
    // If anyone can, feel free to PR
    #[allow(clippy::type_complexity)]
    pub(crate) types: PhantomData<(T, L, TMP, M, C, LI, Q, A, S, D)>,
    pub inner: number,
}

pub trait UnitToDerived {
    type LengthExp: Number;
    type TimeExp: Number;
    type TempExp: Number;
    type MassExp: Number;
    type CurrentExp: Number;
    type LuminousIntensityExp: Number;
    type QuantityExp: Number;
    type AngleExp: Number;
    type SolidAngleExp: Number;
    type DigitalInformationExp: Number;

    type LengthUnit: LengthUnit;
    type TimeUnit: TimeUnit;
    type TempUnit: TempUnit;
    type MassUnit: MassUnit;
    type CurrentUnit: CurrentUnit;
    type LuminousIntensityUnit: LuminousIntensityUnit;
    type QuantityUnit: QuantityUnit;
    type AngleUnit: AngleUnit;
    type SolidAngleUnit: SolidAngleUnit;
    type DigitalInformationUnit: DigitalInformationUnit;

    // I can't think of any actually good way to make this simpler
    // If anyone can, feel free to PR
    #[allow(clippy::type_complexity)]
    fn to_derived(
        self,
    ) -> Derived<
        LengthPower<Self::LengthExp, Self::LengthUnit>,
        TimePower<Self::TimeExp, Self::TimeUnit>,
        TempPower<Self::TempExp, Self::TempUnit>,
        MassPower<Self::MassExp, Self::MassUnit>,
        CurrentPower<Self::CurrentExp, Self::CurrentUnit>,
        LuminousIntensityPower<Self::LuminousIntensityExp, Self::LuminousIntensityUnit>,
        QuantityPower<Self::QuantityExp, Self::QuantityUnit>,
        AnglePower<Self::AngleExp, Self::AngleUnit>,
        SolidAnglePower<Self::SolidAngleExp, Self::SolidAngleUnit>,
        DigitalInformationPower<Self::DigitalInformationExp, Self::DigitalInformationUnit>,
    >;
}

pub trait DerivedToUnit {
    type Output;
    fn to_unit(self) -> Self::Output;
}

impl<T, U> MulHelper<U> for T
where
    T: UnitToDerived,
    U: UnitToDerived,

    T::LengthUnit: EqualsOrZero<U::LengthUnit>,
    T::TimeUnit: EqualsOrZero<U::TimeUnit>,
    T::TempUnit: EqualsOrZero<U::TempUnit>,
    T::MassUnit: EqualsOrZero<U::MassUnit>,
    T::CurrentUnit: EqualsOrZero<U::CurrentUnit>,
    T::LuminousIntensityUnit: EqualsOrZero<U::LuminousIntensityUnit>,
    T::QuantityUnit: EqualsOrZero<U::QuantityUnit>,
    T::AngleUnit: EqualsOrZero<U::AngleUnit>,
    T::SolidAngleUnit: EqualsOrZero<U::SolidAngleUnit>,
    T::DigitalInformationUnit: EqualsOrZero<U::DigitalInformationUnit>,

    <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType: LengthUnit,
    <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType: TimeUnit,
    <T::TempUnit as EqualsOrZero<U::TempUnit>>::SelfType: TempUnit,
    <T::MassUnit as EqualsOrZero<U::MassUnit>>::SelfType: MassUnit,
    <T::CurrentUnit as EqualsOrZero<U::CurrentUnit>>::SelfType: CurrentUnit,
    <T::LuminousIntensityUnit as EqualsOrZero<U::LuminousIntensityUnit>>::SelfType:
        LuminousIntensityUnit,
    <T::QuantityUnit as EqualsOrZero<U::QuantityUnit>>::SelfType: QuantityUnit,
    <T::AngleUnit as EqualsOrZero<U::AngleUnit>>::SelfType: AngleUnit,
    <T::SolidAngleUnit as EqualsOrZero<U::SolidAngleUnit>>::SelfType: SolidAngleUnit,
    <T::DigitalInformationUnit as EqualsOrZero<U::DigitalInformationUnit>>::SelfType:
        DigitalInformationUnit,

    T::LengthExp: Add<U::LengthExp>,
    T::TimeExp: Add<U::TimeExp>,
    T::TempExp: Add<U::TempExp>,
    T::MassExp: Add<U::MassExp>,
    T::CurrentExp: Add<U::CurrentExp>,
    T::LuminousIntensityExp: Add<U::LuminousIntensityExp>,
    T::QuantityExp: Add<U::QuantityExp>,
    T::AngleExp: Add<U::AngleExp>,
    T::SolidAngleExp: Add<U::SolidAngleExp>,
    T::DigitalInformationExp: Add<U::DigitalInformationExp>,

    // <T::LengthExp as Add<U::LengthExp>>::Output: Number,
    // <T::TimeExp as Add<U::TimeExp>>::Output: Number,
    <T::LengthExp as Add<U::LengthExp>>::Output: Number,
    <T::TimeExp as Add<U::TimeExp>>::Output: Number,
    <T::TempExp as Add<U::TempExp>>::Output: Number,
    <T::MassExp as Add<U::MassExp>>::Output: Number,
    <T::CurrentExp as Add<U::CurrentExp>>::Output: Number,
    <T::LuminousIntensityExp as Add<U::LuminousIntensityExp>>::Output: Number,
    <T::QuantityExp as Add<U::QuantityExp>>::Output: Number,
    <T::AngleExp as Add<U::AngleExp>>::Output: Number,
    <T::SolidAngleExp as Add<U::SolidAngleExp>>::Output: Number,
    <T::DigitalInformationExp as Add<U::DigitalInformationExp>>::Output: Number,
    Derived<
        LengthPower<
            <T::LengthExp as Add<U::LengthExp>>::Output,
            <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType,
        >,
        TimePower<
            <T::TimeExp as Add<U::TimeExp>>::Output,
            <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType,
        >,
        TempPower<
            <T::TempExp as Add<U::TempExp>>::Output,
            <T::TempUnit as EqualsOrZero<U::TempUnit>>::SelfType,
        >,
        MassPower<
            <T::MassExp as Add<U::MassExp>>::Output,
            <T::MassUnit as EqualsOrZero<U::MassUnit>>::SelfType,
        >,
        CurrentPower<
            <T::CurrentExp as Add<U::CurrentExp>>::Output,
            <T::CurrentUnit as EqualsOrZero<U::CurrentUnit>>::SelfType,
        >,
        LuminousIntensityPower<
            <T::LuminousIntensityExp as Add<U::LuminousIntensityExp>>::Output,
            <T::LuminousIntensityUnit as EqualsOrZero<U::LuminousIntensityUnit>>::SelfType,
        >,
        QuantityPower<
            <T::QuantityExp as Add<U::QuantityExp>>::Output,
            <T::QuantityUnit as EqualsOrZero<U::QuantityUnit>>::SelfType,
        >,
        AnglePower<
            <T::AngleExp as Add<U::AngleExp>>::Output,
            <T::AngleUnit as EqualsOrZero<U::AngleUnit>>::SelfType,
        >,
        SolidAnglePower<
            <T::SolidAngleExp as Add<U::SolidAngleExp>>::Output,
            <T::SolidAngleUnit as EqualsOrZero<U::SolidAngleUnit>>::SelfType,
        >,
        DigitalInformationPower<
            <T::DigitalInformationExp as Add<U::DigitalInformationExp>>::Output,
            <T::DigitalInformationUnit as EqualsOrZero<U::DigitalInformationUnit>>::SelfType,
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
        TempPower<
            <T::TempExp as Add<U::TempExp>>::Output,
            <T::TempUnit as EqualsOrZero<U::TempUnit>>::SelfType,
        >,
        MassPower<
            <T::MassExp as Add<U::MassExp>>::Output,
            <T::MassUnit as EqualsOrZero<U::MassUnit>>::SelfType,
        >,
        CurrentPower<
            <T::CurrentExp as Add<U::CurrentExp>>::Output,
            <T::CurrentUnit as EqualsOrZero<U::CurrentUnit>>::SelfType,
        >,
        LuminousIntensityPower<
            <T::LuminousIntensityExp as Add<U::LuminousIntensityExp>>::Output,
            <T::LuminousIntensityUnit as EqualsOrZero<U::LuminousIntensityUnit>>::SelfType,
        >,
        QuantityPower<
            <T::QuantityExp as Add<U::QuantityExp>>::Output,
            <T::QuantityUnit as EqualsOrZero<U::QuantityUnit>>::SelfType,
        >,
        AnglePower<
            <T::AngleExp as Add<U::AngleExp>>::Output,
            <T::AngleUnit as EqualsOrZero<U::AngleUnit>>::SelfType,
        >,
        SolidAnglePower<
            <T::SolidAngleExp as Add<U::SolidAngleExp>>::Output,
            <T::SolidAngleUnit as EqualsOrZero<U::SolidAngleUnit>>::SelfType,
        >,
        DigitalInformationPower<
            <T::DigitalInformationExp as Add<U::DigitalInformationExp>>::Output,
            <T::DigitalInformationUnit as EqualsOrZero<U::DigitalInformationUnit>>::SelfType,
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
    T::TempUnit: EqualsOrZero<U::TempUnit>,
    T::MassUnit: EqualsOrZero<U::MassUnit>,
    T::CurrentUnit: EqualsOrZero<U::CurrentUnit>,
    T::LuminousIntensityUnit: EqualsOrZero<U::LuminousIntensityUnit>,
    T::QuantityUnit: EqualsOrZero<U::QuantityUnit>,
    T::AngleUnit: EqualsOrZero<U::AngleUnit>,
    T::SolidAngleUnit: EqualsOrZero<U::SolidAngleUnit>,
    T::DigitalInformationUnit: EqualsOrZero<U::DigitalInformationUnit>,

    <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType: LengthUnit,
    <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType: TimeUnit,
    <T::TempUnit as EqualsOrZero<U::TempUnit>>::SelfType: TempUnit,
    <T::MassUnit as EqualsOrZero<U::MassUnit>>::SelfType: MassUnit,
    <T::CurrentUnit as EqualsOrZero<U::CurrentUnit>>::SelfType: CurrentUnit,
    <T::LuminousIntensityUnit as EqualsOrZero<U::LuminousIntensityUnit>>::SelfType:
        LuminousIntensityUnit,
    <T::QuantityUnit as EqualsOrZero<U::QuantityUnit>>::SelfType: QuantityUnit,
    <T::AngleUnit as EqualsOrZero<U::AngleUnit>>::SelfType: AngleUnit,
    <T::SolidAngleUnit as EqualsOrZero<U::SolidAngleUnit>>::SelfType: SolidAngleUnit,
    <T::DigitalInformationUnit as EqualsOrZero<U::DigitalInformationUnit>>::SelfType:
        DigitalInformationUnit,

    T::LengthExp: Add<<U::LengthExp as Number>::Neg>,
    T::TimeExp: Add<<U::TimeExp as Number>::Neg>,
    T::TempExp: Add<<U::TempExp as Number>::Neg>,
    T::MassExp: Add<<U::MassExp as Number>::Neg>,
    T::CurrentExp: Add<<U::CurrentExp as Number>::Neg>,
    T::LuminousIntensityExp: Add<<U::LuminousIntensityExp as Number>::Neg>,
    T::QuantityExp: Add<<U::QuantityExp as Number>::Neg>,
    T::AngleExp: Add<<U::AngleExp as Number>::Neg>,
    T::SolidAngleExp: Add<<U::SolidAngleExp as Number>::Neg>,
    T::DigitalInformationExp: Add<<U::DigitalInformationExp as Number>::Neg>,

    <T::LengthExp as Add<<U::LengthExp as Number>::Neg>>::Output: Number,
    <T::TimeExp as Add<<U::TimeExp as Number>::Neg>>::Output: Number,
    <T::TempExp as Add<<U::TempExp as Number>::Neg>>::Output: Number,
    <T::MassExp as Add<<U::MassExp as Number>::Neg>>::Output: Number,
    <T::CurrentExp as Add<<U::CurrentExp as Number>::Neg>>::Output: Number,
    <T::LuminousIntensityExp as Add<<U::LuminousIntensityExp as Number>::Neg>>::Output: Number,
    <T::QuantityExp as Add<<U::QuantityExp as Number>::Neg>>::Output: Number,
    <T::AngleExp as Add<<U::AngleExp as Number>::Neg>>::Output: Number,
    <T::SolidAngleExp as Add<<U::SolidAngleExp as Number>::Neg>>::Output: Number,
    <T::DigitalInformationExp as Add<<U::DigitalInformationExp as Number>::Neg>>::Output: Number,

    Derived<
        // LengthPower<
        //     <T::LengthExp as Add<<U::LengthExp as Number>::Neg>>::Output,
        //     <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType,
        // >,
        // TimePower<
        //     <T::TimeExp as Add<<U::TimeExp as Number>::Neg>>::Output,
        //     <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType,
        // >,
        LengthPower<
            <T::LengthExp as Add<<U::LengthExp as Number>::Neg>>::Output,
            <T::LengthUnit as EqualsOrZero<U::LengthUnit>>::SelfType,
        >,
        TimePower<
            <T::TimeExp as Add<<U::TimeExp as Number>::Neg>>::Output,
            <T::TimeUnit as EqualsOrZero<U::TimeUnit>>::SelfType,
        >,
        TempPower<
            <T::TempExp as Add<<U::TempExp as Number>::Neg>>::Output,
            <T::TempUnit as EqualsOrZero<U::TempUnit>>::SelfType,
        >,
        MassPower<
            <T::MassExp as Add<<U::MassExp as Number>::Neg>>::Output,
            <T::MassUnit as EqualsOrZero<U::MassUnit>>::SelfType,
        >,
        CurrentPower<
            <T::CurrentExp as Add<<U::CurrentExp as Number>::Neg>>::Output,
            <T::CurrentUnit as EqualsOrZero<U::CurrentUnit>>::SelfType,
        >,
        LuminousIntensityPower<
            <T::LuminousIntensityExp as Add<<U::LuminousIntensityExp as Number>::Neg>>::Output,
            <T::LuminousIntensityUnit as EqualsOrZero<U::LuminousIntensityUnit>>::SelfType,
        >,
        QuantityPower<
            <T::QuantityExp as Add<<U::QuantityExp as Number>::Neg>>::Output,
            <T::QuantityUnit as EqualsOrZero<U::QuantityUnit>>::SelfType,
        >,
        AnglePower<
            <T::AngleExp as Add<<U::AngleExp as Number>::Neg>>::Output,
            <T::AngleUnit as EqualsOrZero<U::AngleUnit>>::SelfType,
        >,
        SolidAnglePower<
            <T::SolidAngleExp as Add<<U::SolidAngleExp as Number>::Neg>>::Output,
            <T::SolidAngleUnit as EqualsOrZero<U::SolidAngleUnit>>::SelfType,
        >,
        DigitalInformationPower<
            <T::DigitalInformationExp as Add<<U::DigitalInformationExp as Number>::Neg>>::Output,
            <T::DigitalInformationUnit as EqualsOrZero<U::DigitalInformationUnit>>::SelfType,
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
        TempPower<
            <T::TempExp as Add<<U::TempExp as Number>::Neg>>::Output,
            <T::TempUnit as EqualsOrZero<U::TempUnit>>::SelfType,
        >,
        MassPower<
            <T::MassExp as Add<<U::MassExp as Number>::Neg>>::Output,
            <T::MassUnit as EqualsOrZero<U::MassUnit>>::SelfType,
        >,
        CurrentPower<
            <T::CurrentExp as Add<<U::CurrentExp as Number>::Neg>>::Output,
            <T::CurrentUnit as EqualsOrZero<U::CurrentUnit>>::SelfType,
        >,
        LuminousIntensityPower<
            <T::LuminousIntensityExp as Add<<U::LuminousIntensityExp as Number>::Neg>>::Output,
            <T::LuminousIntensityUnit as EqualsOrZero<U::LuminousIntensityUnit>>::SelfType,
        >,
        QuantityPower<
            <T::QuantityExp as Add<<U::QuantityExp as Number>::Neg>>::Output,
            <T::QuantityUnit as EqualsOrZero<U::QuantityUnit>>::SelfType,
        >,
        AnglePower<
            <T::AngleExp as Add<<U::AngleExp as Number>::Neg>>::Output,
            <T::AngleUnit as EqualsOrZero<U::AngleUnit>>::SelfType,
        >,
        SolidAnglePower<
            <T::SolidAngleExp as Add<<U::SolidAngleExp as Number>::Neg>>::Output,
            <T::SolidAngleUnit as EqualsOrZero<U::SolidAngleUnit>>::SelfType,
        >,
        DigitalInformationPower<
            <T::DigitalInformationExp as Add<<U::DigitalInformationExp as Number>::Neg>>::Output,
            <T::DigitalInformationUnit as EqualsOrZero<U::DigitalInformationUnit>>::SelfType,
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
