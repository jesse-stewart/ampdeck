use crate::{app::{App, AppResult}, audio::Audio};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App, audio: &Audio) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Play audio with spacebar
        KeyCode::Char(' ') => {
            if app.paused || !app.playing {
                app.play_audio(audio).await?;
            } else {
                app.pause_audio(audio).await?;
            }
        }
        // Stop audio with `s`
        KeyCode::Char('s') => {
            app.stop_audio(audio).await?;
        }
        // track_index handlers
        KeyCode::Right => {
            app.increment_track(audio).await;
        }
        KeyCode::Left => {
            app.decrement_track(audio).await;
        }
        // Volume handlers
        KeyCode::Up => {
            app.increase_volume(audio).await;
        }
        KeyCode::Down => {
            app.decrease_volume(audio).await;
        }
        // print the key event for debugging
        _ => {
            println!("{:?}", key_event);
        }
        
    }
    Ok(())
}
