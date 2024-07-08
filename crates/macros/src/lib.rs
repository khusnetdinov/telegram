mod bot_commands;
mod deref_inner;
mod from_inner;
mod from_remote;

#[proc_macro_derive(BotCommands, attributes(command))]
pub fn bot_commands_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bot_commands::impl_proc_macro(input)
}

#[proc_macro_derive(DerefInner)]
pub fn deref_inner_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    deref_inner::impl_proc_macro(input)
}

#[proc_macro_derive(FromInner)]
pub fn from_inner_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_inner::impl_proc_macro(input)
}

#[proc_macro_derive(FromRemote)]
pub fn from_remote_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_remote::impl_proc_macro(input)
}
