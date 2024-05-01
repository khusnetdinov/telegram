#[proc_macro_derive(DerefInner)]
pub fn deref_inner_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

#[proc_macro_derive(FromInner)]
pub fn from_inner_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
