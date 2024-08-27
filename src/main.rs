// use libmpv2::Mpv;
use radiooooo::app::{App, AppResult};
use radiooooo::event::{Event, EventHandler};
use radiooooo::handler::handle_key_events;
use radiooooo::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // // Create an `Mpv` and set some properties.
    // let mpv = Mpv::with_initializer(|init| {
    //     init.set_property("vo", "null")?;
    //     Ok(())
    // })
    // .unwrap();

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
