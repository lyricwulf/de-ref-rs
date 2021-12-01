extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

fn access_struct_inner(
    data: &syn::DataStruct,
) -> Result<(proc_macro2::TokenStream, &syn::Type, bool), proc_macro2::TokenStream> {
    if data.fields.len() != 1 {
        return Err(quote! {"Struct must have a single field to derive Deref"});
    }
    let field = data.fields.iter().next().unwrap();
    let inner_name = &field
        .ident
        .as_ref()
        .map(|n| quote! { #n })
        .unwrap_or(quote! { 0 });

    // Identical to the above but with support for unit struct
    // Can you even Deref as () ???
    // let inner_access = match data.fields {
    //     syn::Fields::Named(field) => {let},
    //     syn::Fields::Unnamed(field) => quote! { #field.0 },
    //     syn::Fields::Unit => return Err(quote! {"Struct unit field not supported yet"}),
    // };

    let (ty, add_ref) = match &field.ty {
        syn::Type::Reference(ty) => (ty.elem.as_ref(), false),
        _ => (&field.ty, true),
    };

    Ok((
        quote! {
            self.#inner_name
        },
        ty,
        add_ref,
    ))
}

fn access_enum_inner(
    data: &syn::DataEnum,
) -> Result<(proc_macro2::TokenStream, &syn::Type, bool), proc_macro2::TokenStream> {
    if data.variants.len() != 1 {
        return Err(quote! {"Enum must have a single variant to derive Deref"});
    }
    let variant = data.variants.iter().next().unwrap();
    let v_ident = &variant.ident;

    if variant.fields.len() != 1 {
        return Err(quote! {"Enum variant must have a single field to derive Deref"});
    }

    let field = variant.fields.iter().next().unwrap();
    let _inner_name = &field.ident;

    let (ty, add_ref) = match &field.ty {
        syn::Type::Reference(ty) => (ty.elem.as_ref(), false),
        _ => (&field.ty, false),
    };

    Ok((
        quote! {
            match self {
                Self::#v_ident(k) => k
            }
        },
        ty,
        add_ref,
    ))
}

fn access_inner(
    data: &syn::Data,
) -> Result<(proc_macro2::TokenStream, &syn::Type, bool), proc_macro2::TokenStream> {
    match data {
        syn::Data::Struct(data) => access_struct_inner(data),
        syn::Data::Enum(data) => access_enum_inner(data),
        _ => Err(quote! {"Only structs and enums can derive Deref"}),
    }
}

#[proc_macro_derive(Deref)]
pub fn deref(item: TokenStream) -> TokenStream {
    // Parse the macro input
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    // Access the inner type
    let (access_inner, ty, add_ref) = match access_inner(&input.data) {
        Ok(inner) => inner,
        Err(err) => return quote! { compile_error!(#err) }.into(),
    };

    let name = &input.ident;
    let gens = &input.generics;

    let (target_ref, inner_ref) = if add_ref {
        (quote! {}, quote! { & })
    } else {
        (quote! {}, quote! {})
    };

    let derived = quote! {
        impl #gens std::ops::Deref for #name #gens {
            type Target = #target_ref #ty;
            fn deref(&self) -> &Self::Target {
                #inner_ref #access_inner
            }
        }
    };
    derived.into()
}

#[proc_macro_derive(DerefMut)]
pub fn deref_mut(item: TokenStream) -> TokenStream {
    // Parse the macro input
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    // Access the inner type
    let (access_inner, _, add_ref) = match access_inner(&input.data) {
        Ok(inner) => inner,
        Err(err) => return quote! { compile_error!(#err) }.into(),
    };

    let inner_ref = if add_ref {
        quote! { &mut }
    } else {
        quote! {}
    };

    let name = &input.ident;
    let gens = &input.generics;

    let derived = quote! {
        impl #gens std::ops::DerefMut for #name #gens {
            fn deref_mut(&mut self) -> &mut Self::Target {
                #inner_ref #access_inner
            }
        }
    };
    derived.into()
}
