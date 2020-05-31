use anyhow::Context;
use log::info;
use reqwest::Response;
use rotmg_extractor::ParsedClient;
use rotmg_packets::Parameters;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::{metadata, read_to_string, write};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientData {
    /// The URL of the client this data was extracted from
    pub client_url: String,

    /// The extracted data
    pub params: Parameters,
}

async fn get_client() -> anyhow::Result<Response> {
    reqwest::get("https://realmofthemadgodhrd.appspot.com/client")
        .await
        .context("downloading flash ROTMG client")
}

/// Download the latest ROTMG client and extract parameters from it, saving them
/// to the given file
async fn extract_params(to: &Path, response: Option<Response>) -> anyhow::Result<ClientData> {
    info!("Downloading ROTMG client...");

    let response = match response {
        Some(r) => r,
        None => get_client().await?,
    };

    let client_url = response.url().to_string();
    let client = response
        .bytes()
        .await
        .context("receiving flash ROTMG client")?;

    info!("Extracting ROTMG client data...");
    let params = ParsedClient::new(&client)
        .context("parsing client")?
        .extract_all()
        .context("extracting client parameters")?;

    let data = ClientData { client_url, params };
    write(to, serde_json::to_string_pretty(&data)?)
        .await
        .context("saving client params")?;

    Ok(data)
}

/// Get the client parameters, downloading and extracting data if necessary.
pub async fn get_params(data_dir: &Path) -> anyhow::Result<Parameters> {
    let file = data_dir.join("parameters.json");

    if metadata(&file).await.is_ok() {
        // there's some existing data - load it and check if it's up to date
        let old = serde_json::from_str::<ClientData>(
            &read_to_string(&file)
                .await
                .context("reading cached client params")?,
        )
        .context("parsing cached client params")?;

        let res = get_client().await?;

        if res.url().as_str() == old.client_url {
            // use cached data
            info!("Loaded cached client data");
            Ok(old.params)
        } else {
            // update data
            info!("Client data out of date");
            extract_params(&file, Some(res)).await.map(|p| p.params)
        }
    } else {
        // download and extract new data
        extract_params(&file, None).await.map(|p| p.params)
    }
}
