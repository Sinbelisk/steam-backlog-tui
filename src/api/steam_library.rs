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

#[derive(Deserialize, Debug)]
pub struct OwnedGamesCollection {
    game_count: u32,
    games : Vec<SteamGame>
}

impl SteamClient {
    pub async fn fetch_owned_games  (
        self,
        steamid: String,
    ) -> Result<OwnedGamesCollection, Box<dyn std::error::Error>> {
        let url = SteamEndpoint::GetOwnedGames
            .create_path()
            .parameter(format!("steamid={steamid}"))
            .parameter("format=json")
            .parameter("include_appinfo=true")
            .build();

        let response = self.client.get(url)
            .send().await?; 

        let library = response.json::<ResponseWrapper<OwnedGamesCollection>>().await?;
        Ok(library.response)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    #[ignore = "Execute Manually"]
    async fn fetch_steam_games_total_games_equals_153() {
        dotenv().ok();

        let api_key = dotenv!("API_KEY");
        let steam_id = dotenv!("STEAM_ID");
        let sclient = SteamClient::new(api_key.to_string());

        let response = sclient.fetch_owned_games(steam_id.to_string())
            .await.expect("Error fetching games");

        assert_eq!(153, response.game_count);
    }
}
