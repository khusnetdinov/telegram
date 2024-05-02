pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, .. } = syn::parse_macro_input!(input);

    let quote = quote::quote! {
        impl From<Inner> for #ident {
            fn from(inner: Inner) -> Self {
                Self { inner }
            }
        }
    };

    quote.into()
}
