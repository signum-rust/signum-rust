use std::str::FromStr;

use anyhow::Context;
use serde::Deserialize;
use sqlx::sqlite::SqliteConnectOptions;

use crate::models::p2p::PeerAddress;

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get the base execution director
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    // Set the configuration file
    let configuration_file = "configuration.yml";

    let settings = config::Config::builder()
        // .add_defaults()?
        //add values from a file
        .add_source(config::File::from(base_path.join(configuration_file)))
        .build()?;

    let settings = settings.try_deserialize::<Settings>();
    tracing::debug!("Settings values: {:#?}", &settings);
    settings
}

trait ConfigBuilderExtensions {
    fn add_defaults(self) -> Result<Self, config::ConfigError>
    where
        Self: Sized;
}

// impl<St: config::builder::BuilderState> ConfigBuilderExtensions for ConfigBuilder<St> {
//     fn add_defaults(self) -> Result<Self, config::ConfigError> {
//         self.set_default(
//             "settings.p2p.bootstrap_peers",
//             vec![PeerAddress::from_str("us-east.signum.network:8123")?],
//         )
//     }
// }

/// Settings for the node.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub srs_api: SrsApiSettings,
    pub database: DatabaseSettings,
    pub node: NodeSettings,
    pub p2p: PeerToPeerSettings,
}

/// Settings for the signum-style API.
#[derive(Clone, Debug, Deserialize)]
pub struct SrsApiSettings {
    pub base_url: String,
    pub listen_address: String,
    pub listen_port: u16,
}

/// Database settings.
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseSettings {
    pub filename: String,
}

impl DatabaseSettings {
    pub fn get_writable_db(&self) -> Result<SqliteConnectOptions, anyhow::Error> {
        let connection_string = if !&self.filename.starts_with("sqlite:") {
            format!("sqlite://{}", self.filename)
        } else {
            self.filename.clone()
        };
        Ok(SqliteConnectOptions::from_str(&connection_string)
            .context("couldn't parse connection string")?
            .optimize_on_close(true, None)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .create_if_missing(true))
    }
    pub fn get_read_only_db(&self) -> Result<SqliteConnectOptions, anyhow::Error> {
        Ok(self.get_writable_db()?.read_only(true))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct NodeSettings {
    pub cash_back_id: String,
    pub network: String,
}

/// Peer to Peer settings.
#[derive(Clone, Debug, Deserialize)]
pub struct PeerToPeerSettings {
    /// Peer addresses to use if none are in the database already.
    #[serde(default = "default_value_bootstrap_peers")]
    pub bootstrap_peers: Vec<PeerAddress>,
    #[serde(default = "default_value_my_address")]
    pub my_address: String,
    #[serde(default = "default_value_platform")]
    pub platform: String,
    #[serde(default = "default_value_share_address")]
    pub share_address: bool,
    #[serde(default = "default_value_network_name")]
    pub network_name: String,
    #[serde(default = "default_value_snr_reward_address")]
    pub snr_reward_address: String,
}

fn default_value_bootstrap_peers() -> Vec<PeerAddress> {
    //TODO: add default peers here
    Vec::new()
}

fn default_value_my_address() -> String {
    //TODO: Figure out a way to get external IP and populate it
    String::new()
}

fn default_value_platform() -> String {
    String::new()
}

fn default_value_share_address() -> bool {
    true
}

fn default_value_network_name() -> String {
    "Signum".to_string()
}

fn default_value_snr_reward_address() -> String {
    String::new()
}
