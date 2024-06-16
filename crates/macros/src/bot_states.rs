pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _input = syn::parse_macro_input!(input as syn::DeriveInput);

    let quote = quote::quote! {};

    quote.into()
}
