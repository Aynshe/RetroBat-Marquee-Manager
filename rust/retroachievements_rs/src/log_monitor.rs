use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub enum LogEvent {
    GameIdentified(u32),
    AchievementUnlocked(u32),
}

pub fn start_watching(log_path: &Path, tx: Sender<LogEvent>) -> std::io::Result<()> {
    let mut file = File::open(log_path)?;
    let mut last_pos = file.seek(SeekFrom::End(0))?;

    let re_game = Regex::new(r"Identified game: (\d+)").unwrap();
    let re_achievement = Regex::new(r"Achievement (\d+) awarded").unwrap();

    loop {
        let current_pos = file.seek(SeekFrom::End(0))?;
        if current_pos > last_pos {
            file.seek(SeekFrom::Start(last_pos))?;
            let reader = BufReader::new(&mut file);
            for line in reader.lines() {
                let line = line?;
                if let Some(caps) = re_game.captures(&line) {
                    if let Some(game_id_str) = caps.get(1) {
                        if let Ok(game_id) = game_id_str.as_str().parse() {
                            tx.send(LogEvent::GameIdentified(game_id)).unwrap();
                        }
                    }
                } else if let Some(caps) = re_achievement.captures(&line) {
                    if let Some(ach_id_str) = caps.get(1) {
                        if let Ok(ach_id) = ach_id_str.as_str().parse() {
                            tx.send(LogEvent::AchievementUnlocked(ach_id)).unwrap();
                        }
                    }
                }
            }
            last_pos = file.seek(SeekFrom::Current(0))?;
        }
        thread::sleep(Duration::from_millis(500));
    }
}
