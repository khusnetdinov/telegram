#[derive(Debug)]
#[allow(dead_code)]
struct StructFieldVisitor {
    ident: proc_macro2::Ident,
    structs: Vec<String>,
}

impl StructFieldVisitor {
    fn angle_bracketed_visitor(angle_bracketed: &syn::PathArguments) -> Vec<String> {
        let mut types: Vec<String> = Vec::new();

        if let syn::PathArguments::AngleBracketed(ref angle_bracketed) = angle_bracketed {
            if let syn::GenericArgument::Type(syn::Type::Path(ref ty_path)) =
                angle_bracketed.args[0]
            {
                types.push(ty_path.path.segments[0].ident.to_string());
                types.append(&mut Self::angle_bracketed_visitor(
                    &ty_path.path.segments[0].arguments,
                ));
            }
        }

        types
    }

    fn visitor(field: &syn::Field) -> Vec<String> {
        let mut types: Vec<String> = Vec::new();

        if let syn::Type::Path(ref ty_path) = field.ty {
            types.push(ty_path.path.segments[0].ident.to_string());
            types.append(&mut Self::angle_bracketed_visitor(
                &ty_path.path.segments[0].arguments,
            ));
        }

        types
    }

    pub fn new(field: &syn::Field) -> Self {
        Self {
            ident: field.clone().ident.unwrap(),
            structs: Self::visitor(field),
        }
    }

    fn is_build_in(&self) -> bool {
        let build_in: [&str; 11] = [
            "String", "&str", "i64", "i32", "i16", "i8", "u64", "u32", "u16", "u8", "bool",
        ];

        build_in.contains(&self.structs.last().unwrap().as_str())
    }

    fn is_boxed(&self) -> bool {
        self.structs.contains(&String::from("Box"))
    }

    pub fn quoted(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        if self.is_build_in() {
            quote::quote! { #ident: remote.#ident }
        } else {
            let quoted = self
                .structs
                .iter()
                .rev()
                .fold(quote::quote! {}, |quoted, ty| {
                    if self.structs.len() == 1 {
                        quote::quote! { into() }
                    } else {
                        match ty.as_str() {
                            "Option" => {
                                if self.structs.len() == 2 {
                                    quote::quote! { map(|inner| #quoted) }
                                } else {
                                    quote::quote! { map(|option| option.#quoted) }
                                }
                            }
                            "Vec" => quote::quote! { iter().map(|inner| #quoted ).collect() },
                            "Box" => quote::quote! { #quoted },
                            _ => {
                                if self.is_boxed() {
                                    quote::quote! { map(|inner| Box::new((*inner).into())) }
                                } else {
                                    quote::quote! { inner.to_owned().into() }
                                }
                            }
                        }
                    }
                });

            quote::quote! {
                #ident: remote.#ident.#quoted
            }
        }
    }
}

pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => proc_macro_error::abort_call_site!("derive(FromRemoteStruct) expected struct"),
    };

    let named = match &struct_data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => proc_macro_error::abort_call_site!("derive(FromRemoteStruct) expected struct fields"),
    };

    let setters = named
        .iter()
        .map(|field| StructFieldVisitor::new(field).quoted());

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
