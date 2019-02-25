#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, Data, DataEnum, DeriveInput, Field,
    Fields, FieldsNamed, FieldsUnnamed, GenericParam, Ident, Type, TypeParam,
};

fn replace<A: PartialEq, P>(list: &mut Punctuated<A, P>, target: &A, value: A) {
    for t in list {
        if target == t {
            *t = value;
            return;
        }
    }
    panic!("failed to substitute");
}

#[proc_macro_derive(Lift)]
pub fn derive_lift(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let generic_type = input
        .generics
        .type_params()
        .next()
        .expect("can't Lift a type without type parameters");
    let generic_type: GenericParam = generic_type.clone().into();

    let mut impl_vars_2 = input.generics.params.clone();
    impl_vars_2.push(parse_quote!(LiftTarget1));

    let mut impl_vars_3 = impl_vars_2.clone();
    impl_vars_3.push(parse_quote!(LiftTarget2));

    let mut target_vars_2 = input.generics.params.clone();
    replace(&mut target_vars_2, &generic_type, parse_quote!(LiftTarget1));

    let mut target_vars_3 = input.generics.params.clone();
    replace(&mut target_vars_3, &generic_type, parse_quote!(LiftTarget2));

    let (_, type_generics, where_clause) = input.generics.split_for_impl();

    let out = quote! {
        impl<#impl_vars_2> ::higher::Lift<#generic_type, LiftTarget1>
        for #name #type_generics #where_clause {
            type Target1 = #name<#target_vars_2>;
        }

        impl<#impl_vars_3> ::higher::Lift3<#generic_type, LiftTarget2, LiftTarget1>
        for #name #type_generics #where_clause {
            type Target2 = #name<#target_vars_3>;
        }
    };
    proc_macro::TokenStream::from(out)
}

#[proc_macro_derive(Bilift)]
pub fn derive_bilift(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut types = input.generics.type_params();
    let left_type = types
        .next()
        .expect("can't Bilift a type without type parameters");
    let left_type: GenericParam = left_type.clone().into();
    let right_type = types
        .next()
        .expect("can't Bilift a type with less than two type parameters");
    let right_type: GenericParam = right_type.clone().into();

    let mut impl_vars = input.generics.params.clone();
    impl_vars.push(parse_quote!(LiftTarget1));
    impl_vars.push(parse_quote!(LiftTarget2));

    let mut target_vars = input.generics.params.clone();
    replace(&mut target_vars, &left_type, parse_quote!(LiftTarget1));
    replace(&mut target_vars, &right_type, parse_quote!(LiftTarget2));

    let (_, type_generics, where_clause) = input.generics.split_for_impl();

    let out = quote! {
        impl<#impl_vars> ::higher::Bilift<#left_type, #right_type, LiftTarget1, LiftTarget2>
        for #name #type_generics #where_clause {
            type Target = #name<#target_vars>;
        }
    };
    proc_macro::TokenStream::from(out)
}

fn match_type_param(param: &TypeParam, ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        if let Some(segment) = path.path.segments.iter().next() {
            if segment.ident == param.ident {
                return true;
            }
        }
    }
    false
}

fn filter_fields<'a, P, F1, F2>(
    fields: &'a Punctuated<Field, P>,
    ty: &TypeParam,
    transform: F1,
    copy: F2,
) -> Vec<TokenStream>
where
    F1: Fn(&Ident) -> TokenStream,
    F2: Fn(&Ident) -> TokenStream,
{
    fields
        .iter()
        .map(|field| {
            if match_type_param(ty, &field.ty) {
                transform(&field.ident.clone().unwrap())
            } else {
                copy(&field.ident.clone().unwrap())
            }
        })
        .collect()
}

#[proc_macro_derive(Functor)]
pub fn derive_functor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let type_params = &input.generics.params;
    let where_clause = &input.generics.where_clause;
    let generic_type = input
        .generics
        .type_params()
        .next()
        .expect("can't derive Functor for a type without type parameters");
    let map_impl = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => derive_functor_named_struct(name, fields, &generic_type),
            Fields::Unnamed(fields) => derive_functor_unnamed_struct(name, fields, &generic_type),
            Fields::Unit => panic!("can't derive Functor for an empty struct"),
        },
        Data::Enum(data) => derive_functor_enum(name, data, &generic_type),
        Data::Union(_) => panic!("can't derive Functor for a union type"),
    };
    quote!(
        impl<#type_params, TargetType> ::higher::Functor<#generic_type, TargetType> for #name<#type_params> #where_clause {
            fn map<F>(self, f: F) -> <Self as ::higher::Lift<#generic_type, TargetType>>::Target1
            where
                F: Fn(#generic_type) -> TargetType
            {
                #map_impl
            }
        }
    ).into()
}

fn derive_functor_named_struct(
    name: &Ident,
    fields: &FieldsNamed,
    generic_type: &TypeParam,
) -> TokenStream {
    let apply_fields = filter_fields(
        &fields.named,
        generic_type,
        |field| {
            quote! {
                #field: f(self.#field),
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
    generic_type: &TypeParam,
) -> TokenStream {
    let fields = fields.unnamed.iter().enumerate().map(|(index, field)| {
        if match_type_param(generic_type, &field.ty) {
            quote! { f(self.#index), }
        } else {
            quote! { self.#index, }
        }
    });
    quote! { #name(#(#fields)*) }
}

fn derive_functor_enum(name: &Ident, data: &DataEnum, generic_type: &TypeParam) -> TokenStream {
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
                            if match_type_param(generic_type, &field.ty) {
                                quote! { #name: f(#arg) }
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
                    if match_type_param(generic_type, &field.ty) {
                        quote! { f(#arg) }
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
