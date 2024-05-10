use std::fs::{self, File};
use std::io::{self, Read};
use std::error;
use std::path::Path;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use crate::audio::Audio;
use crate::meta::Meta;
use log::{info, error};
use tokio::task;

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
    pub loop_playlist: bool,
    pub track_duration: u64,
    pub track_progress: u64,
    pub track_file: String,
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
            loop_playlist: true,
            track_duration: 0,
            track_progress: 0,
            track_file: String::new(),
            track_title: String::new(),
            track_artist: String::new(),
            track_album: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub track_index: usize,
    pub volume: f32,
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

    pub fn read_state() -> io::Result<AppState> {
        let mut contents = String::new();
        let file_path = "state.json";
        if !Path::new(file_path).exists() {
            let default_state = AppState { track_index: 0, volume: 0.05 };
            let serialized = serde_json::to_string(&default_state)?;
            fs::write(file_path, serialized)?;
        }
        File::open(file_path)?.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents).unwrap_or(AppState { track_index: 0, volume: 0.05 }))
    }
    
    pub  fn write_state(state: &AppState) -> io::Result<()> {
        let serialized = serde_json::to_string(state)?;
        fs::write("state.json", serialized)
    }

    pub async fn increment_track(&mut self, audio: &Audio) {
        audio.stop().await;
        if self.track_index < self.track_list.len() - 1 {
            self.track_index += 1;
        } else if self.loop_playlist {
            self.track_index = 0;
        }
        self.update_meta().await;
        let _ = App::write_state(&AppState { track_index: self.track_index, volume: self.volume });
        if self.playing {
            if let Err(_) = audio.play(&self.track_list[self.track_index], self.volume).await {
                Box::pin(self.increment_track(audio)).await;
            }
        }
        if self.paused {
            audio.pause().await;
        }
    }

    pub async fn decrement_track(&mut self, audio: &Audio) {
        audio.stop().await;
        if self.loop_playlist && self.track_index == 0 {
            self.track_index = self.track_list.len() - 1;
        } else if let Some(res) = self.track_index.checked_sub(1) {
            self.track_index = res;
        }
        self.update_meta().await;
        let _ = App::write_state(&AppState { track_index: self.track_index, volume: self.volume});
        if self.playing {
            if let Err(_) = audio.play(&self.track_list[self.track_index], self.volume).await {
                Box::pin(self.decrement_track(audio)).await;
            }
        }
        if self.paused {
            audio.pause().await;
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
        let _ = App::write_state(&AppState { track_index: self.track_index, volume: rounded_volume });
        self.set_volume(rounded_volume, &audio).await;
    }
    
    pub async fn decrease_volume(&mut self, audio: &Audio) {
        const VOLUME_DECREMENT: f32 = 0.01;
        let new_volume = (self.volume - VOLUME_DECREMENT).max(0.0); // Ensure volume doesn't go below 0.0
        let rounded_volume = (new_volume * 1000.0).round() / 1000.0; 
        let _ = App::write_state(&AppState { track_index: self.track_index, volume: rounded_volume });
        self.set_volume(rounded_volume, &audio).await;
    }

    pub async fn update_meta(&mut self) {
        // while we are at it, update the Settings.toml file with the current track index
        let path = self.track_list[self.track_index].clone();
        let meta = Meta::new();
    
        // Use spawn_blocking to perform the heavy I/O task in a separate thread
        let result = task::spawn_blocking(move || {
            meta.get_audio_metadata(&path)
        }).await.unwrap();  // handle errors appropriately
    
        if let Ok(metadata) = result {
            self.track_file = metadata.file.unwrap_or_default();
            self.track_title = metadata.title.unwrap_or_default();
            self.track_artist = metadata.artist.unwrap_or_default();
            self.track_album = metadata.album.unwrap_or_default();
        }
    }

    pub fn get_next_track(&mut self) -> AppResult<String> {
        let next_track_index = if self.loop_playlist && self.track_index == self.track_list.len() - 1 {
            0
        } else {
            self.track_index + 1
        };
        
        let next_track_path = &self.track_list[next_track_index];
        let meta = Meta::new();
    
        // Use spawn_blocking to perform the heavy I/O task in a separate thread
        let result = meta.get_audio_metadata(next_track_path);  // handle errors appropriately
    
        if let Ok(metadata) = result {
            self.track_title = metadata.title.unwrap_or_default();
            self.track_artist = metadata.artist.unwrap_or_default();
            self.track_album = metadata.album.unwrap_or_default();
        }
        
        Ok(self.track_title.clone())
    }

    pub async fn play_audio(&mut self, audio: &Audio) -> AppResult<()> {
        self.update_meta().await;
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

    pub async fn disable_loop_playlist(&mut self) {
        self.loop_playlist = !self.loop_playlist;
    }
    
    pub async fn check_and_advance_track(&mut self, audio: &Audio) {
        if self.playing && !self.paused && self.sink_empty && self.track_progress > 1 {
            audio.stop().await;
            let next_track = if self.loop_playlist && self.track_index == self.track_list.len() - 1 {
                0
            } else {
                self.track_index + 1
            };
            self.track_index = next_track;
            self.update_meta().await;
            let _ = App::write_state(&AppState { track_index: self.track_index, volume: self.volume });
            if let Err(_) = audio.play(&self.track_list[self.track_index], self.volume).await {
                self.increment_track(audio).await;
            }
        }
    }

    pub fn initialize_state(&mut self, continue_session: bool) {
        let state = App::read_state().unwrap();
        if continue_session {
            self.track_index = state.track_index;
        }
        self.volume = state.volume;
    }

    pub fn load_tracks(&mut self, folder_path: &str) -> Result<(), io::Error> {
        let mut tracks = vec![];
    
        for entry in WalkDir::new(folder_path)
            .sort_by(|a, b| a.file_name().cmp(b.file_name())) // Sort entries alphabetically by file name
        {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    error!("Error reading directory entry: {}", e);
                    continue; // Skip this entry and log the error
                }
            };
    
            let path = entry.path();
            if path.is_file() {
                let is_hidden = path.file_name()
                    .map(|name| name.to_string_lossy().starts_with('.'))
                    .unwrap_or(true); // Assume hidden if there's any issue getting the file name
    
                let valid_extension = path.extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("mp3") || ext.eq_ignore_ascii_case("wav") || ext.eq_ignore_ascii_case("flac"));
    
                if !is_hidden && valid_extension {
                    tracks.push(path.to_string_lossy().into_owned());
                }
            }
        }
    
        if tracks.is_empty() {
            error!("No valid tracks found in the specified folder.");
            return Err(io::Error::new(io::ErrorKind::NotFound, "No valid tracks found"));
        } else {
            info!("Loaded {} tracks from {}", tracks.len(), folder_path);
        }
    
        tracks.sort(); // Sort the track list alphabetically
        self.track_list = tracks;
    
        Ok(())
    }
    
    
    

    pub fn set_track_duration(&mut self, duration: u64) {
        self.track_duration = duration;
    }
    
}
