pub use config::ConfigError;
use dotenv::dotenv;
use serde::Deserialize;
use slog::{o, Drain};
use slog_async;
use slog_envlogger;
use slog_term;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub url: String,
    pub secret_key: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::load_dotenv();
        Self::configure_log();

        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        cfg.try_deserialize()
    }

    fn load_dotenv() {
        dotenv().ok();
    }

    fn configure_log() {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        let log = slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")));
        slog_scope::set_global_logger(log).cancel_reset();
        slog_stdlog::init().ok();
    }
}
