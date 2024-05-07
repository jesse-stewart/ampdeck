use std::io::{self};
use std::error;
use walkdir::WalkDir;
use crate::audio::Audio;

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

    pub async fn increment_track(&mut self, audio: &Audio) {
        audio.stop().await;
        if self.track_index < self.track_list.len() - 1 {
            self.track_index += 1;
            if self.playing {
                if let Err(_) = audio.play(&self.track_list[self.track_index], self.volume).await {
                    Box::pin(self.increment_track(audio)).await;
                }
            }
            if self.paused {
                audio.pause().await;
            }
        }
    }

    pub async fn decrement_track(&mut self, audio: &Audio) {
        audio.stop().await;
        if let Some(res) = self.track_index.checked_sub(1) {
            self.track_index = res;
            if self.playing {
                if let Err(_) = audio.play(&self.track_list[self.track_index], self.volume).await {
                    Box::pin(self.decrement_track(audio)).await;
                }
            }
            if self.paused {
                audio.pause().await;
            }
        }
    }

    pub async fn set_volume(&mut self, volume: f32, audio: &Audio) {
        self.volume = volume;
        audio.set_volume(volume).await;
    }

    pub async fn increase_volume(&mut self, audio: &Audio) {
        const VOLUME_INCREMENT: f32 = 0.01;
        let new_volume = (self.volume + VOLUME_INCREMENT).min(1.0); // Ensure volume doesn't exceed 1.0
        let rounded_volume = (new_volume * 1000.0).round() / 1000.0; 
        self.set_volume(rounded_volume, &audio).await;
    }
    
    pub async fn decrease_volume(&mut self, audio: &Audio) {
        const VOLUME_DECREMENT: f32 = 0.01;
        let new_volume = (self.volume - VOLUME_DECREMENT).max(0.0); // Ensure volume doesn't go below 0.0
        let rounded_volume = (new_volume * 1000.0).round() / 1000.0; 
        self.set_volume(rounded_volume, &audio).await;
    }

    pub async fn play_audio(&mut self, audio: &Audio) -> AppResult<()> {
        if self.sink_empty {
            if let Err(_) = audio.play(&self.track_list[self.track_index], self.volume).await {
                self.increment_track(audio).await;
            }
        } else {
            audio.resume().await;
        }
        self.playing = true;
        self.paused = false;
        Ok(())
    }

    pub async fn pause_audio(&mut self, audio: &Audio) -> AppResult<()> {
        audio.pause().await;
        self.paused = true;
        Ok(())
    }

    pub async fn stop_audio(&mut self, audio: &Audio) -> AppResult<()> {
        audio.stop().await;
        self.playing = false;
        self.paused = false;
        Ok(())
    }

    pub fn load_tracks(&mut self) -> Result<(), io::Error> {
        let mut tracks = WalkDir::new("tracks")
            .sort_by(|a, b| a.file_name().cmp(b.file_name())) // Sort entries alphabetically by file name
            .into_iter()
            .filter_map(|entry| entry.ok()) // Handle WalkDir errors here
            .filter_map(|e| {
                let path = e.path();
                if path.is_file() &&
                    path.file_name()
                        .map(|name| !name.to_string_lossy().starts_with('.'))
                        .unwrap_or(false) && // Check that the file does not start with '.'
                    path.extension()
                        .map_or(false, |ext| ext.eq_ignore_ascii_case("mp3") || ext.eq_ignore_ascii_case("wav") || ext.eq_ignore_ascii_case("flac")) {
                    Some(path.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        tracks.sort(); // Sort the track list alphabetically
        self.track_list = tracks;
        Ok(())
    }
    

    pub fn set_track_duration(&mut self, duration: u64) {
        self.track_duration = duration;
    }
    
}
