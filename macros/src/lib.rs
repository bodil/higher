#![recursion_limit = "256"]

//! Custom derives for the [`higher`][higher] crate.
//!
//! Please see the relevant crate for documentation.
//!
//! [higher]: https://docs.rs/crate/higher

extern crate proc_macro;

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DataEnum,
    DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Ident, Index, Type,
    TypeParam,
};

fn type_params_replace(
    input_params: &Punctuated<GenericParam, Comma>,
    replace: &TypeParam,
    with: Ident,
) -> Punctuated<GenericParam, Comma> {
    let mut output = input_params.clone();
    for param in output.iter_mut() {
        match param {
            GenericParam::Type(ref mut type_param) if type_param == replace => {
                *(&mut type_param.ident) = with;
                break;
            }
            _ => {}
        }
    }
    output
}

fn report_error(span: Span, msg: &str) -> proc_macro::TokenStream {
    (quote_spanned! {span => compile_error! {#msg}}).into()
}

fn decide_functor_generic_type<'a>(
    input: &'a DeriveInput,
) -> Result<&'a TypeParam, proc_macro::TokenStream> {
    let mut generics_iter = input.generics.type_params();
    let generic_type = match generics_iter.next() {
        Some(t) => t,
        None => {
            return Err(report_error(
                input.ident.span(),
                "can't derive Functor for a type without type parameters",
            ));
        }
    };

    if let Some(next_type_param) = generics_iter.next() {
        return Err(report_error(
            next_type_param.span(),
            "can't derive Functor for a type with multiple type parameters; did you mean Bifunctor?",
        ));
    }

    return Ok(generic_type);
}

fn decide_bifunctor_generic_types<'a>(
    input: &'a DeriveInput,
) -> Result<(&'a TypeParam, &'a TypeParam), proc_macro::TokenStream> {
    let mut generics_iter = input.generics.type_params();
    let generic_type_a = match generics_iter.next() {
        Some(t) => t,
        None => {
            return Err(report_error(
                input.ident.span(),
                "can't derive Bifunctor for a type without type parameters",
            ))
        }
    };

    let generic_type_b = match generics_iter.next() {
        Some(t) => t,
        None => return Err(report_error(
            input.ident.span(),
            "can't derive Bifunctor for a type with only one type parameter; did you mean Functor?",
        )),
    };

    if let Some(next_type_param) = generics_iter.next() {
        return Err(report_error(
            next_type_param.span(),
            "can't derive Functor for a type with three or more type parameters",
        ));
    }

    return Ok((generic_type_a, generic_type_b));
}

#[proc_macro_derive(Bifunctor)]
pub fn derive_bifunctor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let type_params = &input.generics.params;
    let where_clause = &input.generics.where_clause;

    let (generic_type_a, generic_type_b) = match decide_bifunctor_generic_types(&input) {
        Ok(t) => t,
        Err(err) => return err,
    };

    let type_map = HashMap::from([
        (
            generic_type_a.ident.clone(),
            Ident::new("left", Span::call_site()),
        ),
        (
            generic_type_b.ident.clone(),
            Ident::new("right", Span::call_site()),
        ),
    ]);

    let bimap_impl = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => derive_functor_named_struct(name, fields, &type_map),
            Fields::Unnamed(fields) => derive_functor_unnamed_struct(name, fields, &type_map),
            Fields::Unit => {
                return report_error(
                    input.ident.span(),
                    "can't derive Bifunctor for an empty struct",
                );
            }
        },
        Data::Enum(data) => derive_functor_enum(name, data, &type_map),
        Data::Union(_) => {
            return report_error(
                input.ident.span(),
                "can't derive Bifunctor for a union type",
            );
        }
    };

    let type_params_generic = type_params_replace(
        &type_params_replace(
            type_params,
            generic_type_a,
            Ident::new("DerivedTargetTypeA", Span::call_site()),
        ),
        generic_type_b,
        Ident::new("DerivedTargetTypeB", Span::call_site()),
    );

    quote!(
        impl<#type_params> ::higher::Bifunctor<#generic_type_a, #generic_type_b> for #name<#type_params> #where_clause {
            type Target<DerivedTargetTypeA, DerivedTargetTypeB> = #name<#type_params_generic>;
            fn bimap<DerivedTypeA, DerivedTypeB, L, R>(self, left: L, right: R) -> Self::Target<DerivedTypeA, DerivedTypeB>
            where
                L: Fn(#generic_type_a) -> DerivedTypeA,
                R: Fn(#generic_type_b) -> DerivedTypeB
            {
                #bimap_impl
            }
        }
    )
    .into()
}

