use std::{
    fs::read_to_string,
    io::{self, stdout},
    path::Path,
};

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

use copypasta::{ClipboardContext, ClipboardProvider};
use fpicker::{File, FileExplorer, Theme};

fn main() -> io::Result<()> {
    // grab first arg as path
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    // change cwd to path
    std::env::set_current_dir(&path).unwrap();
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let layout = Layout::vertical([
        Constraint::Ratio(4, 5), // Main file explorer and content area
        Constraint::Ratio(1, 5), // Bottom window for selected file paths
    ]);

    // Inner layout for the top section
    let top_layout = Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);

    // Create a new file explorer with the default theme and title.
    let theme = get_theme();
    let mut file_explorer = FileExplorer::with_theme(theme)?;

    loop {
        // Get the content of the current selected file (if it's indeed a file).
        let file_content = get_file_content(file_explorer.current().path())?;
        let selected_files: String = file_explorer
            .selected_files()
            .iter()
            .map(|file| file.path().display().to_string())
            .collect::<Vec<String>>()
            .join("\n");

        // Render the file explorer widget, file content, and selected file paths.
        terminal.draw(|f| {
            let chunks = layout.split(f.area());
            let top_chunks = top_layout.split(chunks[0]);

            // Top section
            f.render_widget(&file_explorer.widget(), top_chunks[0]);
            f.render_widget(
                Paragraph::new(file_content).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                ),
                top_chunks[1],
            );

            // Bottom section
            f.render_widget(
                Paragraph::new(selected_files.clone()).block(
                    Block::default().borders(Borders::ALL).title(
                        "<q> Quit | <c> Copy Files to Clipboard | <p> Copy Paths to Clipboard",
                    ),
                ),
                chunks[1],
            );
        })?;

        // Read the next event from the terminal.
        let event = read()?;
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => break, // Quit the application
                KeyCode::Char('c') => {
                    // Copy selected file paths to clipboard
                    if let Ok(mut clipboard) = ClipboardContext::new() {
                        if let Err(err) = clipboard.set_contents(get_selected_files_content(
                            &file_explorer.selected_files(),
                        )) {
                            eprintln!("Failed to copy to clipboard: {}", err);
                        }
                    } else {
                        eprintln!("Clipboard not available.");
                    }
                }
                KeyCode::Char('p') => {
                    // Copy selected file paths to clipboard
                    if let Ok(mut clipboard) = ClipboardContext::new() {
                        if let Err(err) = clipboard.set_contents(selected_files.clone()) {
                            eprintln!("Failed to copy to clipboard: {}", err);
                        }
                    } else {
                        eprintln!("Clipboard not available.");
                    }
                }
                _ => {}
            }
        }
        // Handle the event in the file explorer.
        file_explorer.handle(&event)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    // Print the selected file paths to stdout.
    for file in file_explorer.selected_files() {
        println!("{}", file.path().display());
    }
    Ok(())
}

fn get_file_content(path: &Path) -> io::Result<String> {
    let mut content = String::new();

    // If the path is a file, read its content.
    if path.is_file() {
        content = match read_to_string(path) {
            Ok(content) => content,
            Err(err) => format!("Unable to read file: {}", err),
        };
    }

    Ok(content)
}
fn get_selected_files_content(selected_files: &Vec<File>) -> String {
    selected_files
        .iter()
        .map(|file| {
            let path = file.path().display().to_string();
            let content = get_file_content(file.path())
                .unwrap_or_else(|_| "Unable to read file.".to_string());
            format!("File: {}\nContent:\n{}\n", path, content)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_theme() -> Theme {
    Theme::default()
        .with_block(Block::default().borders(Borders::ALL))
        .with_dir_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .with_highlight_dir_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        )
}
