use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub track_index: usize,
    pub is_playing: bool,
    pub is_paused: bool,
    pub track_list: Vec<&'static str>,
    pub volume: f32,
    pub auto_next_track: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            track_index: 0,
            is_playing: false,
            is_paused: false,
            track_list: vec!["tracks/01 Intro.mp3", "tracks/StarWars3.wav", "tracks/Sample_BeeMoved_96kHz24bit.flac", "tracks/07 - Bitty Mclean - Dedicated To The One I Love.mp3"],
            volume: 0.05,
            auto_next_track: true,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_track_index(&mut self) {
        if self.track_index < self.track_list.len() - 1 {
            self.track_index += 1;
        }
    }

    pub fn decrement_track_index(&mut self) {
        if let Some(res) = self.track_index.checked_sub(1) {
            self.track_index = res;
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn increase_volume(&mut self) {
        const VOLUME_INCREMENT: f32 = 0.01;
        let new_volume = (self.volume + VOLUME_INCREMENT).min(1.0); // Ensure volume doesn't exceed 1.0
        let rounded_volume = (new_volume * 1000.0).round() / 1000.0; 
        self.set_volume(rounded_volume);
    }
    
    pub fn decrease_volume(&mut self) {
        const VOLUME_DECREMENT: f32 = 0.01;
        let new_volume = (self.volume - VOLUME_DECREMENT).max(0.0); // Ensure volume doesn't go below 0.0
        let rounded_volume = (new_volume * 1000.0).round() / 1000.0; 
        self.set_volume(rounded_volume);
    }

    pub fn play_audio(&mut self) -> AppResult<()> {
        self.is_playing = true;
        self.is_paused = false;
        Ok(())
    }

    pub fn pause_audio(&mut self) -> AppResult<()> {
        self.is_playing = true;
        self.is_paused = true;
        Ok(())
    }

    pub fn stop_audio(&mut self) -> AppResult<()> {
        self.is_playing = false;
        self.is_paused = false;
        Ok(())
    }
}