#[proc_macro_derive(Functor)]
pub fn derive_functor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let type_params = &input.generics.params;
    let where_clause = &input.generics.where_clause;

    let generic_type = match decide_functor_generic_type(&input) {
        Ok(t) => t,
        Err(err) => return err,
    };

    let type_map = HashMap::from([(
        generic_type.ident.clone(),
        Ident::new("f", Span::call_site()),
    )]);

    let fmap_impl = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => derive_functor_named_struct(name, fields, &type_map),
            Fields::Unnamed(fields) => derive_functor_unnamed_struct(name, fields, &type_map),
            Fields::Unit => {
                return report_error(
                    input.ident.span(),
                    "can't derive Functor for an empty struct",
                );
            }
        },
        Data::Enum(data) => derive_functor_enum(name, data, &type_map),
        Data::Union(_) => {
            return report_error(input.ident.span(), "can't derive Functor for a union type");
        }
    };

    let type_params_with_t = type_params_replace(
        type_params,
        generic_type,
        Ident::new("DerivedTargetType", Span::call_site()),
    );

    quote!(
        impl<#type_params> ::higher::Functor<#generic_type> for #name<#type_params> #where_clause {
            type Target<DerivedTargetType> = #name<#type_params_with_t>;
            fn fmap<DerivedType, F>(self, f: F) -> Self::Target<DerivedType>
            where
                F: Fn(#generic_type) -> DerivedType
            {
                #fmap_impl
            }
        }
    )
    .into()
}

fn match_type_param<'a>(params: &'a HashMap<Ident, Ident>, ty: &Type) -> Option<&'a Ident> {
    if let Type::Path(path) = ty {
        if let Some(segment) = path.path.segments.iter().next() {
            return params.get(&segment.ident);
        }
    }
    None
}

fn filter_fields<'a, P, F1, F2>(
    fields: &'a Punctuated<Field, P>,
    ty: &HashMap<Ident, Ident>,
    transform: F1,
    copy: F2,
) -> Vec<TokenStream>
where
    F1: Fn(&Ident, &Ident) -> TokenStream,
    F2: Fn(&Ident) -> TokenStream,
{
    fields
        .iter()
        .map(|field| {
            if let Some(f) = match_type_param(ty, &field.ty) {
                transform(&field.ident.clone().unwrap(), f)
            } else {
                copy(&field.ident.clone().unwrap())
            }
        })
        .collect()
}

fn derive_functor_named_struct(
    name: &Ident,
    fields: &FieldsNamed,
    generic_types: &HashMap<Ident, Ident>,
) -> TokenStream {
    let apply_fields = filter_fields(
        &fields.named,
        generic_types,
        |field, function_name| {
            quote! {
                #field: #function_name(self.#field),
            }
        },
        |field| {
            quote! {
                #field: self.#field,
            }
        },
    )
    .into_iter();
    quote! {
        #name {
            #(#apply_fields)*
        }
    }
}

