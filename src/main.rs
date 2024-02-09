use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
        KeyModifiers
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    fmt,
    io,
};
use rand::{Rng};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

fn color_palette(color: &str) -> tui::style::Color {
    match color {
        "bg" => Color::Black,
        "fg" => Color::White,
        "barbg" => Color::Blue,
        "red" => Color::Red,
        _ => Color::LightRed,
    }
}

enum Mode {
    Normal,
    Insert,
}
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "normal"),
            Mode::Insert => write!(f, "insert"),
        }
    }
}

struct App {
    mode: Mode,
    input_box: String,
    submission: String
}

impl Default for App {
    fn default() -> App {
        App {
            mode: Mode::Normal,
            input_box: String::new(),
            submission: String::new(),
        }
    }
}

fn delete_word(s: &mut String) -> String {
    let new_len = s.len() -s.split(' ').last().unwrap().len();
    s.truncate(new_len);
    String::from(s.trim())
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res { println!("{:?}", err) }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        // Key bindings
        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => { return Ok(()); }
                    KeyCode::Char('i') => { app.mode = Mode::Insert; }
                    _ => {}
                },
                Mode::Insert => match key.code {
                    KeyCode::Esc => { app.mode = Mode::Normal; }
                    KeyCode::Enter => {
                        if !app.input_box.trim().is_empty(){
                            app.submission = String::from(&app.input_box);
                            app.input_box.clear();
                        }
                    }
                    // Linux terminal detects Ctrl+Backspace as Ctrl-h
                    // So this captures it separately and filters if Ctrl is pressed
                    KeyCode::Char('h') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            app.input_box = delete_word(&mut app.input_box);
                        } else { app.input_box.push('h'); }
                    }
                    KeyCode::Char('w') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            app.input_box = delete_word(&mut app.input_box);
                        } else { app.input_box.push('h'); }
                    }
                    KeyCode::Char(c) => { app.input_box.push(c); }
                    KeyCode::Backspace => { app.input_box.pop(); }
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1), // Body
                Constraint::Length(1), // Bar
                Constraint::Length(1), // Input box
            ]
            .as_ref(),
        )
        .split(f.size());

    // Change for matrix
    let contents = Paragraph::new(
        format!("{submission}", submission=app.submission))
        .style(Style::default()
               .bg(color_palette("bg"))
               .fg(color_palette("fg"))
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim:true });
    f.render_widget(contents, chunks[0]);

    // Bar
    let ltext = format!( " {mode} <x> <y> <filled char> <bg char>", mode=app.mode);
    let rtext = format!( "<filled char> <bg char> <x> <y> ");
    let lbar = Paragraph::new(ltext.clone())
        .style(
            Style::default()
                .bg(color_palette("barbg"))
                .fg(color_palette("fg"))
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(lbar, chunks[1]);
    let rbar = Paragraph::new(rtext.clone())
        .style(
            Style::default()
                .bg(color_palette("barbg"))
                .fg(color_palette("fg"))
        )
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });
    f.render_widget(rbar, chunks[1]);

    match app.mode {
        Mode::Normal => {}
        Mode::Insert => {
            f.set_cursor(
                // Put the cursor past the end of the input text
                chunks[2].x + app.input_box.width() as u16,
                chunks[2].y,
            )
        }
    }

    let input_box = Paragraph::new(app.input_box.as_ref())
        .style(
            Style::default()
           .bg(color_palette("bg"))
           .fg(color_palette("fg"))
        );
    f.render_widget(input_box, chunks[2]);
}
