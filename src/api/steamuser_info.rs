use crate::api::{ResponseWrapper, SteamClient, SteamEndpoint};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SteamUser {
    steamid : String,
    personaname: String,
    avatar: String,
    profileurl: String,
}


impl SteamClient {
    pub async fn fetch_user_info(
        self,
        steamid: String,
    ) -> Result<SteamUser, Box<dyn std::error::Error>> {
        let url = SteamEndpoint::GetOwnedGames
            .create_path()
            .parameter(format!("steamid={steamid}"))
            .parameter("format=json")
            .build();

        let response = self.client.get(url)
            .send().await?;

        Ok(response.json::<ResponseWrapper<SteamUser>>().await?
            .response)
    }
}
