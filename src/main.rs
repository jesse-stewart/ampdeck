use ampdeck::app::{App, AppResult};
use ampdeck::audio::Audio;
use ampdeck::event::{Event, EventHandler};
use ampdeck::handler::handle_key_events;
use ampdeck::tui::Tui;
use rodio::OutputStream;
use tokio::runtime::Runtime;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

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

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                let previous_index = app.track_index;
                handle_key_events(key_event, &mut app)?;
                // Determine if track has changed
                if app.track_index != previous_index {
                    // Track has changed: stop previous and play new
                    audio.stop().await;
                    let path = app.track_list[app.track_index].to_string();
                    audio.play(&path).await;
                    app.is_paused = false; // Reset pause state when track changes
                } else if app.is_paused {
                    // If paused, no action needed unless pause toggle requested
                } else if !app.is_playing {
                    // If not playing and not paused, ensure audio is stopped
                    audio.stop().await;
                } else {
                    // Otherwise continue playing current track
                    let path = app.track_list[app.track_index].to_string();
                    audio.play(&path).await;
                }
            },
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {}
        }
         // Manage audio state based on current flags
        if app.is_playing && !app.is_paused {
            // Continue playing current track or resume if paused
            let path = app.track_list[app.track_index].to_string();
            audio.resume().await; // Resume plays from pause or continues if already playing
        } else if app.is_paused {
            // Pause audio
            audio.pause().await;
        } else {
            // Ensure audio is stopped
            audio.stop().await;
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
