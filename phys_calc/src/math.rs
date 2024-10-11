use std::ops::Add;

#[allow(non_camel_case_types)]
pub type number = f64;

#[derive(Clone, Copy, Debug)]
pub struct NegFive;
#[derive(Clone, Copy, Debug)]
pub struct NegFour;
#[derive(Clone, Copy, Debug)]
pub struct NegThree;
#[derive(Clone, Copy, Debug)]
pub struct NegTwo;
#[derive(Clone, Copy, Debug)]
pub struct NegOne;
#[derive(Clone, Copy, Debug)]
pub struct Zero;
#[derive(Clone, Copy, Debug)]
pub struct One;
#[derive(Clone, Copy, Debug)]
pub struct Two;
#[derive(Clone, Copy, Debug)]
pub struct Three;
#[derive(Clone, Copy, Debug)]
pub struct Four;
#[derive(Clone, Copy, Debug)]
pub struct Five;
#[derive(Clone, Copy, Debug)]
pub struct Invalid;

pub trait EqualsOrZero<T = Self> {
    type SelfType;
}
// impl<T> Equals<T> for T {}

mod private {
    pub trait Sealed {}
}
pub trait Specialized {
    type Bool: Boolean;
}
use private::Sealed;
pub trait Boolean: Sealed {}
#[derive(Debug, Clone, Copy)]
struct True;
#[derive(Debug, Clone, Copy)]
struct False;
impl Sealed for True {}
impl Boolean for True {}
impl Sealed for False {}
impl Boolean for False {}

pub trait Number:
    Copy
    + Add<NegFive>
    + Add<NegFour>
    + Add<NegThree>
    + Add<NegTwo>
    + Add<NegOne>
    + Add<Zero>
    + Add<One>
    + Add<Two>
    + Add<Three>
    + Add<Four>
    + Add<Five>
    + Add<Invalid>
{
    type Neg: Number;
    fn check() {}
}

macro_rules! impl_math {
    ($lhs:ident, $rhs:ident, $res:ident) => {
        impl Add<$rhs> for $lhs {
            type Output = $res;
            fn add(self, _rhs: $rhs) -> Self::Output {
                panic!("Type-level Add shouldn't be called with actual values!")
            }
        }
    };
}

impl Number for NegFive {
    type Neg = Five;
}
impl Number for NegFour {
    type Neg = Four;
}
impl Number for NegThree {
    type Neg = Three;
}
impl Number for NegTwo {
    type Neg = Two;
}
impl Number for NegOne {
    type Neg = One;
}
impl Number for Zero {
    type Neg = Zero;
}
impl Number for One {
    type Neg = NegOne;
}
impl Number for Two {
    type Neg = NegTwo;
}
impl Number for Three {
    type Neg = NegThree;
}
impl Number for Four {
    type Neg = NegFour;
}
impl Number for Five {
    type Neg = NegFive;
}
impl Number for Invalid {
    type Neg = Invalid;
}

/* Generated via python script:
m = {
    -5: "NegFive",
    -4: "NegFour",
    -3: "NegThree",
    -2: "NegTwo",
    -1: "NegOne",
     0: "Zero",
     1: "One",
     2: "Two",
     3: "Three",
     4: "Four",
     5: "Five",
     -999999: "Invalid", # we will never go this low
}


for n1, s1 in m.items():
    for n2, s2 in m.items():
        if n1+n2 > 5 or n1+n2 < -5:
            print(f"impl_math!({s1}, {s2}, Invalid);")
        else:
            print(f"impl_math!({s1}, {s2}, {m[n1+n2]});")
    print()
*/

impl_math!(NegFive, NegFive, Invalid);
impl_math!(NegFive, NegFour, Invalid);
impl_math!(NegFive, NegThree, Invalid);
impl_math!(NegFive, NegTwo, Invalid);
impl_math!(NegFive, NegOne, Invalid);
impl_math!(NegFive, Zero, NegFive);
impl_math!(NegFive, One, NegFour);
impl_math!(NegFive, Two, NegThree);
impl_math!(NegFive, Three, NegTwo);
impl_math!(NegFive, Four, NegOne);
impl_math!(NegFive, Five, Zero);
impl_math!(NegFive, Invalid, Invalid);

impl_math!(NegFour, NegFive, Invalid);
impl_math!(NegFour, NegFour, Invalid);
impl_math!(NegFour, NegThree, Invalid);
impl_math!(NegFour, NegTwo, Invalid);
impl_math!(NegFour, NegOne, NegFive);
impl_math!(NegFour, Zero, NegFour);
impl_math!(NegFour, One, NegThree);
impl_math!(NegFour, Two, NegTwo);
impl_math!(NegFour, Three, NegOne);
impl_math!(NegFour, Four, Zero);
impl_math!(NegFour, Five, One);
impl_math!(NegFour, Invalid, Invalid);

impl_math!(NegThree, NegFive, Invalid);
impl_math!(NegThree, NegFour, Invalid);
impl_math!(NegThree, NegThree, Invalid);
impl_math!(NegThree, NegTwo, NegFive);
impl_math!(NegThree, NegOne, NegFour);
impl_math!(NegThree, Zero, NegThree);
impl_math!(NegThree, One, NegTwo);
impl_math!(NegThree, Two, NegOne);
impl_math!(NegThree, Three, Zero);
impl_math!(NegThree, Four, One);
impl_math!(NegThree, Five, Two);
impl_math!(NegThree, Invalid, Invalid);

