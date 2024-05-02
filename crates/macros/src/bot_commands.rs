use telegram_bots_api::api::enums::bot_command_scopes::BotCommandScopes;

const IDENT_NAME: &str = "command";
const DESCRIPTION_CHARS_MAX_COUNT: u16 = 256;
const COMMAND_REGEXP_PATTERN: &str = r"[a-z_1-9]*";

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Commands {
    commands: Vec<Command>,
    scope: Option<BotCommandScopes>,
    language_code: Option<String>
}

impl Commands {
    pub fn push(&mut self, command: Command) {
        self.commands.push(command);
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Command {
    command: String,
    description: String
}

fn parse_commands(enum_data: &syn::DataEnum) -> Commands {
    let mut commands = Commands::default();

    enum_data.variants.iter().for_each(|variant: &syn::Variant| {
        let syn::Variant { ref attrs, .. } = variant;

        for attr in attrs {
            if attr.path().is_ident(IDENT_NAME) {
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

#[proc_macro_error::proc_macro_error(allow_not_macro)]
pub fn impl_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => proc_macro_error::abort_call_site!("derive(BotCommands) expected enum"),
    };

    let commands = parse_commands(enum_data);

    dbg!(commands);

    let quote = quote::quote! {};

    proc_macro::TokenStream::from(quote)
}
