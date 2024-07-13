#![allow(clippy::collapsible_match)]

const COMMAND_ATTRIBUTE_IDENT_NAME: &str = "command";
const DESCRIPTION_CHARS_MAX_COUNT: u16 = 256;

type PunctuatedAttributes = syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>;

fn parse_commands(enum_data: &syn::DataEnum) -> Vec<(String, String)> {
    let mut commands = Vec::new();

    enum_data.variants.iter().for_each(|variant: &syn::Variant| {
        let syn::Variant { ref attrs, .. } = variant;

        for attr in attrs {
            if attr.path().is_ident(COMMAND_ATTRIBUTE_IDENT_NAME) {
                attr.parse_nested_meta(|meta| {
                    let parse_stream = meta.value().unwrap();
                    let lit_str = parse_stream.parse::<syn::LitStr>().unwrap();
                    let description = lit_str.value();

                    if description.len() > DESCRIPTION_CHARS_MAX_COUNT as usize {
                        proc_macro_error::abort_call_site!(
                            format!("command description value length must be less than {DESCRIPTION_CHARS_MAX_COUNT}")
                        )
                    }

                    commands.push((variant.ident.to_string(), description.to_string(), ));

                    Ok(())
                }).unwrap();
            }
        }
    });

    commands
}

fn parse_nested(attrs: &[syn::Attribute]) -> Option<PunctuatedAttributes> {
    let mut nested = None;

    for attr in attrs {
        if attr.path().is_ident(COMMAND_ATTRIBUTE_IDENT_NAME) {
            nested = Some(
                attr.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .ok()?,
            );
        }
    }

    nested
}

fn parse_nested_value(predicate_name: &str, nested: &PunctuatedAttributes) -> Option<String> {
    let mut predicate_value = None;

    for attr in nested {
        if attr.path().is_ident(predicate_name) {
            if let syn::Meta::NameValue(syn::MetaNameValue { value, .. }) = attr {
                if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = value {
                    if let syn::Lit::Str(lit_str) = lit {
                        predicate_value = Some(lit_str.value());
                    }
                }
            }
        }
    }

    predicate_value
}