impl_math!(NegTwo, NegFive, Invalid);
impl_math!(NegTwo, NegFour, Invalid);
impl_math!(NegTwo, NegThree, NegFive);
impl_math!(NegTwo, NegTwo, NegFour);
impl_math!(NegTwo, NegOne, NegThree);
impl_math!(NegTwo, Zero, NegTwo);
impl_math!(NegTwo, One, NegOne);
impl_math!(NegTwo, Two, Zero);
impl_math!(NegTwo, Three, One);
impl_math!(NegTwo, Four, Two);
impl_math!(NegTwo, Five, Three);
impl_math!(NegTwo, Invalid, Invalid);

impl_math!(NegOne, NegFive, Invalid);
impl_math!(NegOne, NegFour, NegFive);
impl_math!(NegOne, NegThree, NegFour);
impl_math!(NegOne, NegTwo, NegThree);
impl_math!(NegOne, NegOne, NegTwo);
impl_math!(NegOne, Zero, NegOne);
impl_math!(NegOne, One, Zero);
impl_math!(NegOne, Two, One);
impl_math!(NegOne, Three, Two);
impl_math!(NegOne, Four, Three);
impl_math!(NegOne, Five, Four);
impl_math!(NegOne, Invalid, Invalid);

impl_math!(Zero, NegFive, NegFive);
impl_math!(Zero, NegFour, NegFour);
impl_math!(Zero, NegThree, NegThree);
impl_math!(Zero, NegTwo, NegTwo);
impl_math!(Zero, NegOne, NegOne);
impl_math!(Zero, Zero, Zero);
impl_math!(Zero, One, One);
impl_math!(Zero, Two, Two);
impl_math!(Zero, Three, Three);
impl_math!(Zero, Four, Four);
impl_math!(Zero, Five, Five);
impl_math!(Zero, Invalid, Invalid);

impl_math!(One, NegFive, NegFour);
impl_math!(One, NegFour, NegThree);
impl_math!(One, NegThree, NegTwo);
impl_math!(One, NegTwo, NegOne);
impl_math!(One, NegOne, Zero);
impl_math!(One, Zero, One);
impl_math!(One, One, Two);
impl_math!(One, Two, Three);
impl_math!(One, Three, Four);
impl_math!(One, Four, Five);
impl_math!(One, Five, Invalid);
impl_math!(One, Invalid, Invalid);

impl_math!(Two, NegFive, NegThree);
impl_math!(Two, NegFour, NegTwo);
impl_math!(Two, NegThree, NegOne);
impl_math!(Two, NegTwo, Zero);
impl_math!(Two, NegOne, One);
impl_math!(Two, Zero, Two);
impl_math!(Two, One, Three);
impl_math!(Two, Two, Four);
impl_math!(Two, Three, Five);
impl_math!(Two, Four, Invalid);
impl_math!(Two, Five, Invalid);
impl_math!(Two, Invalid, Invalid);

impl_math!(Three, NegFive, NegTwo);
impl_math!(Three, NegFour, NegOne);
impl_math!(Three, NegThree, Zero);
impl_math!(Three, NegTwo, One);
impl_math!(Three, NegOne, Two);
impl_math!(Three, Zero, Three);
impl_math!(Three, One, Four);
impl_math!(Three, Two, Five);
impl_math!(Three, Three, Invalid);
impl_math!(Three, Four, Invalid);
impl_math!(Three, Five, Invalid);
impl_math!(Three, Invalid, Invalid);

impl_math!(Four, NegFive, NegOne);
impl_math!(Four, NegFour, Zero);
impl_math!(Four, NegThree, One);
impl_math!(Four, NegTwo, Two);
impl_math!(Four, NegOne, Three);
impl_math!(Four, Zero, Four);
impl_math!(Four, One, Five);
impl_math!(Four, Two, Invalid);
impl_math!(Four, Three, Invalid);
impl_math!(Four, Four, Invalid);
impl_math!(Four, Five, Invalid);
impl_math!(Four, Invalid, Invalid);

impl_math!(Five, NegFive, Zero);
impl_math!(Five, NegFour, One);
impl_math!(Five, NegThree, Two);
impl_math!(Five, NegTwo, Three);
impl_math!(Five, NegOne, Four);
impl_math!(Five, Zero, Five);
impl_math!(Five, One, Invalid);
impl_math!(Five, Two, Invalid);
impl_math!(Five, Three, Invalid);
impl_math!(Five, Four, Invalid);
impl_math!(Five, Five, Invalid);
impl_math!(Five, Invalid, Invalid);

impl_math!(Invalid, NegFive, Invalid);
impl_math!(Invalid, NegFour, Invalid);
impl_math!(Invalid, NegThree, Invalid);
impl_math!(Invalid, NegTwo, Invalid);
impl_math!(Invalid, NegOne, Invalid);
impl_math!(Invalid, Zero, Invalid);
impl_math!(Invalid, One, Invalid);
impl_math!(Invalid, Two, Invalid);
impl_math!(Invalid, Three, Invalid);
impl_math!(Invalid, Four, Invalid);
impl_math!(Invalid, Five, Invalid);
impl_math!(Invalid, Invalid, Invalid);
