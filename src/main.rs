use ampdeck::logger::setup_logging;
use tokio::runtime::Runtime;
use rodio::OutputStream;
use ampdeck::app::{App, AppResult};
use ampdeck::audio::Audio;
use ampdeck::event::{Event, EventHandler};
use ampdeck::handler::handle_key_events;
use ampdeck::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ampdeck::meta::Meta;
use serde::Deserialize;
use config::{Config as Configuration, File, FileFormat, ConfigError};

#[derive(Debug, Deserialize)]
struct Config {
    music_directory: String,
}

fn load_config() -> Result<Config, ConfigError> {
    Configuration::builder()
        .add_source(File::new("./Settings.toml", FileFormat::Toml))
        .build()?
        .try_deserialize::<Config>()
}

fn main() -> AppResult<()> {
    setup_logging().unwrap();
    let cfg = load_config().unwrap();
    let rt = Runtime::new()?;
    rt.block_on(async_main(cfg))
}

async fn async_main(cfg: Config) -> AppResult<()> {

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let audio = Audio::new(&stream_handle);

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // load all the tracks in the the tracks directory
    app.load_tracks(cfg.music_directory.as_str())?;

    // Start the main loop.
    while app.running {
        let progress = audio.elapsed_time().await;
        app.track_progress = progress.as_secs();
        
        let meta = Meta::new();
        if let Ok(metadata) = meta.get_audio_metadata(&app.track_list[app.track_index]) {
            app.track_file = metadata.file.unwrap_or_default();
            app.track_title = metadata.title.unwrap_or_default();
            app.track_artist = metadata.artist.unwrap_or_default();
            app.track_album = metadata.album.unwrap_or_default();
        }

        app.check_and_advance_track(&audio).await;

        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                app.tick();
            }
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app, &audio).await?;
            },
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        }
        
        // Update sink_empty state after handling events
        app.sink_empty = audio.is_sink_empty().await;

    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
