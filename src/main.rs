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

    let mut previous_index = app.track_index;

    // Start the main loop.
    while app.running {
        audio.set_volume(app.volume).await;

        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app)?;

                // Determine if track has changed
                if app.track_index != previous_index {
                    // Stop the current audio
                    audio.stop().await;
                    // Play the new track
                    audio.play(&app.track_list[app.track_index]).await;
                    previous_index = app.track_index; // Update previous index to the new value
                } else if app.is_playing && app.is_paused {
                    // If playing and paused, no action needed unless pause toggle requested
                    audio.pause().await;
                } else if !app.is_playing && !app.is_paused {
                    // If not playing and not paused, ensure audio is stopped
                    audio.stop().await;
                } else if app.is_playing && !app.is_paused && audio.is_sink_empty().await {
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
