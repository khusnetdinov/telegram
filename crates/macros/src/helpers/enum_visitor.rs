#[derive(Debug)]
pub struct EnumVisitor {
    variant: syn::Variant,
}

impl EnumVisitor {
    pub fn new(variant: &syn::Variant) -> Self {
        Self {
            variant: variant.to_owned(),
        }
    }

    fn is_build_in(&self) -> bool {
        let build_in: [&str; 11] = [
            "String", "&str", "i64", "i32", "i16", "i8", "u64", "u32", "u16", "u8", "bool",
        ];
        let mut ty = String::new();

        if let syn::Variant {
            fields: syn::Fields::Unnamed(ref filed_unnamed),
            ..
        } = &self.variant
        {
            if let syn::Type::Path(ref ty_path) = filed_unnamed.unnamed[0].ty {
                ty = ty_path.path.segments[0].ident.to_string()
            };
        };

        build_in.contains(&ty.as_str())
    }

    fn is_boxed(&self) -> bool {
        if let syn::Variant {
            fields: syn::Fields::Unnamed(ref filed_unnamed),
            ..
        } = &self.variant
        {
            if let syn::Type::Path(ref ty_path) = filed_unnamed.unnamed[0].ty {
                return ty_path.path.segments[0].ident.to_string().as_str() == "Box";
            }
        }

        false
    }

    pub fn quote(&self, ident: Option<&proc_macro2::Ident>) -> proc_macro2::TokenStream {
        let variant_ident = &self.variant.ident;
        let enum_ident = if ident.is_some() {
            quote::quote! { #ident }
        } else {
            quote::quote! { Remote }
        };

        let variant_into = if self.is_build_in() {
            quote::quote! { variant }
        } else if self.is_boxed() {
            quote::quote! { Box::new((*variant).into()) }
        } else {
            quote::quote! { variant.into() }
        };

        quote::quote! {
            #enum_ident::#variant_ident(variant) => Self::#variant_ident(#variant_into)
        }
    }
}
