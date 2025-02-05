use arti_client::{TorClient, TorClientConfig};
use tor_rtcompat::PreferredRuntime;

pub struct Tor {
    tor_client: TorClient<PreferredRuntime>,
}

impl Tor {
    pub async fn new() -> anyhow::Result<Self> {
        let config = TorClientConfig::default();
        let tor_client = TorClient::create_bootstrapped(config).await?;
        Ok(Self { tor_client })
    }
}
