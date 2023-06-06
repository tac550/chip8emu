mod app;
mod ui;

use std::{io, time::{Instant, Duration}, env, panic, any::Any};

use app::{App, Failure};
use crossterm::{self, terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, KeyCode, KeyModifiers, KeyEventKind}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal};

fn main() -> Result<(), io::Error> {
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(None);

    load_rom_cmdl(&mut app)?;

    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    Ok(())
}

fn load_rom_cmdl(app: &mut App) -> io::Result<()> {
    // check command line for rom file
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.get(1) {
        app.load_program(arg)?;
    }
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
                            KeyCode::Char('r') => {
                                app.reset();
                                load_rom_cmdl(&mut app)?;
                            },
                            _ => {},
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
                            KeyCode::Char('s') => app = try_tick(app)?,
                            KeyCode::Char('f') => app.mem_row_sel_override = None,
                            KeyCode::Char('u') => app.inc_tick_rate(),
                            KeyCode::Char('j') => app.dec_tick_rate(),
                            KeyCode::Char('p') => app.pause_tick(),
                            _ => {},
                        }
                    }
                }
            }
        }

        if last_tick.elapsed() >= app.get_tick_rate() {
            app = try_tick(app)?;
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn try_tick(app: App) -> Result<App, io::Error> {
    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {
        // do nothing
    }));
    let last_instr_count = app.instr_count;
    let app = panic::catch_unwind(|| Ok(app.on_tick())).unwrap_or_else(|panic| {
        let mut new_app = App::new(Some(Failure { panic_message: format!("{:?}", display_caught_panic(&panic)), last_instr_count }));
        load_rom_cmdl(&mut new_app)?;
        Ok(new_app)
    });
    panic::set_hook(old_hook);

    app
}

fn display_caught_panic(panic: &Box<dyn Any + Send>) -> String {
    if let Some(msg) = panic.downcast_ref::<&'static str>() {
        String::from(*msg)
    } else if let Some(msg) = panic.downcast_ref::<String>() {
        String::from(msg)
    } else {
        String::from("Unknown Error")
    }
}