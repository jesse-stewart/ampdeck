use std::error;

use walkdir::WalkDir;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub track_index: usize,
    pub sink_empty: bool,
    pub playing: bool,
    pub paused: bool,
    pub track_list: Vec<String>,
    pub volume: f32,
    pub auto_next_track: bool,
    pub track_duration: u64,
    pub track_progress: u64,
    pub track_title: String,
    pub track_artist: String,
    pub track_album: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            track_index: 0,
            sink_empty: true,
            playing: false,
            paused: false,
            track_list: vec![],
            volume: 0.05,
            auto_next_track: true,
            track_duration: 0,
            track_progress: 0,
            track_title: String::new(),
            track_artist: String::new(),
            track_album: String::new(),
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
        self.playing = true;
        self.paused = false;
        Ok(())
    }

    pub fn pause_audio(&mut self) -> AppResult<()> {
        self.playing = true;
        self.paused = true;
        Ok(())
    }

    pub fn stop_audio(&mut self) -> AppResult<()> {
        self.playing = false;
        self.paused = false;
        Ok(())
    }

    pub fn load_tracks(&mut self) -> AppResult<()> {
        let tracks = WalkDir::new("tracks")
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|e| {
                let path = e.path();
                if path.is_file() && 
                    (path.extension().map_or(false, |ext| 
                        ext.eq("mp3") || ext.eq("wav") || ext.eq("flac"))) {
                    Some(path.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        self.track_list = tracks;
        Ok(())
    }

    pub fn set_track_duration(&mut self, duration: u64) {
        self.track_duration = duration;
    }
    
}
