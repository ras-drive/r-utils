use std::io;
use clap::{Arg, Command};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use cat::app::{App, AppResult};
use cat::event::{Event, EventHandler};
use cat::handler::handle_key_events;
use cat::tui::Tui;

fn main() -> AppResult<()> {
    // Get Clap arguments.
    let matches = Command::new("wc")
        .version("1.0")
        .author("Sarah Petkovic")
        .about("print newline, word, and byte counts for each file")
        .arg(Arg::new("filename").required(false))
        .get_matches();

    // Create an application.
    let mut app: App;
    if matches.is_present("filename") {
        app = App::new(Some(matches.value_of("filename").unwrap().to_string()));
    } else {
        app = App::default();
    }

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
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
