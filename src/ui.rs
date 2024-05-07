use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!(
            "Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Switch track with `Left` and `Right` arrow keys.\n\
                Play/Pause with `Spacebar`. Adjust volume with 'Up' and 'Down' arrow keys.\n\
                Stop with `s`. Loop playlist with 'l'.\n\
                \n\
                title: {}\n\
                artist: {}\n\
                album: {}\n\
                index: {}\n\
                Playing: {}\n\
                Paused: {}\n\
                Volume: {}\n\
                loop_playlist: {}\n\
                track_duration: {}\n\
                track_progress: {}\n\
                file: {}\n\
                sink_empty: {}",
                app.track_title,
                app.track_artist,
                app.track_album,
                app.track_index,
                app.playing,
                app.paused,
                app.volume,
                app.loop_playlist,
                app.track_duration,
                app.track_progress,
                app.track_file,
                app.sink_empty
        ))
        .block(
            Block::bordered()
                .title("Ampdeck")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered(),
        frame.size(),
    )
}
