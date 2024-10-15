use indexmap::IndexMap as HashMap;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, DeriveInput, Ident, LitStr, Path,
    Token, TypeParam,
};

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
#[proc_macro]
pub fn impl_derived_conversions(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(ts as UnitToDerivedInput);
    let UnitToDerivedInput {
        name,
        generics,
        type_mappings,
    } = input;
    if type_mappings.is_empty() {
        panic!("Should have at least 1 type mapping! Ex: Angle: One, A;");
    }
    let mut hm: HashMap<Ident, (Ident, Ident, Ident)> = vec![
        ("Length", ("Zero", "None", "L")),
        ("Time", ("Zero", "None", "T")),
        ("Temp", ("Zero", "None", "TMP")),
        ("Mass", ("Zero", "None", "M")),
        ("Current", ("Zero", "None", "C")),
        ("LuminousIntensity", ("Zero", "None", "LI")),
        ("Quantity", ("Zero", "None", "Q")),
        ("Angle", ("Zero", "None", "A")),
        ("SolidAngle", ("Zero", "None", "S")),
        ("DigitalInformation", ("Zero", "None", "D")),
    ]
    .iter()
    .map(|(typ, (num, unit, gen_ident))| {
        let span = Span::call_site();
        let typ = Ident::new(typ, span);
        let num = Ident::new(num, span);
        let unit = Ident::new(unit, span);
        let gen_ident = Ident::new(gen_ident, span);
        (typ, (num, unit, gen_ident))
    })
    .collect();

    for map in type_mappings.iter() {
        let TypeMapping {
            typ,
            exp,
            unit_letter,
        } = map;
        if !hm.contains_key(typ) {
            panic!("Key `{}` is not a valid base unit", typ);
        }
        let gen_ident = hm.get(typ).expect("Just checked before").2.clone();
        hm.insert(typ.clone(), (exp.clone(), unit_letter.clone(), gen_ident));
    }

    let generics_idents = generics.iter().map(|generic| generic.ident.clone());
    // .collect::<Vec<_>>();
    let generics_idents = quote!(#(#generics_idents),*);
    let generics = generics.iter().map(|gen| {
        let Generics {
            ident,
            colon: _colon,
            trait_,
        } = gen;
        quote! {
            #ident: #trait_
        }
    });
    let generics = quote!(#(#generics),*);

    let mut exps = vec![];
    let mut units = vec![];

    for (typ, (exp, unit_letter, _)) in hm.iter() {
        let exp_ident = Ident::new(&format!("{typ}Exp"), Span::call_site());
        let unit_ident = Ident::new(&format!("{typ}Unit"), Span::call_site());
        exps.push(quote! {
            type #exp_ident = #exp;
        });
        units.push(quote! {
            type #unit_ident = #unit_letter;
        });
    }
    let mut d_to_u_associated = vec![];
    for (typ, (exp, _, gen_ident)) in hm.iter() {
        let exp_generic = Ident::new(&format!("{typ}Power"), Span::call_site());
        let line = quote! {
            #exp_generic<#exp, #gen_ident>
        };
        d_to_u_associated.push(line);
    }

    let exps = quote!(#(#exps)*);
    let units = quote!(#(#units)*);

    quote! {
        impl<
                T: TimeUnit,
                L: LengthUnit,
                TMP: TempUnit,
                M: MassUnit,
                C: CurrentUnit,
                LI: LuminousIntensityUnit,
                Q: QuantityUnit,
                A: AngleUnit,
                S: SolidAngleUnit,
                D: DigitalInformationUnit,
            > DerivedToUnit
            for Derived<
            #(#d_to_u_associated),*
            >
        where
            #name<#generics_idents>: crate::specialization::Specialized<Bool = crate::specialization::False>,
        {
            type Output = #name<#generics_idents>;

            fn to_unit(self) -> Self::Output {
                #name {
                    inner: self.inner,
                    types: PhantomData,
                }
            }
        }

        impl<#generics> UnitToDerived for #name <#generics_idents> {
            #exps
            #units
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
            > {
                Derived {
                    inner: self.inner,
                    types: std::marker::PhantomData,
                }
            }
        }
    }
    .into()
}

/*

impl_unit_to_derived! {
    Angle<A>,

}

*/

#[derive(Clone)]
struct UnitToDerivedInput {
    name: Ident,
    generics: Punctuated<Generics, Token![,]>,
    type_mappings: Punctuated<TypeMapping, Token![;]>,
}

impl Parse for UnitToDerivedInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _larrow: Token![<] = input.parse()?;
        let generics = Punctuated::<Generics, Token![,]>::parse_separated_nonempty(input)?;
        let _rarrow: Token![>] = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let type_mappings = Punctuated::<TypeMapping, Token![;]>::parse_terminated(input)?;
        Ok(UnitToDerivedInput {
            name,
            generics,
            type_mappings,
        })
    }
}

#[derive(Clone, Debug)]
struct TypeMapping {
    typ: Ident,
    exp: Ident,
    unit_letter: Ident,
}

impl Parse for TypeMapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let typ = input.parse()?;
        let _colon: Token![:] = input.parse()?;
        let exp = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let unit_letter = input.parse()?;
        Ok(TypeMapping {
            typ,
            exp,
            unit_letter,
        })
    }
}

#[derive(Clone)]
struct Generics {
    ident: Ident,
    colon: Token![:],
    trait_: Ident,
}

impl Parse for Generics {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Generics {
            ident: input.parse()?,
            colon: input.parse()?,
            trait_: input.parse()?,
        })
    }
}
