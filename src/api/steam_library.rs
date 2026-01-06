use std::fmt::{format, write};

use color_eyre::eyre::Error;
use serde::Deserialize;
use super::*;

#[derive(Deserialize, Debug)]
pub struct SteamGame {
    appid : u32,
    name: String,
    playtime_forever : u64,
    img_icon_url: String,
}

impl fmt::Display for SteamGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "appid: {}, \nname:{}, \nplaytime:{}, \nicon_url:{}",
            self.appid, self.name, self.playtime_forever, self.img_icon_url)
    }
}

#[derive(Deserialize, Debug)]
pub struct OwnedGamesCollection {
    game_count: u32,
    games : Vec<SteamGame>
}

impl fmt::Display for OwnedGamesCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "owned games: {}, \ngames:{:?}", self.game_count, self.games)
    }
}

impl SteamClient {
    pub async fn fetch_owned_games  (
        self,
        steamid: String,
    ) -> Result<OwnedGamesCollection, Box<dyn std::error::Error>> {
        let url = SteamEndpoint::GetOwnedGames
            .create_path()
            .parameter(format!("key={}", self.api_key))
            .parameter(format!("steamid={steamid}"))
            .parameter("format=json")
            .parameter("include_appinfo=true")
            .parameter("include_played_free_games=false")
            .build();

        let response = self.get::<OwnedGamesCollection, _>(url).await?;
        Ok(response)
    }

}

// This test is stoopid, ignore it :)
#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use tracing::Level;

    #[tokio::test]
    #[ignore = "Execute Manually"]
    async fn fetch_steam_games_total_games_equals_153() {
        dotenv().ok();
        let stdout_subscriber = tracing_subscriber::fmt()
            .with_test_writer()
            .try_init();

        let api_key = dotenv!("API_KEY");
        let steam_id = dotenv!("STEAM_ID");
        let sclient = SteamClient::new(api_key.to_string());

        let response = sclient.fetch_owned_games(steam_id.to_string())
            .await.expect("Error fetching games");

        assert_eq!(153, response.game_count);
    }
}
