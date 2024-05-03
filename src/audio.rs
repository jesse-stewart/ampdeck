use rodio::{Decoder, OutputStreamHandle, Sink};
use std::{fs::File, io::Read, io::Cursor, sync::Arc};
use tokio::sync::Mutex;

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
        let path = path.to_owned();
        let stream_handle = self.stream_handle.clone();
        let sink_clone = self.sink.clone();
    
        // Clear previous sink to ensure a clean start
        let mut sink_guard = sink_clone.lock().await;
        *sink_guard = None; // Reset sink
        let sink = sink_guard.get_or_insert_with(|| Sink::try_new(&stream_handle).unwrap());
    
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
    
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
        let mut sink_guard = sink_clone.lock().await;
        if let Some(sink) = sink_guard.as_mut() {
            sink.stop();
            *sink_guard = None; // Reset the sink to ensure it's reinitialized next play
        }
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
