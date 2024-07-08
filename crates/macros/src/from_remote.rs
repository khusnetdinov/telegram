pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => proc_macro_error::abort_call_site!("derive(FromRemote) expected struct"),
    };

    let named = match &struct_data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => proc_macro_error::abort_call_site!("derive(FromRemote) expected struct fields"),
    };

    let setters = named.iter().map(|field| {
        let name = &field.ident;

        quote::quote! {
            #name: remote.#name
        }
    });

    let ident = &input.ident;
    let quote = quote::quote! {
        impl From<Remote> for #ident {
            fn from(remote: Remote) -> Self {
                Self {
                   #(#setters,)*
                }
            }
        }
    };

    quote.into()
}
