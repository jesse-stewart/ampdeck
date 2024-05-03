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
                Play/Pause with `Spacebar`.\n\
                Stop with `s`.\n\
                 \n\
                index: {}\n\
                Is Playing: {}\n\
                is Paused: {}",
            app.track_index,
            app.is_playing,
            app.is_paused
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
