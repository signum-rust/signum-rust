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
        //add values from a file
        .add_source(config::File::from(base_path.join(configuration_file)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    let settings: Result<Settings, config::ConfigError> = settings.try_deserialize();
    tracing::debug!("Settings values: {:#?}", &settings);
    settings
}

trait ConfigBuilderExtensions {
    fn add_defaults(self) -> Result<Self, config::ConfigError>
    where
        Self: Sized;
}

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
    /// Address that peers should attempt to connect to.
    #[serde(default = "default_value_my_address")]
    pub my_address: String,
    /// A string indicating the platform in use. Often set to a signum address for SNR rewards.
    #[serde(default = "default_value_platform")]
    pub platform: String,
    /// Whether or not peers should pass along your address to their own peers.
    #[serde(default = "default_value_share_address")]
    pub share_address: bool,
    /// The name of the network to which this node is connecting.
    #[serde(default = "default_value_network_name")]
    pub network_name: String,
    /// The address to which SNR awards should be paid. Currently unused on the network.
    #[serde(default = "default_value_snr_reward_address")]
    pub snr_reward_address: String,
}

fn default_value_bootstrap_peers() -> Vec<PeerAddress> {
    vec![
        PeerAddress::from_str("212.98.92.236").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("108.61.251.202").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("190.15.195.118").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("24.96.113.8").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("88.64.234.237").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("137.135.203.145").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("70.108.6.237").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("144.91.84.164").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("213.32.102.141").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("5.196.65.184").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("89.163.239.219").expect("could not parse bootstrap ip address"),
        PeerAddress::from_str("165.227.36.71").expect("could not parse bootstrap ip address"),
    ]
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
