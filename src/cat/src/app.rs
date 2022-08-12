use std::error;
use std::io::BufRead;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Paragraph, Widget};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub file_name: String,
    pub data: String,
}

impl Default for App {
    fn default() -> Self {
        let data = std::io::stdin().lock().lines().next().unwrap().unwrap();
        Self { running: true, file_name: "STDIN".to_string(), data }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(file_name: Option<String>) -> Self {
        match file_name {
            None => { Self::default() }
            Some(file_name) => {
                let data = std::fs::read_to_string(&file_name).unwrap();
                Self { running: true,
                    file_name,
                    data }
            }
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples
        frame.render_widget(
            Paragraph::new(format!("{}\n\n{}", self.file_name.as_str(), self.data))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left),
            frame.size(),
        )
    }
}
