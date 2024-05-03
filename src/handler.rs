use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
            if app.is_paused || !app.is_playing {
                app.play_audio()?;
            } else {
                app.pause_audio()?;
            }
        }
        // Stop audio with `s`
        KeyCode::Char('s') => {
            app.stop_audio()?;
        }
        // track_index handlers
        KeyCode::Right => {
            app.increment_track_index();
        }
        KeyCode::Left => {
            app.decrement_track_index();
        }
        // Volume handlers
        KeyCode::Up => {
            app.increase_volume();
        }
        KeyCode::Down => {
            app.decrease_volume();
        }
        // print the key event for debugging
        _ => {
            println!("{:?}", key_event);
        }
        
    }
    Ok(())
}
