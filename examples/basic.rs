use std::io::{self, stdout};
use std::process;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use fpicker::{FileExplorer, Theme};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Create a new file explorer with the default theme and title.
    let theme = Theme::default().add_default_title();
    let mut file_explorer = FileExplorer::with_theme(theme)?;

    let mut selected_paths = vec![];

    loop {
        // Render the file explorer widget.
        terminal.draw(|f| {
            f.render_widget(&file_explorer.widget(), f.area());
        })?;

        // Read the next event from the terminal.
        let event = read()?;
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                // Collect selected file paths.
                selected_paths = file_explorer
                    .selected_files()
                    .iter()
                    .map(|file| file.path().display().to_string())
                    .collect();
                break;
            }
        }
        // Handle the event in the file explorer.
        file_explorer.handle(&event)?;
    }

    // Restore the terminal to normal mode.
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    // Print the selected file paths to stdout.
    for path in selected_paths {
        println!("{}", path);
    }

    // Return exit code 0 explicitly.
    process::exit(0);
}
