use std::borrow::Borrow;

use crate::app::{App, AppResult, PlayState, SelectedList};
use crate::radiooo::{self, Track};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::{debug, info};

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

        // play pause
        KeyCode::Char(' ') => app.playpause(),

        // navigate ui
        KeyCode::Down | KeyCode::Char('j') => match app.list_selected {
            SelectedList::Mood => app.mood_state.select_next(),
            SelectedList::Decade => app.decade_state.select_next(),
            SelectedList::Country => app.country_state.select_next(),
        },
        KeyCode::Up | KeyCode::Char('k') => match app.list_selected {
            SelectedList::Mood => app.mood_state.select_previous(),
            SelectedList::Decade => app.decade_state.select_previous(),
            SelectedList::Country => app.country_state.select_previous(),
        },
        KeyCode::Left | KeyCode::Char('h') => match app.list_selected {
            SelectedList::Country => app.list_selected = SelectedList::Decade,
            SelectedList::Decade => app.list_selected = SelectedList::Mood,
            SelectedList::Mood => app.list_selected = SelectedList::Country,
        },
        KeyCode::Right | KeyCode::Char('l') => match app.list_selected {
            SelectedList::Country => app.list_selected = SelectedList::Mood,
            SelectedList::Decade => app.list_selected = SelectedList::Country,
            SelectedList::Mood => app.list_selected = SelectedList::Decade,
        },
        // VOLUME
        KeyCode::Char('+') | KeyCode::Char('*') => {
            app.increment_volume();
        }
        KeyCode::Char('-') | KeyCode::Char('/') => {
            app.decrement_volume();
        }
        KeyCode::Char('m') => {
            app.toggle_mute();
        }
        KeyCode::Enter => {
            let indexmoode = app.mood_state.selected().unwrap_or(0);
            let indexdecade = app.decade_state.selected().unwrap_or(0);
            let indexcountry = app.country_state.selected().unwrap_or(0);
            debug!("indexes: {} {} {}", indexmoode, indexdecade, indexcountry);
            let mood = radiooo::MOODS.get(indexmoode).unwrap_or(&"");
            let decade = radiooo::DECADES.get(indexdecade).unwrap();
            let country = app
                .get_countries_available()
                .get(indexcountry)
                .unwrap()
                .clone();
            let opt = radiooo::get_track(mood, *decade, country.as_str());
            if let Some(track) = opt {
                info!("{:?}", track);
                app.current_setting = format!(
                    "{} - {} - {}",
                    track.title,
                    track.artist,
                    track.album.unwrap_or_default()
                );
                app.mpv
                    .command("loadfile", &[track.links.mpeg.as_str(), "replace"])
                    .unwrap();
                app.play_state = PlayState::Playing(track.clone());
            } else {
                app.current_setting = format!("{}", "no track was found for current setting")
            }
        }

        // next
        KeyCode::Char('n') => todo!("next"),
        KeyCode::Char('p') => todo!("p"),
        _ => {}
    }
    Ok(())
}
