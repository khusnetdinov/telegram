pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, .. } = syn::parse_macro_input!(input);

    let quote = quote::quote! {
        impl std::ops::Deref for #ident {
            type Target = Inner;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };

    quote.into()
}
