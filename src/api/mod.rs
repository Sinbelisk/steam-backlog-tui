use std::fmt::{self, Display};
use serde::Deserialize;
use tracing::info;

use crate::util;

/// This struct represents a wrapper for the reqwest client using steam parameters
pub struct SteamClient { 
    client : reqwest::Client,
    api_key: String
}

#[derive(Debug)]
pub enum SteamEndpoint {
    GetOwnedGames,
    GetPlayerSummaries
}

impl fmt::Display for SteamEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self)
    }
}

impl SteamEndpoint {

    pub fn create_path(self) -> util::UrlBuilder{
        const BASE_URL : &str = "http://api.steampowered.com";
        let interface = self.get_interface_and_version();

        util::UrlBuilder::new(BASE_URL)
            .endpoint(interface.0)
            .endpoint(format!("{}/{}/{}/",interface.0, self, interface.1))
    }

    // I could make another private method for getting the version, but i am lazy.
    fn get_interface_and_version(&self) -> (&str, &str) {
        match self {
            SteamEndpoint::GetOwnedGames => ("IPlayerService", "v0001"),
            SteamEndpoint::GetPlayerSummaries => ("ISteamUser", "v0002")
        }
    }
}

pub fn test() {
    SteamEndpoint::GetOwnedGames.create_path();
}

#[derive(Deserialize, Debug)]
struct ResponseWrapper<T> {
    response: T
}

impl SteamClient {
    pub fn new(api_key : String) -> SteamClient {
        SteamClient {
            client:  reqwest::Client::new(),
            api_key: api_key
        }
    }

    /// Generic method for GET operations using Steam Web Api
    async fn get<T, U>(self, url : U) -> Result<T, Box<dyn std::error::Error>>
    where 
        T: for<'de> Deserialize<'de> + fmt::Display,
        U: AsRef<str> + reqwest::IntoUrl + fmt::Display,
    {
        info!("Fetching data from {}", &url);

        let response = self.client.get(url.as_ref().to_owned()).send().await?;
        let wrapper = response.json::<ResponseWrapper<T>>().await?;

        info!("Received data: {}", &wrapper.response);
        Ok(wrapper.response)
    }

}

pub mod steam_library;
pub mod steamuser_info;
