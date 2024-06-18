use clap::Parser;
use telegram_bots_api::config::Config as Inner;

#[derive(Debug, Clone, Parser)]
pub struct Config {
    /// Environment: Debug mode.
    #[arg(long)]
    pub debug: bool,

    /// Environment: Is production.
    #[arg(long)]
    pub production: bool,

    /// Telegram: Token.
    #[arg(long)]
    pub token: String,

    /// Telegram: Api url.
    #[arg(long, default_value = "https://api.telegram.org")]
    pub url: String,

    /// Telegram: Api url.
    #[arg(long, default_value = "")]
    pub webhook: String,

    /// Path to TSL Certificate. Used with webhook
    #[arg(long, default_value = None)]
    pub certificate: Option<String>,

    /// Secret token. Used with webhook.
    #[arg(long, default_value = None)]
    pub secret_token: Option<String>,

    /// Ip address. Used with webhook.
    #[arg(long, default_value = None)]
    pub ip_address: Option<String>,

    ///
    #[arg(long, default_value = None)]
    pub max_connections: Option<u32>,

    /// Client: Timeout in secs. The timeout is applied from when the request starts connecting until the response body has finished.
    #[arg(long, default_value = "5")]
    pub timeout: u64,

    /// Client: Connect timeout in secs. Set a timeout for only the connect phase.
    #[arg(long, default_value = "5")]
    pub connect_timeout: u64,

    /// Pooling timeout for getting updates.
    #[arg(long, default_value = "1")]
    pub pooling_timeout: u64,

    /// Updates: Drop pending on restart
    #[arg(long, default_value = "true")]
    pub updates_drop_pending: bool,

    /// Updates: Identifier of the first update to be returned.
    #[arg(long, default_value = "0")]
    pub updates_offset: i64,

    /// Updates: Limits the number of updates to be retrieved.
    #[arg(long, default_value = "100")]
    pub updates_limit: i64,

    /// Updates: Timeout in seconds for long polling.
    #[arg(long, default_value = "0")]
    pub updates_timeout: u64,

    /// Updates: Allowed type of updates
    #[arg(long, default_value = None)]
    pub updates_allowed: Option<Vec<String>>,
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }
}

impl From<Config> for Inner {
    fn from(config: Config) -> Self {
        Inner {
            debug: config.debug,
            production: config.production,
            token: config.token,
            url: config.url,
            webhook: config.webhook,
            timeout: config.timeout,
            connect_timeout: config.connect_timeout,
            updates_limit: config.updates_limit,
            updates_offset: config.updates_offset,
            updates_timeout: config.updates_timeout,
        }
    }
}
