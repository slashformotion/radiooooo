use std::{borrow::BorrowMut, vec};

use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{Color, Style},
    widgets::*,
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};
use serde_json::to_string;

use crate::app::{App, PlayState};
use crate::radiooo;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // master layout
    let master_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Fill(10),
            Constraint::Length(1),
        ])
        .split(frame.size());

    // header horizontal layout
    let header_layout = Layout::horizontal(vec![
        Constraint::Fill(1),
        Constraint::Fill(10),
        Constraint::Fill(1),
    ])
    .split(master_layout[0]);

    let body_layout = Layout::horizontal(vec![Constraint::Fill(1), Constraint::Percentage(30)])
        .split(master_layout[1]);

    /////////////////////////////////////
    // HEADER
    /////////////////////////////////////

    // play state
    frame.render_widget(
        render_play_state(app.play_state)
            .alignment(Alignment::Left)
            .block(Block::new().padding(Padding::left(1))),
        header_layout[0],
    );

    // request state
    frame.render_widget(
        Paragraph::new(app.current_setting.as_str())
            .alignment(Alignment::Center)
            .block(Block::new().padding(Padding::right(1))),
        header_layout[1],
    );

    // volume
    frame.render_widget(
        Paragraph::new(render_volume_header_span(app.volume, app.muted))
            .alignment(Alignment::Right)
            .block(Block::new().padding(Padding::right(1))),
        header_layout[2],
    );

    /////////////////////////////////////
    // body
    /////////////////////////////////////

    let lists_layout = Layout::horizontal(vec![
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .split(body_layout[0]);

    frame.render_stateful_widget(
        List::new(radiooo::DECADES.map(|n| n.to_string()))
            .block(Block::bordered().title("Decades"))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        lists_layout[1],
        app.decade_state.borrow_mut(),
    );

    frame.render_stateful_widget(
        List::new(radiooo::MOODS)
            .block(Block::bordered().title("Moods"))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        lists_layout[0],
        app.mood_state.borrow_mut(),
    );

    frame.render_stateful_widget(
        List::new(app.get_countries_available())
            .block(Block::bordered().title("Countries"))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        lists_layout[2],
        app.country_state.borrow_mut(),
    );

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::bordered()
                    .title("Title Infos")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        body_layout[1],
    );

    /////////////////////////////////////
    // footer
    /////////////////////////////////////
    frame.render_widget(Paragraph::new("shortcuts").centered(), master_layout[2])
}

fn render_play_state(p: PlayState) -> Paragraph<'static> {
    // .fg(Color::from_u32(233))
    // .bg(Color::from_str("#FF5F87")
    //     .expect("this is a valid color that should be recognised at runtime"))

    match p {
        PlayState::Paused => Paragraph::new("Paused"),
        PlayState::Playing => Paragraph::new("Playing"),
        PlayState::Stopped => Paragraph::new("Stopped"),
    }
}

fn render_volume_header_span(volume: u16, muted: bool) -> Line<'static> {
    if muted {
        Line::from(vec![
            "muted".red(),
            " (".into(),
            Span::styled(format!("{}%", volume), Style::default().crossed_out()),
            ")".into(),
        ])
    } else {
        Line::from(vec![Span::styled(format!("{}%", volume), Style::default())])
    }
}
