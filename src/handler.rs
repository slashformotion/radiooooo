use crate::app::{App, AppResult, SelectedList};
use crate::radiooo::{self, Track};
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

        // play pause
        KeyCode::Char(' ') => app.playpause(),

        // navigate ui
        KeyCode::Down | KeyCode::Char('j') => match app.list_selected {
            SelectedList::Mood => app.mood_state.select_next(),
            SelectedList::Decade => {
                app.decade_state.select_next();

                app.update_country_available();
            }
            SelectedList::Country => app.country_state.select_next(),
        },
        KeyCode::Up | KeyCode::Char('k') => match app.list_selected {
            SelectedList::Mood => app.mood_state.select_previous(),
            SelectedList::Decade => {
                app.decade_state.select_previous();

                app.update_country_available();
            }
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
            let mood = radiooo::MOODS.get(app.mood_state.offset()).unwrap_or(&"");
            let decade = radiooo::DECADES
                .get(app.decade_state.offset())
                .unwrap_or(&0);
            let country = radiooo::COUNTRY_CODES
                .get(app.country_state.offset())
                .unwrap_or(&"");

            if let Some(track) = radiooo::get_track(mood, *decade, country) {
                app.current_setting = format!("{}", track.album)
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
