use std::fmt::{self, Display};
use reqwest::Response;
use serde::Deserialize;

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
        const BASE_URL : &str = "http://api.steampowered.com/IPlayerService";

        util::UrlBuilder::new(BASE_URL)
            .endpoint(format!("/{}/v0001/", self))
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
    async fn get<T, U>(&self, url : U) -> Result<T, Box<dyn std::error::Error>>
    where 
        T: for<'de> Deserialize<'de>,
        U: AsRef<str> + reqwest::IntoUrl,
    {
        let response = self.client.get(url.as_ref()).send().await?;
        let wrapper = response.json::<ResponseWrapper<T>>().await?;

        Ok(wrapper.response)
    }

}


pub mod steam_library;