fn derive_functor_unnamed_struct(
    name: &Ident,
    fields: &FieldsUnnamed,
    generic_types: &HashMap<Ident, Ident>,
) -> TokenStream {
    let fields = fields.unnamed.iter().enumerate().map(|(index, field)| {
        let index = Index::from(index);
        if let Some(function_name) = match_type_param(generic_types, &field.ty) {
            quote! { #function_name(self.#index), }
        } else {
            quote! { self.#index, }
        }
    });
    quote! { #name(#(#fields)*) }
}

fn derive_functor_enum(
    name: &Ident,
    data: &DataEnum,
    generic_types: &HashMap<Ident, Ident>,
) -> TokenStream {
    let variants = data.variants.iter().map(|variant| {
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let args: Vec<Ident> = fields
                    .named
                    .iter()
                    .map(|field| {
                        Ident::new(
                            &format!("arg_{}", field.ident.clone().unwrap()),
                            field.ident.clone().unwrap().span(),
                        )
                    })
                    .collect();
                let apply =
                    fields
                        .named
                        .iter()
                        .zip(args.clone().into_iter())
                        .map(|(field, arg)| {
                            let name = &field.ident;
                            if let Some(function_name) = match_type_param(generic_types, &field.ty)
                            {
                                quote! { #name: #function_name(#arg) }
                            } else {
                                quote! { #name: #arg }
                            }
                        });
                let args = fields
                    .named
                    .iter()
                    .zip(args.into_iter())
                    .map(|(field, arg)| {
                        let name = &field.ident;
                        quote! { #name:#arg }
                    });
                quote! {
                    #name::#ident { #(#args,)* } => #name::#ident { #(#apply,)* },
                }
            }
            Fields::Unnamed(fields) => {
                let args: Vec<Ident> = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, _)| Ident::new(&format!("arg{}", index), Span::call_site()))
                    .collect();
                let fields = fields.unnamed.iter().zip(args.iter()).map(|(field, arg)| {
                    if let Some(function_name) = match_type_param(generic_types, &field.ty) {
                        quote! { #function_name(#arg) }
                    } else {
                        quote! { #arg }
                    }
                });
                let args = args.iter();
                quote! {
                    #name::#ident(#(#args,)*) => #name::#ident(#(#fields,)*),
                }
            }
            Fields::Unit => quote! {
                #name::#ident => #name::#ident,
            },
        }
    });
    quote! {
        match self {
            #(#variants)*
        }
    }
}

#[cfg(test)]
mod test {
    use higher::{Bifunctor, Functor};

    #[derive(PartialEq, Eq, Debug, Functor)]
    struct FunctorNamed<A> {
        named: A,
    }

    #[derive(PartialEq, Eq, Debug, Functor)]
    struct FunctorUnnamed<A>(A);

    #[derive(PartialEq, Eq, Debug, Functor)]
    #[allow(dead_code)]
    enum FunctorEnum<A> {
        Some(A),
        SomeNumber(usize),
        SomeOther(A),
        None,
    }

    #[test]
    fn derive_functor() {
        assert_eq!(
            (FunctorNamed { named: 2u32 }).fmap(|x| x + 3),
            FunctorNamed { named: 5u32 }
        );

        assert_eq!(FunctorUnnamed(2u32).fmap(|x| x + 3), FunctorUnnamed(5u32));

        assert_eq!(
            FunctorEnum::Some(2u32).fmap(|x| x + 3),
            FunctorEnum::Some(5u32)
        );
        assert_eq!(
            FunctorEnum::<u32>::SomeNumber(2).fmap(|x| x + 3),
            FunctorEnum::<u32>::SomeNumber(2)
        );
        assert_eq!(
            FunctorEnum::SomeOther(2u32).fmap(|x| x + 3),
            FunctorEnum::SomeOther(5u32)
        );
        assert_eq!(FunctorEnum::<u32>::None.fmap(|x| x + 3), FunctorEnum::None);
    }

    #[derive(PartialEq, Eq, Debug, Bifunctor)]
    struct BifunctorNamed<A, B> {
        a: A,
        b: B,
    }

    #[derive(PartialEq, Eq, Debug, Bifunctor)]
    struct BifunctorUnnamed<A, B>(A, B);

    #[derive(PartialEq, Eq, Debug, Bifunctor)]
    #[allow(dead_code)]
    enum BifunctorEnum<A, B> {
        Ok(A),
        Err(B),
        Number(usize),
        Nothing,
    }

    #[test]
    fn derive_bifunctor() {
        assert_eq!(
            (BifunctorNamed { a: 2u32, b: 2u8 }).bimap(|x| x + 3, |x| x + 4),
            BifunctorNamed { a: 5u32, b: 6u8 }
        );

        assert_eq!(
            BifunctorUnnamed(2u32, 2u8).bimap(|x| x + 3, |x| x + 4),
            BifunctorUnnamed(5u32, 6u8)
        );

        assert_eq!(
            BifunctorEnum::<u32, u8>::Ok(2u32).bimap(|x| x + 3, |x| x + 4),
            BifunctorEnum::Ok(5u32)
        );
        assert_eq!(
            BifunctorEnum::<u32, u8>::Err(2u8).bimap(|x| x + 3, |x| x + 4),
            BifunctorEnum::Err(6u8)
        );
        assert_eq!(
            BifunctorEnum::<u32, u8>::Number(2).bimap(|x| x + 3, |x| x + 4),
            BifunctorEnum::Number(2)
        );
        assert_eq!(
            BifunctorEnum::<u32, u8>::Nothing.bimap(|x| x + 3, |x| x + 4),
            BifunctorEnum::Nothing
        );
    }
}