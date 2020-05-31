use anyhow::Context;
use log::info;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Server {
    /// The user-readable name of the server
    #[serde(alias = "Name")]
    pub name: String,

    /// The hostname of the server
    #[serde(alias = "DNS")]
    pub host: String,
}

impl Server {
    /// Automatically generate an abbreviated form of this server's name.
    pub fn abbreviated_name(&self) -> String {
        self.name
            .to_lowercase()
            .replace("east", "e")
            .replace("west", "w")
            .replace("south", "s")
            .replace("north", "n")
            .replace("asia", "as")
            .replace("mid", "m")
            .replace("australia", "aus")
    }
}

pub struct ServerList {
    /// Map of server names/abbreviations -> address
    map: HashMap<String, String>,

    /// Default server address
    default: String,
}

impl ServerList {
    /// Load the official ROTMG server list
    pub async fn load(default: &str) -> anyhow::Result<Self> {
        let list = reqwest::get("https://realmofthemadgodhrd.appspot.com/char/list")
            .await
            .context("getting server list")?
            .text()
            .await
            .context("reading server list")?;

        #[derive(Deserialize)]
        struct Servers {
            #[serde(alias = "Server")]
            server: Vec<Server>,
        }

        #[derive(Deserialize)]
        struct Chars {
            #[serde(alias = "Servers")]
            servers: Servers,
        }

        let list: Chars = serde_xml_rs::from_str(&list).context("parsing server list")?;

        info!("Loaded {} servers", list.servers.server.len());

        let mut map = HashMap::new();
        for server in list.servers.server {
            let abbr = server.abbreviated_name();
            let Server { name, host } = server;
            map.insert(name, host.clone());
            map.insert(abbr, host);
        }

        let default = map
            .get(default)
            .context("getting default server address")?
            .to_string();

        Ok(Self { map, default })
    }

    /// Get the address of a server from this list
    pub fn get(&self, name: &str) -> Option<&str> {
        self.map.get(name).map(|s| s.as_str())
    }

    /// Get the address of the default server
    pub fn get_default(&self) -> &str {
        &self.default
    }
}
