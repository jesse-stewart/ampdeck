use std::io::{self, Read};
use std::fs::File;
use std::io::Cursor;
use log::{error, info};
use rodio::{Decoder, Sink, OutputStreamHandle};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Instant, Duration};

pub struct Audio {
    stream_handle: OutputStreamHandle,
    sink: Arc<Mutex<Option<Sink>>>,
    start_time: Arc<Mutex<Option<Instant>>>,
    accumulated_duration: Arc<Mutex<Duration>>, // Accumulated playback time
}

impl Audio {
    pub fn new(stream_handle: &OutputStreamHandle) -> Self {
        Self { 
            stream_handle: stream_handle.clone(),
            sink: Arc::new(Mutex::new(None)),
            start_time: Arc::new(Mutex::new(None)),
            accumulated_duration: Arc::new(Mutex::new(Duration::from_secs(0))), // Initialize to 0
        }
    }

    pub async fn play(&self, path: &str, volume: f32) -> Result<(), io::Error> {
        let path = path.to_owned();
        let stream_handle = self.stream_handle.clone();
        let sink_clone = self.sink.clone();
        let start_time_clone = self.start_time.clone();
    
        let mut sink_guard = sink_clone.lock().await;
        *sink_guard = None; // Reset sink
        let sink = sink_guard.get_or_insert_with(|| Sink::try_new(&stream_handle).expect("Failed to create audio sink"));
        sink.set_volume(volume);
    
        let file = File::open(&path);
        match file {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_err() {
                    error!("Failed to read audio file: {}", path);
                    return Err(io::Error::new(io::ErrorKind::Other, "Failed to read audio file"));
                }
    
                let cursor = Cursor::new(buffer);
                match Decoder::new(cursor) {
                    Ok(source) => {
                        sink.append(source);
                        *start_time_clone.lock().await = Some(Instant::now()); // Set the start time when playback begins
                        *self.accumulated_duration.lock().await = Duration::from_secs(0); // Reset accumulated time on new play
                        info!("Started playing: {}", path);
                        Ok(())
                    },
                    Err(_) => {
                        error!("Failed to decode audio file: {}", path);
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to decode audio"));
                    }
                }
            },
            Err(e) => {
                error!("Error opening file: {} with error: {:?}", path, e);
                Err(e)
            }
        }
    }
    
    

    pub async fn pause(&self) {
        let sink_clone = self.sink.clone();
        let start_time_clone = self.start_time.clone();
        let accumulated_duration_clone = self.accumulated_duration.clone();
        
        let mut sink_guard = sink_clone.lock().await;
        if let Some(sink) = sink_guard.as_mut() {
            sink.pause();
            // Calculate and accumulate the elapsed duration
            let mut acc_duration_guard = accumulated_duration_clone.lock().await;
            if let Some(start_time) = start_time_clone.lock().await.take() {
                *acc_duration_guard += start_time.elapsed();
            }
        }
    }
    
    pub async fn resume(&self) {
        let sink_clone = self.sink.clone();
        let start_time_clone = self.start_time.clone();
        
        let mut sink_guard = sink_clone.lock().await;
        if let Some(sink) = sink_guard.as_mut() {
            sink.play();
            // Reset the start time when resuming
            *start_time_clone.lock().await = Some(Instant::now());
        }
    }
    
    pub async fn stop(&self) {
        let sink_clone = self.sink.clone();
        let start_time_clone = self.start_time.clone();
        let accumulated_duration_clone = self.accumulated_duration.clone();
        
        let mut sink_guard = sink_clone.lock().await;
        if let Some(sink) = sink_guard.as_mut() {
            sink.stop();
            *sink_guard = None; // Reset the sink to ensure it's reinitialized next play
        }
        // Reset both the start time and the accumulated duration
        *start_time_clone.lock().await = None;
        *accumulated_duration_clone.lock().await = Duration::from_secs(0);
    }
    
    
    pub async fn elapsed_time(&self) -> Duration {
        let acc_duration_guard = self.accumulated_duration.lock().await;
        let additional_time = if let Some(start_time) = &*self.start_time.lock().await {
            start_time.elapsed()
        } else {
            Duration::from_secs(0)
        };
        *acc_duration_guard + additional_time
    }
    
    
    pub async fn set_volume(&self, volume: f32) {
        let sink_clone = self.sink.clone();
        let mut sink_guard = sink_clone.lock().await;
        if let Some(sink) = sink_guard.as_mut() {
            sink.set_volume(volume);
        }
    }


    pub async fn is_sink_empty(&self) -> bool {
        let sink_clone = self.sink.clone();
        let sink_guard = sink_clone.lock().await;
        if let Some(sink) = sink_guard.as_ref() {
            sink.empty()
        } else {
            true 
        }
    }

}