fn parse_scope(attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    if let Some(nested) = parse_nested(attrs) {
        let scope_name = parse_nested_value("scope", &nested);

        let parse_chat_uid =
            |kind: &str, nested: &PunctuatedAttributes| -> proc_macro2::TokenStream {
                let message = format!(
                    "#[command(scope = \"{}\", chat_id = ???)] expect chat_id",
                    kind
                );
                let chat_id = parse_nested_value("chat_id", nested).expect(&message);

                quote::quote! { telegram_bots_api::api::enums::chat_uid::ChatUId::from(#chat_id) }
            };

        let parse_user_id =
            |kind: &str, nested: &PunctuatedAttributes| -> proc_macro2::TokenStream {
                let message = format!(
                    "#[command(scope = \"{}\", user_id = ???)] expect user_id",
                    kind
                );

                let user_id = parse_nested_value("user_id", nested)
                    .expect(&message)
                    .parse::<i64>()
                    .unwrap();

                quote::quote! { #user_id }
            };

        match scope_name {
            Some(kind) if kind == "default" => {
                quote::quote! {
                    Some(telegram_bots_api::api::enums::bot_command_scope::BotCommandScope::Default(
                        telegram_bots_api::api::structs::bot_command_scope_default::BotCommandScopeDefault {
                            kind: #kind.to_string()
                        }
                    ))
                }
            }
            Some(kind) if kind == "all_chat_administrators" => {
                quote::quote! {
                    Some(telegram_bots_api::api::enums::bot_command_scope::BotCommandScope::AllChatAdministrators(
                        telegram_bots_api::api::structs::bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators {
                            kind: #kind.to_string()
                        }
                    ))
                }
            }
            Some(kind) if kind == "all_group_chats" => {
                quote::quote! {
                    Some(telegram_bots_api::api::enums::bot_command_scope::BotCommandScope::AllGroupChats(
                        telegram_bots_api::api::structs::bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats {
                            kind: #kind.to_string()
                        }
                    ))
                }
            }
            Some(kind) if kind == "all_private_chats" => {
                quote::quote! {
                    Some(telegram_bots_api::api::enums::bot_command_scope::BotCommandScope::AllPrivateChats(
                        telegram_bots_api::api::structs::bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats {
                            kind: #kind.to_string()
                        }
                    ))
                }
            }
            Some(kind) if kind == "chat" => {
                let chat_uid = parse_chat_uid(&kind, &nested);

                quote::quote! {
                    Some(telegram_bots_api::api::enums::bot_command_scope::BotCommandScope::Chat(
                        telegram_bots_api::api::structs::bot_command_scope_chat::BotCommandScopeChat {
                            chat_id: #chat_uid,
                            kind: #kind.to_string(),
                        }
                    ))
                }
            }
            Some(kind) if kind == "chat_administrators" => {
                let chat_uid = parse_chat_uid(&kind, &nested);

                quote::quote! {
                    Some(telegram_bots_api::api::enums::bot_command_scope::BotCommandScope::ChatAdministrators(
                        telegram_bots_api::api::structs::bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators {
                            chat_id: #chat_uid,
                            kind: #kind.to_string(),
                        }
                    ))
                }
            }
            Some(kind) if kind == "chat_member" => {
                let user_id = parse_user_id(&kind, &nested);
                let chat_uid = parse_chat_uid(&kind, &nested);

                quote::quote! {
                    Some(telegram_bots_api::api::enums::bot_command_scope::BotCommandScope::ChatMember(
                        telegram_bots_api::api::structs::bot_command_scope_chat_member::BotCommandScopeChatMember {
                            user_id: #user_id,
                            chat_id: #chat_uid,
                            kind: #kind.to_string(),
                        }
                    ))
                }
            }
            None => quote::quote! { None },
            _ => {
                proc_macro_error::abort_call_site!(format!(
                    "#[command(scope = \"{}\")] scope doesn't supported",
                    scope_name.unwrap()
                ))
            }
        }
    } else {
        quote::quote! { None }
    }
}

fn parse_language_code(attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    let language_code = match parse_nested(attrs) {
        Some(nested) => parse_nested_value("language_code", &nested),
        None => None,
    };

    if language_code.is_some() {
        quote::quote! { Some(#language_code.to_string()) }
    } else {
        quote::quote! { None }
    }
}

#[proc_macro_error::proc_macro_error(allow_not_macro)]
pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => proc_macro_error::abort_call_site!("derive(BotCommands) expected enum"),
    };
    let ident = &input.ident;

    let raw_commands = parse_commands(enum_data);
    let commands: Vec<proc_macro2::TokenStream> = raw_commands
        .iter()
        .map(|(ident, description)| {
            let command = format!("/{}", ident.to_lowercase());

            quote::quote! {
                telegram_bots_api::api::structs::bot_command::BotCommand {
                    command: #command.to_string(),
                    description: #description.to_string(),
                }
            }
        })
        .collect();

    let language_code = parse_language_code(&input.attrs);
    let scope = parse_scope(&input.attrs);

    let enum_variants: Vec<_> = raw_commands
        .iter()
        .map(|(command, _)| {
            let command_pattern = format!("/{}", command.to_lowercase());
            let enum_variant =
                syn::parse_str::<syn::Expr>(&format!("{}::{}", ident, command)).unwrap();

            quote::quote! {
                #command_pattern => Some(#enum_variant)
            }
        })
        .collect();

    let quote = quote::quote! {
        impl #ident {
            pub fn dispatch(message: &telegram_framework::structs::messages::command::Command) -> Option<#ident> {
                match message.text.as_str() {
                    #(#enum_variants,)*
                    _ => None
                }
            }

            pub fn test(&self) {
                println!("Test");
            }
        }

        impl telegram_framework::traits::params::EnumParams for #ident {
            type Delete = telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
            type Get = telegram_bots_api::api::params::get_my_commands::GetMyCommands;
            type Set = telegram_bots_api::api::params::set_my_commands::SetMyCommands;

            fn config() -> (Self::Delete, Self::Get, Self::Set) {
                (Self::delete_params(), Self::get_params(), Self::set_params())
            }

            fn delete_params() -> Self::Delete {
                Self::Delete {
                    language_code: #language_code,
                    scope: #scope,
                }
            }

            fn get_params() -> Self::Get {
                Self::Get {
                    language_code: #language_code,
                    scope: #scope,
                }
            }

            fn set_params() -> Self::Set {
                Self::Set {
                    language_code: #language_code,
                    scope: #scope,
                    commands: vec![#(#commands), *]
                }
            }
        }
    };

    proc_macro::TokenStream::from(quote)
}
