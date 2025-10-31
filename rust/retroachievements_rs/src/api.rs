use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const API_BASE_URL: &str = "https://retroachievements.org/API/";
const CACHE_DIR: &str = "RA/API";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "UserPic")]
    pub user_pic: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInfoAndUserProgress {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "GameIcon")]
    pub game_icon: String,
    #[serde(rename = "Achievements")]
    pub achievements: HashMap<String, Achievement>,
    #[serde(rename = "NumAwardedToUser")]
    pub num_awarded_to_user: u32,
    #[serde(rename = "UserCompletion")]
    pub user_completion: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Achievement {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Points")]
    pub points: u32,
    #[serde(rename = "BadgeName")]
    pub badge_name: String,
}

pub struct ApiClient {
    client: reqwest::Client,
    username: String,
    api_key: String,
    cache_path: PathBuf,
}

impl ApiClient {
    pub fn new(username: String, api_key: String) -> Self {
        fs::create_dir_all(CACHE_DIR).expect("Failed to create cache directory");
        ApiClient {
            client: reqwest::Client::new(),
            username,
            api_key,
            cache_path: PathBuf::from(CACHE_DIR),
        }
    }

    async fn get<T: DeserializeOwned>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T, reqwest::Error> {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        self.client.get(&url).query(params).send().await?.json().await
    }

    async fn get_with_cache<T: DeserializeOwned + Serialize>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
        cache_key: &str,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let cache_file = self.cache_path.join(format!("{}.json", cache_key));
        if cache_file.exists() {
            let cached_data = fs::read_to_string(cache_file)?;
            let result = serde_json::from_str(&cached_data)?;
            return Ok(result);
        }

        let mut all_params = params.to_vec();
        all_params.push(("z", &self.username));
        all_params.push(("y", &self.api_key));

        let response: T = self.get(endpoint, &all_params).await?;
        let response_json = serde_json::to_string(&response)?;
        fs::write(cache_file, response_json)?;
        Ok(response)
    }

    pub async fn get_user_profile(&self, user: &str) -> Result<UserProfile, Box<dyn std::error::Error>> {
        let params = [("u", user)];
        let cache_key = format!("API_GetUserProfile.php_user{}", user);
        self.get_with_cache("API_GetUserProfile.php", &params, &cache_key).await
    }

    pub async fn get_game_info_and_user_progress(
        &self,
        game_id: u32,
        user: &str,
    ) -> Result<GameInfoAndUserProgress, Box<dyn std::error::Error>> {
        let game_id_str = game_id.to_string();
        let params = [("g", game_id_str.as_str()), ("u", user)];
        let cache_key = format!("API_GetGameInfoAndUserProgress.php_game{}", game_id);
        self.get_with_cache("API_GetGameInfoAndUserProgress.php", &params, &cache_key)
            .await
    }
}
