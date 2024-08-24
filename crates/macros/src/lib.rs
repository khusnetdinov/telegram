mod bot_commands;
mod from_remote_enum;
mod from_remote_struct;
mod helpers;
mod into_remote_enum;
mod into_remote_struct;

#[proc_macro_derive(BotCommands, attributes(command))]
pub fn bot_commands_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bot_commands::impl_proc_macro(input)
}

#[proc_macro_derive(FromRemoteStruct)]
pub fn from_remote_struct_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_remote_struct::impl_proc_macro(input)
}

#[proc_macro_derive(IntoRemoteStruct)]
pub fn into_remote_struct_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    into_remote_struct::impl_proc_macro(input)
}

#[proc_macro_derive(FromRemoteEnum)]
pub fn from_remote_enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_remote_enum::impl_proc_macro(input)
}

#[proc_macro_derive(IntoRemoteEnum)]
pub fn into_remote_enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    into_remote_enum::impl_proc_macro(input)
}
