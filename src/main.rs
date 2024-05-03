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
use ampdeck::meta::AudioMetadata;

fn main() -> AppResult<()> {
    let rt = Runtime::new()?;
    rt.block_on(async_main())
}




async fn async_main() -> AppResult<()> {
    // Audio setup
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

    let mut previous_index = app.track_index;

    // load all the tracks in the the tracks directory
    app.load_tracks()?;

    // Start the main loop.
    while app.running {
        audio.set_volume(app.volume).await;
        app.sink_empty = audio.is_sink_empty().await;

        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                app.tick();
                let progress = audio.get_track_progress().await;
                if let Some(progress) = progress {
                    app.track_progress = progress.as_secs();
                }
                if app.playing && !app.paused && app.sink_empty && app.track_progress > 0 {
                    audio.stop().await;
                    let next_track = app.track_index + 1;
                    if next_track < app.track_list.len() {
                        app.track_index = next_track;
                        audio.play(&app.track_list[app.track_index]).await;
                    }
                }
            
            }
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app)?;

                let meta = Meta::new();
                match meta.get_audio_metadata(&app.track_list[app.track_index]) {
                    Ok(metadata) => {
                        if let Some(title) = &metadata.title {
                            app.track_title = title.to_string();
                        }
                        if let Some(artist) = &metadata.artist {
                            app.track_artist = artist.to_string();
                        }
                        if let Some(album) = &metadata.album {
                            app.track_album = album.to_string();
                        }
                    },
                    Err(_e) => (),
                }

                // Determine if track has changed
                if app.track_index != previous_index && app.playing {
                    // Stop the current audio
                    audio.stop().await;
                    // Play the new track
                    audio.play(&app.track_list[app.track_index]).await;
                    previous_index = app.track_index; // Update previous index to the new value
                } else if app.playing && app.paused {
                    // If playing and paused, no action needed unless pause toggle requested
                    audio.pause().await;
                } else if !app.playing && !app.paused {
                    // If not playing and not paused, ensure audio is stopped
                    audio.stop().await;
                } else if app.playing && !app.paused && app.sink_empty {
                    audio.play(&app.track_list[app.track_index]).await;
                } else {
                    audio.resume().await;
                }
            },
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {}
        }

    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
