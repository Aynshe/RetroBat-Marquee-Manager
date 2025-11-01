use crate::config::Config;
use crate::marquee::{self, MarqueeType};
use crate::process;
use crate::state::AppState;
use log::{error, info};

use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{mpsc::channel, Arc, Mutex};
use url::form_urlencoded;

pub fn start_watching(
    config: &Config,
    systems: &HashMap<String, String>,
    app_state: Arc<Mutex<AppState>>,
) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, NotifyConfig::default())?;
    let event_file_path = Path::new("ESEvent.arg");

    // Ensure the file exists before watching
    if !event_file_path.exists() {
        if let Err(e) = fs::File::create(&event_file_path) {
            error!("Failed to create event file: {}", e);
        }
    }

    watcher.watch(&event_file_path, RecursiveMode::NonRecursive)?;

    info!("Watching for events in ESEvent.arg...");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if let notify::EventKind::Modify(_) = event.kind {
                    if let Ok(content) = fs::read_to_string(&event_file_path) {
                        handle_event(&content, config, systems, &app_state);
                    }
                }
            }
            Ok(Err(e)) => error!("Watch error: {:?}", e),
            Err(e) => error!("Channel receive error: {:?}", e),
        }
    }
}

fn handle_event(
    content: &str,
    config: &Config,
    systems: &HashMap<String, String>,
    app_state: &Arc<Mutex<AppState>>,
) {
    let params: HashMap<String, String> = form_urlencoded::parse(content.as_bytes())
        .into_owned()
        .collect();

    let event = params.get("event").cloned().unwrap_or_default();
    let param1 = params.get("param1").cloned().unwrap_or_default();
    let param2 = params.get("param2").cloned().unwrap_or_default();

    info!("Event received: {}, param1: {}, param2: {}", event, param1, param2);

    let marquee_type = match event.as_str() {
        "system-selected" => Some(MarqueeType::System {
            system_name: &param1,
        }),
        "game-selected" => {
            let mut state = app_state.lock().unwrap();
            state.current_game = Some((param1.clone(), param2.clone()));
            Some(MarqueeType::Game {
                system_name: &param1,
                game_name: &param2,
            })
        }
        _ => None,
    };

    if let Some(mt) = marquee_type {
        let marquee_file = marquee::find_marquee_file(mt, config, systems);
        info!("Updating marquee to: {:?}", marquee_file);
        process::update_marquee(&marquee_file, config);
    }
}
