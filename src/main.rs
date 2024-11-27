use libmpv2::Mpv;
use log::error;
use log::info;
use log::warn;
use log::{debug, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use radiooooo::app::{App, AppResult};
use radiooooo::event::{Event, EventHandler};
use radiooooo::handler::handle_key_events;
use radiooooo::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    // logging
    let logfile = FileAppender::builder().build("log/requests.log").unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(logfile)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    // Create an `Mpv` and set some properties.
    let mpv = Mpv::with_initializer(|init| {
        init.set_property("vo", "null")?;
        Ok(())
    })
    .unwrap();

    // Create an application.
    let mut app = App::new(mpv);
    app.populate_countries_available();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // mpv.command(
    //     "loadfile",
    //     &["https://www.youtube.com/watch?v=VLnWf1sQkjY", "append-play"],
    // )
    // .unwrap();
    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
