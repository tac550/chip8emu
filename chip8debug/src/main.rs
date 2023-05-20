mod app;
mod ui;

use std::{io, time::{Instant, Duration}, env};

use app::App;
use chip8exe::chip8_tick;
use crossterm::{self, terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, KeyCode, KeyModifiers, KeyEventKind}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal};

fn main() -> Result<(), io::Error> {
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("Chip8 Debugger");

    // check command line for rom file
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.get(1) {
        app.load_program(arg)?;
    }

    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = app.get_tick_rate()
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        match key.code {
                            KeyCode::Char('q') => app.should_quit = true,
                        _    => {},
                        }
                    } else {
                        match key.code {
                            KeyCode::Up => {
                                let ovr = app.mem_row_sel_override.get_or_insert(app.memory_state.selected().unwrap_or_default());
                                *ovr = ovr.saturating_sub(1);
                            },
                            KeyCode::Down => {
                                let ovr = app.mem_row_sel_override.get_or_insert(app.memory_state.selected().unwrap_or_default());
                                *ovr = ovr.saturating_add(1);
                            },
                            KeyCode::Char('s') => chip8_tick(&mut app.chip_state),
                            KeyCode::Char('f') => app.mem_row_sel_override = None,
                            _ => {},
                        }
                    }
                }
            }
        }

        if last_tick.elapsed() >= app.get_tick_rate() {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}