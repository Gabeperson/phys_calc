mod private {
    pub trait Sealed {}
}
pub trait Specialized {
    type Bool: Boolean;
}
use private::Sealed;
pub trait Boolean: Sealed {}
#[derive(Debug, Clone, Copy)]
pub struct True;
#[derive(Debug, Clone, Copy)]
pub struct False;
impl Sealed for True {}
impl Boolean for True {}
impl Sealed for False {}
impl Boolean for False {}

#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! unspecialize {
    ($ident:ident<$($generic:ident:$trait:ident),+>) => {
        impl<$($generic: $trait),+> crate::specialization::Specialized for $ident<$($generic),+> {
            type Bool = crate::specialization::False;
        }
    };
}

#[macro_export]
macro_rules! specialize {
    ($ident:ident<$($generic:ident),+>) => {
        impl $crate::specialization::Specialized for $ident<$($generic),+> {
            type Bool = True;
        }
    };
}
