mod app;
mod ui;

use std::{io, time::{Duration, UNIX_EPOCH, SystemTime, SystemTimeError}, env, panic, any::Any};

use app::{App, Failure, TIMER_RATE};
use crossterm::{self, terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, KeyCode, KeyModifiers, KeyEventKind, KeyEvent}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal};

fn main() -> io::Result<()> {
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(None);

    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("{err}");
    }

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
    load_rom_cmdl(&mut app)?;

    let mut last_tick = SystemTime::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = app.get_tick_rate()
            .checked_sub(last_tick.elapsed().unwrap_or_default())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char('r') => {
                            app.reset();
                            load_rom_cmdl(&mut app)?;
                        },
                        _ => {},
                    }
                } else if !process_chip8_input(key, &mut app) && key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => {
                            let ovr = app.mem_row_sel_override.get_or_insert(app.memory_state.selected().unwrap_or_default());
                            *ovr = ovr.saturating_sub(1);
                        },
                        KeyCode::Down => {
                            let ovr = app.mem_row_sel_override.get_or_insert(app.memory_state.selected().unwrap_or_default());
                            *ovr = ovr.saturating_add(1);
                        },
                        KeyCode::Char('n') => {
                            let this_tick = SystemTime::now();
                            app = try_tick(app, last_tick, this_tick)?;
                            last_tick = this_tick;
                        }
                        KeyCode::Char('m') => app.mem_row_sel_override = None,
                        KeyCode::Char('u') => app.inc_tick_rate(),
                        KeyCode::Char('j') => app.dec_tick_rate(),
                        KeyCode::Char('p') => app.pause_tick(),
                        _ => {},
                    }
                }
            }
        }

        if last_tick.elapsed().unwrap_or_default() >= app.get_tick_rate() {
            let this_tick = SystemTime::now();
            app = try_tick(app, last_tick, this_tick)?;
            last_tick = this_tick;
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn process_chip8_input(key: KeyEvent, app: &mut App) -> bool {
    if key.kind != KeyEventKind::Press {
        return false
    }
    
    let bitval = match key.code {
        KeyCode::Char('1') => 2,    // 1
        KeyCode::Char('2') => 4,    // 2
        KeyCode::Char('3') => 8,    // 3
        KeyCode::Char('4') => 4096, // C
        KeyCode::Char('q') => 16,   // 4
        KeyCode::Char('w') => 32,   // 5
        KeyCode::Char('e') => 64,   // 6
        KeyCode::Char('r') => 8192, // D
        KeyCode::Char('a') => 128,  // 7
        KeyCode::Char('s') => 256,  // 8
        KeyCode::Char('d') => 512,  // 9
        KeyCode::Char('f') => 16384,// E
        KeyCode::Char('z') => 1024, // A
        KeyCode::Char('x') => 1,    // 0
        KeyCode::Char('c') => 2048, // B
        KeyCode::Char('v') => 32768,// F
        _ => return false
    };

    app.chip_state.input ^= bitval;

    true
}

fn try_tick(app: App, last_tick: SystemTime, this_tick: SystemTime) -> io::Result<App> {
    let timer_ticks = timer_ticks_between(last_tick, this_tick).unwrap_or_default();

    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {
        // do nothing
    }));
    let last_instr_count = app.instr_count;
    let app = panic::catch_unwind(|| Ok(app.on_tick(timer_ticks))).unwrap_or_else(|panic| {
        let mut new_app = App::new(Some(Failure { panic_message: format!("{:?}", display_caught_panic(&panic)), last_instr_count }));
        load_rom_cmdl(&mut new_app)?;
        Ok(new_app)
    });
    panic::set_hook(old_hook);

    app
}

fn timer_ticks_between(last_tick: SystemTime, this_tick: SystemTime) -> Result<u32, SystemTimeError> {
    let last_nanos = last_tick.duration_since(UNIX_EPOCH)?.as_nanos();
    let this_nanos = this_tick.duration_since(UNIX_EPOCH)?.as_nanos();

    let last_pos = last_nanos / TIMER_RATE;
    let this_pos = this_nanos / TIMER_RATE;

    Ok(u32::try_from(this_pos.saturating_sub(last_pos)).unwrap_or_default())
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