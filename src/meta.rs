use std::io::{self};
use std::path::Path;
use metaflac::Tag;


enum AudioFileType {
    FLAC,
    WAV,
    MP3,
}

pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

pub struct Meta {}

impl Meta {
    pub fn new() -> Self {
        Self {}
    }

    // Determine the file type based on the file extension
    fn detect_file_type<P: AsRef<Path>>(path: P) -> io::Result<AudioFileType> {
        match path.as_ref().extension().and_then(|s| s.to_str()) {
            Some("flac") => Ok(AudioFileType::FLAC),
            Some("wav") => Ok(AudioFileType::WAV),
            Some("mp3") => Ok(AudioFileType::MP3),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported file type")),
        }
    }
    
    pub fn get_audio_metadata(&self, path: &str) -> io::Result<AudioMetadata> {
        let file_type = Self::detect_file_type(path)?;
        match file_type {
            AudioFileType::FLAC => {
                let tag = Tag::read_from_path(path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                Ok(AudioMetadata {
                    title: tag.get_vorbis("TITLE").and_then(|mut iter| iter.next()).map(|s| s.to_string()),
                    artist: tag.get_vorbis("ARTIST").and_then(|mut iter| iter.next()).map(|s| s.to_string()),
                    album: tag.get_vorbis("ALBUM").and_then(|mut iter| iter.next()).map(|s| s.to_string()),
                })
            },
            AudioFileType::MP3 => {
                let metadata = mp3_metadata::read_from_file(path).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                Ok(AudioMetadata {
                    title: metadata.tag.as_ref().and_then(|tag| Some(tag.title.clone())),
                    artist: metadata.tag.as_ref().and_then(|tag| Some(tag.artist.clone())),
                    album: metadata.tag.as_ref().and_then(|tag| Some(tag.album.clone())),
                })
            },
            AudioFileType::WAV => {
                let filename = path.split('/').last().unwrap().to_string();
                Ok(AudioMetadata {
                    title: Some(filename.split('.').next().map(|s| s.to_string()).unwrap_or_else(|| String::new())),
                    artist: Some(String::new()),
                    album: Some(String::new()),
                })
            },
        }
    }
    
}
