// This struct will hold the application's shared state
#[derive(Clone, Default)]
pub struct AppState {
    pub current_game: Option<(String, String)>, // (system_name, game_name)
}
