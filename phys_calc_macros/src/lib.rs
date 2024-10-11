use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, DeriveInput, Ident, LitStr, Path, Token, TypeParam};

#[proc_macro_derive(Unit, attributes(multiplier, unit, unit_impl))]
pub fn derive_unit(ts: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(ts).expect("Invalid proc macro input for derive!");
    let ident = input.ident;
    let generics = input.generics;
    let attrs = input.attrs;

    let generics_idents = generics
        .params
        .iter()
        .map(|p| match p {
            syn::GenericParam::Type(t) => t.ident.clone(),
            _ => unimplemented!(),
        })
        .collect::<Punctuated<_, Token![,]>>();

    let mult = if let Some(mult) = attrs.iter().find(|attr| attr.path().is_ident("multiplier")) {
        match mult.meta.clone() {
            syn::Meta::List(mult) => mult.tokens,

            _ => panic!("Expected #[multiplier(<multiplier>)]"),
        }
    } else {
        panic!("Expected `multiplier` attribute!")
    };

    let unit = if let Some(unit) = attrs.iter().find(|attr| attr.path().is_ident("unit")) {
        match unit.meta.clone() {
            syn::Meta::List(marker) => {
                if let Ok(lit) = syn::parse2::<LitStr>(marker.tokens) {
                    lit
                } else {
                    panic!("Expected string literal as unit!")
                }
            }
            _ => panic!("Expected #[unit(<unit>)]"),
        }
    } else {
        panic!("Expected `unit` attribute!")
    };

    let marker = if let Some(marker) = attrs.iter().find(|attr| attr.path().is_ident("unit_impl")) {
        match marker.meta.clone() {
            syn::Meta::List(marker) => {
                if let Ok(lit) = syn::parse2::<Path>(marker.tokens) {
                    lit
                } else {
                    panic!("Expected trait path for marker trait!")
                }
            }
            _ => panic!("Expected #[marker(<MarkerTrait>)]"),
        }
    } else {
        panic!("Expected `marker` attribute!");
    };

    let mut generics_with_rhs = generics.clone();
    generics_with_rhs
        .params
        .push(syn::GenericParam::Type(TypeParam::from(Ident::new(
            "Rhs",
            Span::call_site(),
        ))));

    quote! {
        impl crate::Unit for #ident {}
        impl #generics #marker for #ident #generics_idents {
            fn mult() -> number {
                (#mult) as f64
            }
            fn unit() -> &'static str {
                #unit
            }
        }
        impl #generics crate::math::EqualsOrZero for #ident #generics_idents {
            type SelfType = Self;
        }
        impl #generics crate::math::EqualsOrZero<None> for #ident #generics_idents {
            type SelfType = Self;
        }
        impl #generics_with_rhs std::ops::Mul<Rhs> for #ident #generics_idents
        where Self: MulHelper<Rhs>
        {
            type Output = <Self as MulHelper<Rhs>>::Output;
            fn mul(self, rhs: Rhs) -> Self::Output {
                self.multiply(rhs)
            }
        }
        impl #generics_with_rhs std::ops::Div<Rhs> for #ident #generics_idents
        where Self: DivHelper<Rhs>
        {
            type Output = <Self as DivHelper<Rhs>>::Output;
            fn div(self, rhs: Rhs) -> Self::Output {
                self.divide(rhs)
            }
        }


    }
    .into()
}
