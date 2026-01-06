use std::fmt;

use crate::api::{ResponseWrapper, SteamClient, SteamEndpoint};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SteamUser {
    steamid : String,
    personaname: String,
    avatar: String,
    profileurl: String,
}

#[derive(Deserialize, Debug)]
struct PlayerSummary {
    players: Vec<SteamUser>
}

impl fmt::Display for SteamUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "steamid:{}, username:{}, avatar:{}, profileurl:{}",
            self.steamid, self.personaname, self.avatar, self.profileurl)
    }
}

impl fmt::Display for PlayerSummary{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SteamClient {
    pub async fn fetch_user_info(
        self,
        steamid: String,
    ) -> Result<SteamUser, Box<dyn std::error::Error>> {
        let url = SteamEndpoint::GetPlayerSummaries
            .create_path()
            .parameter(format!("key={}", self.api_key))
            .parameter(format!("steamids={}",steamid))
            .parameter("format=json")
            .build();

        let mut response = self.get::<PlayerSummary,_>(url).await?;

        // Behold this unholy abomination.
        let fetched_user = response.players.pop().unwrap(); 
        Ok(fetched_user)

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use tracing::Level;

    #[tokio::test]
    #[ignore = "Execute Manually"]
    async fn fetch_steam_user_user_is_developer() {
        dotenv().ok();
        let stdout_subscriber = tracing_subscriber::fmt()
            .with_test_writer()
            .try_init();

        let api_key = dotenv!("API_KEY");
        let steam_id = "76561198095102116";
        let sclient = SteamClient::new(api_key.to_string());

        let response = sclient.fetch_user_info(steam_id.to_string())
            .await.expect("Error fetching games");

        assert_eq!("Sinrael", response.personaname)
    }
}
