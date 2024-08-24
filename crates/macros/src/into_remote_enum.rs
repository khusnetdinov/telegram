use crate::helpers::enum_visitor::EnumVisitor;

pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ident = &input.ident;
    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => proc_macro_error::abort_call_site!("derive(FromRemoteEnum) expected enum"),
    };
    let variants: Vec<_> = enum_data
        .variants
        .iter()
        .map(|variant: &syn::Variant| EnumVisitor::new(variant).quote(Some(ident)))
        .collect();

    let quoted = quote::quote! {
        impl From<#ident> for Remote {
            fn from(value: #ident) -> Self {
                match value {
                   #(#variants,)*
                }
            }
        }
    };

    quoted.into()
}
