use std::{fs, path::PathBuf};

use arti_client::{TorClient, TorClientConfig};
use log::error;
use sled::Db;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    runtime::{Builder, Runtime},
};
use tor_rtcompat::PreferredRuntime;

static DATABASE_PATH: &str = "./database";
static CONFIG_DATABASE: &str = "config.db";
static CONTACTS_DATABASE: &str = "contacts_database.db";

pub struct ChickenCore {
    runtime: Runtime,

    config_db: Db,
    contacts_db: Db,

    tor_client: TorClient<PreferredRuntime>,
}

impl ChickenCore {
    pub fn new(runtime: Option<Runtime>) -> anyhow::Result<Self> {
        let runtime = match runtime {
            Some(runtime) => runtime,
            None => Builder::new_multi_thread().enable_all().build()?,
        };

        let config_db = sled::open(PathBuf::from_iter([DATABASE_PATH, CONFIG_DATABASE]))?;
        let contacts_db = sled::open(PathBuf::from_iter([DATABASE_PATH, CONTACTS_DATABASE]))?;

        let tor_config = TorClientConfig::default();
        let tor_client = runtime.block_on(TorClient::create_bootstrapped(tor_config))?;

        Ok(Self {
            runtime,
            config_db,
            contacts_db,

            tor_client,
        })
    }

    pub fn login(&mut self, password: &[u8]) {}

    pub fn logout(&mut self) {}

    pub fn add_contact(&self) {}

    pub fn remove_contact(&self) {}

    pub fn send_message(&self, message: &[u8]) {}

    pub fn receive_messages(&self) {}

    pub fn reset_data(&self) {}
}
