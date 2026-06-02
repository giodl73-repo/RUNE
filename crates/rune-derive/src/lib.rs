use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(RuneContract, attributes(rune))]
pub fn derive_rune_contract(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_span = input.ident.span();
    let ident = input.ident;
    let rust_type = ident.to_string();
    let mut id = None;
    let mut version = None;
    let mut kind = quote! { ::rune_core::ContractKind::Entity };
    let mut trace_links = Vec::new();

    for attr in input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("rune"))
    {
        let parse_result = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                id = Some(meta.value()?.parse::<LitStr>()?.value());
            } else if meta.path.is_ident("version") {
                version = Some(meta.value()?.parse::<LitStr>()?.value());
            } else if meta.path.is_ident("kind") {
                let value = meta.value()?.parse::<LitStr>()?.value();
                kind = match value.as_str() {
                    "entity" => quote! { ::rune_core::ContractKind::Entity },
                    "event" => quote! { ::rune_core::ContractKind::Event },
                    "command" => quote! { ::rune_core::ContractKind::Command },
                    "state" => quote! { ::rune_core::ContractKind::State },
                    "artifact" => quote! { ::rune_core::ContractKind::Artifact },
                    "evidence" => quote! { ::rune_core::ContractKind::Evidence },
                    "source" => quote! { ::rune_core::ContractKind::Source },
                    other => {
                        let other = LitStr::new(other, meta.path.span());
                        quote! { ::rune_core::ContractKind::Other(#other) }
                    }
                };
            } else if meta.path.is_ident("requirement") {
                let target = meta.value()?.parse::<LitStr>()?.value();
                trace_links.push(quote! {
                    ::rune_core::TraceLink {
                        relation: "requirement",
                        target: #target,
                    }
                });
            } else {
                return Err(meta.error("unsupported rune attribute"));
            }
            Ok(())
        });
        if let Err(error) = parse_result {
            return error.to_compile_error().into();
        }
    }

    let Some(id) = id else {
        return syn::Error::new(input_span, "missing required rune attribute: id")
            .to_compile_error()
            .into();
    };
    let Some(version) = version else {
        return syn::Error::new(input_span, "missing required rune attribute: version")
            .to_compile_error()
            .into();
    };

    let fields = match input.data {
        Data::Struct(data) => field_descriptors(data.fields),
        Data::Enum(_) | Data::Union(_) => Vec::new(),
    };

    quote! {
        impl ::rune_core::RuneContract for #ident {
            fn descriptor() -> ::rune_core::ContractDescriptor {
                ::rune_core::ContractDescriptor {
                    id: #id,
                    version: #version,
                    rust_type: #rust_type,
                    kind: #kind,
                    fields: &[#(#fields),*],
                    invariants: &[],
                    trace_links: &[#(#trace_links),*],
                    extensions: &[],
                }
            }
        }
    }
    .into()
}

fn field_descriptors(fields: Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        Fields::Named(named) => named
            .named
            .into_iter()
            .filter_map(|field| {
                let name = field.ident.clone()?.to_string();
                let ty = field.ty;
                let rust_type = quote! { #ty }.to_string();
                Some(quote! {
                    ::rune_core::FieldDescriptor {
                        name: #name,
                        rust_type: #rust_type,
                    }
                })
            })
            .collect(),
        Fields::Unnamed(_) | Fields::Unit => Vec::new(),
    }
}
