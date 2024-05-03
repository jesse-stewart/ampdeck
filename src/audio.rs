use rodio::{Decoder, OutputStreamHandle, Sink};
use std::{fs::File, io::Read, io::Cursor, sync::Arc};
use tokio::{task, sync::Mutex};

pub struct Audio {
    stream_handle: OutputStreamHandle,
    sink: Arc<Mutex<Option<Sink>>>,
}

impl Audio {
    pub fn new(stream_handle: &OutputStreamHandle) -> Self {
        Self { 
            stream_handle: stream_handle.clone(),
            sink: Arc::new(Mutex::new(None)),
        }
    }

    // Asynchronously play the audio without blocking the UI
    pub async fn play(&self, path: &str) {
        let path = path.to_owned(); // Clone path to move into async block.
        let stream_handle = self.stream_handle.clone(); // Clone stream_handle.
        let sink_clone = self.sink.clone();
    
        let mut sink_guard = sink_clone.lock().await; // Correctly await the lock here
        let sink = sink_guard.get_or_insert_with(|| Sink::try_new(&stream_handle).unwrap());
    
        // Load file into memory
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        
        // Decode and play audio
        let cursor = Cursor::new(buffer);
        let source = Decoder::new(cursor).unwrap();
        sink.append(source);
    }
    
    pub async fn pause(&self) {
        let sink_clone = self.sink.clone();
        let mut sink_guard = sink_clone.lock().await; // Correctly await the lock here
        if let Some(sink) = sink_guard.as_mut() {
            sink.pause(); // Pause the current audio playback
        }
    }

    pub async fn resume(&self) {
        let sink_clone = self.sink.clone();
        let mut sink_guard = sink_clone.lock().await;
        if let Some(sink) = sink_guard.as_mut() {
            sink.play(); // Resume the audio playback
        }
    }

    pub async fn stop(&self) {
        let sink_clone = self.sink.clone();
        let mut sink_guard = sink_clone.lock().await; // Correctly await the lock here
        if let Some(sink) = sink_guard.as_mut() {
            sink.stop();
        }
    }
    
    pub async fn set_volume(&self, volume: f32) {
        let sink_clone = self.sink.clone();
        let mut sink_guard = sink_clone.lock().await; // Correctly await the lock here
        if let Some(sink) = sink_guard.as_mut() {
            sink.set_volume(volume);
        }
    }
    
}
