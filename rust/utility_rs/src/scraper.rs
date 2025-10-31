use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use futures_util::StreamExt;

#[derive(Debug, Deserialize)]
struct ScrapEntry {
    system_name: String,
    game_title: String,
    game_name: String,
    marquee_path: String,
    full_marquee_path: String,
    rom_path: String,
}

#[derive(Debug, Deserialize)]
struct ScreenScraperResponse {
    response: Response,
}

#[derive(Debug, Deserialize)]
struct Response {
    jeu: Game,
}

#[derive(Debug, Deserialize)]
struct Game {
    medias: Vec<Media>,
}

#[derive(Debug, Deserialize)]
struct Media {
    #[serde(rename = "type")]
    media_type: String,
    url: String,
}

pub async fn run(pool_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let content = fs::read_to_string(pool_file)?;
    let entries: Vec<ScrapEntry> = content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 6 {
                Some(ScrapEntry {
                    system_name: parts[0].to_string(),
                    game_title: parts[1].to_string(),
                    game_name: parts[2].to_string(),
                    marquee_path: parts[3].to_string(),
                    full_marquee_path: parts[4].to_string(),
                    rom_path: parts[5].to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    for entry in entries {
        println!("Scraping marquee for: {}", entry.game_title);
        if let Err(e) = download_marquee(&client, &entry).await {
            eprintln!("Failed to download marquee for {}: {}", entry.game_title, e);
        }
    }

    Ok(())
}

async fn download_marquee(client: &Client, entry: &ScrapEntry) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.screenscraper.fr/api2/jeuInfos.php?devid=YOUR_DEV_ID&devpassword=YOUR_DEV_PASS&softname=MarqueeManager&output=json&romnom={}&systemeid=0",
        entry.game_name
    );

    let resp: ScreenScraperResponse = client.get(&url).send().await?.json().await?;
    if let Some(marquee_media) = resp.response.jeu.medias.iter().find(|m| m.media_type == "marquee") {
        let mut stream = client.get(&marquee_media.url).send().await?.bytes_stream();
        let target_path = PathBuf::from(&entry.full_marquee_path);
        let mut file = fs::File::create(target_path)?;

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
        }
        println!("Marquee downloaded successfully for {}", entry.game_title);
    }

    Ok(())
}
