#![allow(clippy::collapsible_match)]

use crate::structs::command::Command;
use crate::visitors::commands::Commands;
use telegram_bots_api::api::enums::bot_command_scopes::BotCommandScopes;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::structs::bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators;
use telegram_bots_api::api::structs::bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats;
use telegram_bots_api::api::structs::bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats;
use telegram_bots_api::api::structs::bot_command_scope_chat::BotCommandScopeChat;
use telegram_bots_api::api::structs::bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;
use telegram_bots_api::api::structs::bot_command_scope_chat_member::BotCommandScopeChatMember;
use telegram_bots_api::api::structs::bot_command_scope_default::BotCommandScopeDefault;

const COMMAND_ATTRIBUTE_IDENT_NAME: &str = "command";
const DESCRIPTION_CHARS_MAX_COUNT: u16 = 256;

fn parse_commands(enum_data: &syn::DataEnum) -> Vec<Command> {
    let mut commands = Vec::new();

    enum_data.variants.iter().for_each(|variant: &syn::Variant| {
        let syn::Variant { ref attrs, .. } = variant;

        for attr in attrs {
            if attr.path().is_ident(COMMAND_ATTRIBUTE_IDENT_NAME) {
                attr.parse_nested_meta(|meta| {
                    let parse_stream = meta.value().unwrap();
                    let lit_str = parse_stream.parse::<syn::LitStr>().unwrap();
                    let description = lit_str.value();
                    let command = variant.ident.to_string().to_lowercase();

                    if description.len() > DESCRIPTION_CHARS_MAX_COUNT as usize {
                        proc_macro_error::abort_call_site!(
                            format!("command description value length must be less than {DESCRIPTION_CHARS_MAX_COUNT}")
                        )
                    }

                    commands.push(Command {
                        command,
                        description
                    });

                    Ok(())
                }).unwrap();
            }
        }
    });

    commands
}

fn parse_nested(
    attrs: &[syn::Attribute],
) -> Option<syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>> {
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

fn parse_nested_value(
    predicate_name: &str,
    nested: &syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>,
) -> Option<String> {
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

fn parse_scope(attrs: &[syn::Attribute]) -> Option<BotCommandScopes> {
    let nested = parse_nested(attrs).unwrap();
    let scope_name = parse_nested_value("scope", &nested);

    let parse_chat_uid = |kind: &str,
                          nested: &syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>|
     -> ChatUId {
        let message = format!(
            "#[command(scope = \"{}\", chat_id = ???)] expect chat_id",
            kind
        );
        let chat_id = parse_nested_value("chat_id", nested).expect(&message);

        ChatUId::from(chat_id)
    };

    let parse_user_id =
        |kind: &str, nested: &syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>| -> i64 {
            let message = format!(
                "#[command(scope = \"{}\", user_id = ???)] expect user_id",
                kind
            );

            parse_nested_value("user_id", nested)
                .expect(&message)
                .parse::<i64>()
                .unwrap()
        };

    match scope_name {
        Some(kind) if kind == "default" => {
            Some(BotCommandScopes::Default(BotCommandScopeDefault { kind }))
        }
        Some(kind) if kind == "all_chat_administrators" => Some(
            BotCommandScopes::AllChatAdministrators(BotCommandScopeAllChatAdministrators { kind }),
        ),
        Some(kind) if kind == "all_group_chats" => Some(BotCommandScopes::AllGroupChats(
            BotCommandScopeAllGroupChats { kind },
        )),
        Some(kind) if kind == "all_private_chats" => Some(BotCommandScopes::AllPrivateChats(
            BotCommandScopeAllPrivateChats { kind },
        )),
        Some(kind) if kind == "chat" => Some(BotCommandScopes::Chat(BotCommandScopeChat {
            chat_id: parse_chat_uid(&kind, &nested),
            kind,
        })),
        Some(kind) if kind == "chat_administrators" => Some(BotCommandScopes::ChatAdministrators(
            BotCommandScopeChatAdministrators {
                chat_id: parse_chat_uid(&kind, &nested),
                kind,
            },
        )),
        Some(kind) if kind == "chat_member" => {
            Some(BotCommandScopes::ChatMember(BotCommandScopeChatMember {
                user_id: parse_user_id(&kind, &nested),
                chat_id: parse_chat_uid(&kind, &nested),
                kind,
            }))
        }
        None => None,
        _ => {
            proc_macro_error::abort_call_site!(format!(
                "#[command(scope = \"{}\")] scope doesn't supported",
                scope_name.unwrap()
            ))
        }
    }
}

fn parse_language_code(attrs: &[syn::Attribute]) -> Option<String> {
    let nested = parse_nested(attrs).unwrap();

    parse_nested_value("language_code", &nested)
}

#[proc_macro_error::proc_macro_error(allow_not_macro)]
pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => proc_macro_error::abort_call_site!("derive(BotCommands) expected enum"),
    };

    let commands = Commands {
        commands: parse_commands(enum_data),
        language_code: parse_language_code(&input.attrs),
        scope: parse_scope(&input.attrs),
    };

    dbg!(commands);

    let quote = quote::quote! {};

    proc_macro::TokenStream::from(quote)
}
