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

use crate::app::{self, App, PlayState};
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

    let body_constraints = match app.play_state {
        PlayState::Paused(_) => vec![Constraint::Fill(1)],
        PlayState::Playing(_) => vec![Constraint::Fill(1), Constraint::Percentage(40)],
        PlayState::Stopped => vec![Constraint::Fill(1)],
    };
    let body_layout = Layout::horizontal(body_constraints).split(master_layout[1]);

    /////////////////////////////////////
    // HEADER
    /////////////////////////////////////

    // play state
    frame.render_widget(
        render_play_state(&app.play_state)
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
        List::new(radiooo::MOODS)
            .block(
                get_block_style_selector(app.list_selected, app::SelectedList::Mood).title("Moods"),
            )
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        lists_layout[0],
        app.mood_state.borrow_mut(),
    );

    frame.render_stateful_widget(
        List::new(radiooo::DECADES.map(|n| n.to_string()))
            .block(
                get_block_style_selector(app.list_selected, app::SelectedList::Decade)
                    .title("Decades"),
            )
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        lists_layout[1],
        app.decade_state.borrow_mut(),
    );

    frame.render_stateful_widget(
        List::new(app.get_countries_available())
            .block(
                get_block_style_selector(app.list_selected, app::SelectedList::Country)
                    .title("Countries"),
            )
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        lists_layout[2],
        app.country_state.borrow_mut(),
    );

    match &app.play_state {
        PlayState::Paused(_) | PlayState::Playing(_) => {
            frame.render_widget(
                Table::new(
                    vec![
                        Row::new(vec!["ddd", "dd"]),
                        Row::new(vec!["dqsdqdsqd", "qsdqd"]),
                    ],
                    vec![Constraint::Fill(1), Constraint::Fill(1)],
                )
                .block(
                    Block::bordered()
                        .title("Title Infos")
                        .title_alignment(Alignment::Left),
                )
                .style(Style::default()),
                body_layout[1],
            );
        }
        PlayState::Stopped => {}
    }

    /////////////////////////////////////
    // footer
    /////////////////////////////////////
    frame.render_widget(Paragraph::new("shortcuts").centered(), master_layout[2])
}

fn render_play_state(p: &PlayState) -> Paragraph<'static> {
    // .fg(Color::from_u32(233))
    // .bg(Color::from_str("#FF5F87")
    //     .expect("this is a valid color that should be recognised at runtime"))

    match p {
        PlayState::Paused(_) => Paragraph::new("Paused"),
        PlayState::Playing(_) => Paragraph::new("Playing"),
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

fn get_block_style_selector(
    current_state: app::SelectedList,
    target_block: app::SelectedList,
) -> Block<'static> {
    if current_state == target_block {
        return Block::bordered().border_style(Color::Red);
    }
    return Block::bordered();
}
