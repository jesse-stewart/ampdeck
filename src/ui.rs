use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::{app::App};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Define the layout constraints for each section
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1) // optional: adds a margin around the whole layout
        .constraints([
            Constraint::Length(2), // Height for the title bar
            Constraint::Length(3), // Height for the artist bar
            Constraint::Min(0),    // Remaining space for the main content
            Constraint::Length(3), // Height for the status bar
        ])
        .split(frame.size());

    // Split the stutus area into 5 columns
    let title_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Max(13)])
        .split(chunks[0]);

    // Split the main content area into 2 columns
    let main_content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Max(20)])
        .split(chunks[2]);

    // Split the stutus area into 5 columns
    let volume_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Min(1), Constraint::Max(3)])
        .split(main_content_chunks[1]);

    // Split the stutus area into 5 columns
    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(chunks[3]);

    // Create the title bar widget
    let title_bar = Paragraph::new(format!(
        "{}\n",
        app.track_title
    ))
    .block(
        Block::default()
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White).bg(Color::Blue))
            .padding(Padding::new(2, 2, 1, 0)),
    ).bold();

    // Create the title bar widget
    let artist_bar = Paragraph::new(format!(
        "{}\n{}\n",
        app.track_artist, app.track_album
    ))
    .block(
        Block::default()
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White).bg(Color::Blue))
            .padding(Padding::new(2, 2, 0, 1)),
    );

    // Create the title bar widget
    let logo = Paragraph::new("[AmpDeck]")
    .block(
        Block::default()
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White).bg(Color::Blue))
            .padding(Padding::new(2, 2, 1, 0)),
    ).bold();

    // combine the title and details widgets into a single widget called title_bar

    // Create the main content widgets for each column
    let main_content_1 = Paragraph::new(format!(
        "Track: {}\nIndex: {}/{}    Progress: {} s   Sink Empty: {}",
        app.track_file,
        app.track_index,
        app.track_list.len(),
        app.track_progress,
        app.sink_empty,
    ))
    .block(
        Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 0, 0)),
    );

    let volume_content_1 = Paragraph::new("[↑] Volume Up")
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL),
        )
        .style(if app.volume == 1.00 {
            Style::default().fg(Color::Red)
        } else if app.volume == 0.00 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::Blue)
        })
        .alignment(Alignment::Center);

    fn volume_string(volume: f32) -> String {
        // return a string of volume * 100 and %. If 0, return "Muted".
        if volume == 0.00 {
            return "Muted".to_string();
        }
        format!("{:.0}%", volume * 100.0)
    }

    let volume_content_2 = Paragraph::new(format!("Volume: {}", volume_string(app.volume)))
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .style(if app.volume == 1.00 {
            Style::default().fg(Color::Red)
        } else if app.volume == 0.00 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::Blue)
        })
        .alignment(Alignment::Center);

    let volume_content_3 = Paragraph::new("[↓] Volume Down")
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL),
        )
        .style(if app.volume == 1.00 {
            Style::default().fg(Color::Red)
        } else if app.volume == 0.00 {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::Blue)
        })
        .alignment(Alignment::Center);

    let status_content_1 = Paragraph::new("[←] Prev")
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .alignment(Alignment::Center);

    let status_content_2 = Paragraph::new("[Space] Play")
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .style(if app.playing && !app.paused {
            Style::default().fg(Color::Green)
        } else if app.playing && app.paused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        })
        .alignment(Alignment::Center);

    let status_content_3 = Paragraph::new("[S] Stop")
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .style(if !app.playing && !app.paused {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        })
        .alignment(Alignment::Center);

    let status_content_4 = Paragraph::new("[→] Next")
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .alignment(Alignment::Center);

    let status_content_5 = Paragraph::new("[L] Loop")
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .style(if app.loop_playlist {
            Style::default().fg(Color::Blue)
        } else {
            Style::default()
        })
        .alignment(Alignment::Center);

    // Render each widget in its respective area
    frame.render_widget(title_bar, title_chunks[0]);
    frame.render_widget(logo, title_chunks[1]);
    frame.render_widget(artist_bar, chunks[1]);
    frame.render_widget(main_content_1, main_content_chunks[0]);
    frame.render_widget(volume_content_1, volume_chunks[0]);
    frame.render_widget(volume_content_2, volume_chunks[1]);
    frame.render_widget(volume_content_3, volume_chunks[2]);
    frame.render_widget(status_content_1, status_chunks[0]);
    frame.render_widget(status_content_2, status_chunks[1]);
    frame.render_widget(status_content_3, status_chunks[2]);
    frame.render_widget(status_content_4, status_chunks[3]);
    frame.render_widget(status_content_5, status_chunks[4]);

}
