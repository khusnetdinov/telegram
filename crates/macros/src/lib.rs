mod bot_commands;
mod bot_states;
mod deref_inner;
mod from_inner;

#[proc_macro_derive(BotCommands, attributes(command))]
pub fn bot_commands_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bot_commands::impl_proc_macro(input)
}

#[proc_macro_derive(BotStates)]
pub fn bot_states_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bot_states::impl_proc_macro(input)
}

#[proc_macro_derive(DerefInner)]
pub fn deref_inner_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    deref_inner::impl_proc_macro(input)
}

#[proc_macro_derive(FromInner)]
pub fn from_inner_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_inner::impl_proc_macro(input)
}
