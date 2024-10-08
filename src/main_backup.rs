use std::marker::PhantomData;
use std::ops::{Add, Mul};
use types::math::*;
// use types::type_level_math::*;

fn main() {
    println!("Hello, world!");
}

trait LengthUnit: Copy {}
trait TimeUnit: Copy {}

#[derive(Copy, Clone, Debug)]
struct Second;
impl TimeUnit for Second {}

#[derive(Copy, Clone, Debug)]
struct Meter;
impl LengthUnit for Meter {}

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

trait TimeTrait {}
struct TimePower<T: Number>(PhantomData<T>);
trait LengthTrait {}
struct LengthPower<L: Number>(PhantomData<L>);

// pub struct Number<const N: i8>;

// impl<A: Number, B: Number> Add<TimePower<B>> for TimePower<A>
// where
//     A: Add<B>,
//     <A as Add<B>>::Output: Number,
// {
//     type Output = TimePower<<A as Add<B>>::Output>;

//     fn add(self, _: TimePower<B>) -> Self::Output {
//         TimePower(PhantomData)
//     }
// }

// impl<A: Number, B: Number> Add<LengthPower<B>> for LengthPower<A>
// where
//     A: Add<B>,
//     <A as Add<B>>::Output: Number,
// {
//     type Output = LengthPower<<A as Add<B>>::Output>;

//     fn add(self, _: LengthPower<B>) -> Self::Output {
//         LengthPower(PhantomData)
//     }
// }

impl<T: Number> TimeTrait for TimePower<T> {}
impl<L: Number> LengthTrait for LengthPower<L> {}

struct Derived<L: LengthTrait, T: TimeTrait> {
    types: PhantomData<(T, L)>,
    inner: f32,
}

trait UnitToDerived {
    type LP: Number;
    type TP: Number;
    // type LP: LengthTrait;
    // type TP: TimeTrait;
    fn to_derived(self) -> Derived<LengthPower<Self::LP>, TimePower<Self::TP>>;
}

trait DerivedToUnit {
    type Output;
    fn to_unit(self) -> Self::Output;
}

impl DerivedToUnit for Derived<LengthPower<One>, TimePower<Zero>> {
    type Output = Length<Meter>;

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
    fn to_derived(self) -> Derived<LengthPower<Self::LP>, TimePower<Self::TP>> {
        Derived {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl DerivedToUnit for Derived<LengthPower<Zero>, TimePower<One>> {
    type Output = Time<Second>;

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
    fn to_derived(self) -> Derived<LengthPower<Self::LP>, TimePower<Self::TP>> {
        Derived {
            inner: self.inner,
            types: PhantomData,
        }
    }
}

impl DerivedToUnit for Derived<LengthPower<One>, TimePower<NegOne>> {
    type Output = Speed<Meter, Second>;

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
    fn to_derived(self) -> Derived<LengthPower<One>, TimePower<NegOne>> {
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
    T::LP: Add<U::LP>,
    <T::LP as Add<U::LP>>::Output: Number,
    <T::TP as Add<U::TP>>::Output: Number,
    T::TP: Add<U::TP>,
    Derived<LengthPower<<T::LP as Add<U::LP>>::Output>, TimePower<<T::TP as Add<U::TP>>::Output>>:
        DerivedToUnit,
{
    type Output = <Derived<
        LengthPower<<T::LP as Add<U::LP>>::Output>,
        TimePower<<T::TP as Add<U::TP>>::Output>,
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
    T::TP: Number,
    U::TP: Number,
    T::LP: Add<<U::LP as Number>::Neg>,
    <T::LP as Add<<U::LP as Number>::Neg>>::Output: Number,
    T::TP: Add<<U::TP as Number>::Neg>,
    <T::TP as Add<<U::TP as Number>::Neg>>::Output: Number,
    Derived<
        LengthPower<<T::LP as Add<<U::LP as Number>::Neg>>::Output>,
        TimePower<<T::TP as Add<<U::TP as Number>::Neg>>::Output>,
    >: DerivedToUnit,
{
    type Output = <Derived<
        LengthPower<<T::LP as Add<<U::LP as Number>::Neg>>::Output>,
        TimePower<<T::TP as Add<<U::TP as Number>::Neg>>::Output>,
    > as DerivedToUnit>::Output;

    fn div(self, rhs: U) -> Self::Output {
        Derived {
            types: PhantomData,
            inner: self.to_derived().inner / rhs.to_derived().inner,
        }
        .to_unit()
    }
}

fn test() {
    let meter = Length {
        types: PhantomData::<Meter>,
        inner: 5.,
    };
    let sec = Time {
        types: PhantomData::<Second>,
        inner: 1.,
    };
    // let a: Derived<LengthPower<One>, TimePower<Zero>> = meter.to_derived();
    // let b = sec.to_derived();
    let x = meter.div(sec);
    let y = x.mul(sec);
}

// impl<B, L: LengthUnit, LP1: Number, LP2: Number, TP1: Number, TP2: Number> std::ops::Mul<B>
//     for Length<L>
// where
//     B: Into<Derived<LengthPower<LP1>, TimePower<TP1>>>,
//     LP1: TLAdd<LP2>,
//     TP1: TLAdd<TP2>,
//     Derived<LengthPower<<LP1 as TLAdd<LP2>>::Output>, TimePower<<LP1 as TLAdd<LP2>>::Output>>:
//         DerivedToUnit,
// {
//     type Output =
//         <Derived<LengthPower<add!(LP1, LP2)>, TimePower<add!(LP1, LP2)>> as DerivedToUnit>::Output;

//     fn mul(self, rhs: B) -> Self::Output {
//         todo!()
//     }
// }

trait X {
    type Output;
}

// impl<T, L: LengthUnit> Mul<T> for Length<L> {
//     type Output;

//     fn mul(self, rhs: T) -> Self::Output {
//         todo!()
//     }
// }
