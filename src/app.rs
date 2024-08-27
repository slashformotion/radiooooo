use ratatui::widgets::*;
use std::error;

use crate::radiooo::{self, CountryForDecade};
const MAX_VOLUME: u16 = 150;
const VOLUME_INCREMENT: u16 = 5;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayState {
    Paused,
    Playing,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedList {
    Country,
    Decade,
    Mood,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// volume
    pub volume: u16,
    pub muted: bool,

    pub play_state: PlayState,

    pub decade_state: ListState,
    pub mood_state: ListState,
    pub country_state: ListState,

    pub list_selected: SelectedList,

    pub current_setting: String,
    pub country_available: radiooo::CountryForDecade,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut decade_state = ListState::default();
        decade_state.select(Some(0));
        let mut mood_state = ListState::default();
        mood_state.select(Some(0));
        let mut country_state = ListState::default();
        country_state.select(Some(0));
        Self {
            volume: 50,
            running: true,
            muted: false,
            play_state: PlayState::Stopped,
            decade_state,
            mood_state,
            country_state,
            list_selected: SelectedList::Decade,
            current_setting: String::from(""),
            country_available: CountryForDecade::default(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn get_countries_available(&mut self) -> Vec<String> {
        let mood = radiooo::MOODS.get(self.mood_state.offset()).unwrap_or(&"");
        self.country_available
            .to_hash_map()
            .get(*mood)
            .expect("should find the hash map here")
            .to_vec()
    }

    pub fn update_country_available(&mut self) {
        let decade = radiooo::DECADES
            .get(self.decade_state.offset())
            .unwrap_or(&0);
        if let Some(ca) = radiooo::get_country_for_decade(*decade) {
            self.country_available = ca
        }
    }

    pub fn playpause(&mut self) {
        match self.play_state {
            PlayState::Playing => {
                // send pause to mpv here
                self.play_state = PlayState::Paused;
            }
            PlayState::Stopped | PlayState::Paused => {
                // run play here and change state
                self.play_state = PlayState::Playing;
            }
        }
    }
    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted
    }

    pub fn increment_volume(&mut self) {
        if let Some(res) = self.volume.checked_add(VOLUME_INCREMENT) {
            if res < MAX_VOLUME {
                self.volume = res;
            }
        }
    }

    pub fn decrement_volume(&mut self) {
        if let Some(res) = self.volume.checked_sub(VOLUME_INCREMENT) {
            self.volume = res;
        }
    }
}
