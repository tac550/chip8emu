use chip8exe::{Reg, Chip8State};
use ratatui::{Frame, layout::{Layout, Constraint, Rect, Direction, Alignment}, widgets::{Block, Borders, Row, Cell, Table, BorderType, Paragraph, ListItem, List}, text::{Span, Line}, style::{Style, Modifier, Color}};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Min(36), Constraint::Length(4)].as_ref())
        .split(f.size());

    draw_status(f, app, chunks[0]);
    draw_mem_fb(f, app, chunks[1]);
    draw_reg_dis(f, app, chunks[2]);
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let top_box = Paragraph::new(gen_status_view(app))
        .alignment(Alignment::Right)
        .block(Block::default().title(shortcuts_view()).borders(Borders::NONE));
    f.render_widget(top_box, area);
}

fn draw_mem_fb(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Length(10), Constraint::Min(0), Constraint::Length(66)])
        .direction(Direction::Horizontal)
        .split(area);

    draw_stack(f, app, chunks[0]);

    let table = Table::new(gen_mem_view(&app.chip_state), [Constraint::Length(4); 16])
        .block(Block::default().title("Memory").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Cyan))
        .header(Row::new((0..16).map(|i| Cell::from(format!("xx{i:X?}")))));
    app.memory_state.select(Some(app.mem_row_sel_override.unwrap_or((app.chip_state.pc / 16) as usize)));
    f.render_stateful_widget(table, chunks[1], &mut app.memory_state);

    draw_display(f, app, chunks[2]);
}

fn draw_stack(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Min(0), Constraint::Length(3)])
        .direction(Direction::Vertical)
        .split(area);

    let stack_view = List::new(gen_stack_view(&app.chip_state))
        .block(Block::default().title("Stack").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .highlight_symbol("SP>");
    app.stack_state.select(Some(app.chip_state.sp as usize));
    f.render_stateful_widget(stack_view, chunks[0], &mut app.stack_state);

    let sp_area = Paragraph::new(gen_sp_view(&app.chip_state));
    f.render_widget(sp_area, chunks[1]);
}

fn draw_display(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Length(34), Constraint::Min(0)])
        .direction(Direction::Vertical)
        .split(area);

    let display = Paragraph::new(render_display(&app.chip_state))
        .block(Block::default().title("Display").borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(display, chunks[0]);

    let timers = Paragraph::new(gen_timer_view(&app.chip_state))
        .alignment(Alignment::Right);
    f.render_widget(timers, chunks[1]);
}

fn draw_reg_dis(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .direction(Direction::Horizontal)
        .split(area);

    let table = Table::new(gen_reg_view(&app.chip_state), [Constraint::Length(4); 17])
        .block(Block::default().title("Registers").borders(Borders::LEFT).border_type(BorderType::Thick));
    f.render_widget(table, chunks[0]);

    let disassembly = Paragraph::new(vec![Line::default(), Line::from(vec![Span::raw(format!("{:X?}", app.chip_state.decode_opcode()))])])
        .block(Block::default().title("Disassembly").borders(Borders::RIGHT).border_type(BorderType::Thick));
    f.render_widget(disassembly, chunks[1]);
}

fn gen_status_view(app: &App) -> Vec<Line>{
    let mut spans = vec![];

    if let Some(failure) = &app.last_failure {
        spans.push(Span::styled(format!("Emulator crashed! Error details: {} | Instruction Count: {}", failure.panic_message, failure.last_instr_count), Style::default().bg(Color::Red)).into());
    } else {
        spans.push(vec![
            Span::styled(format!("Instruction Count: {} ", app.instr_count), style_warn_overrun(app.instr_count, u64::MAX)),
            Span::raw(format!("| {}", app.disp_frequency())),
        ].into());
    }

    spans
}

fn shortcuts_view() -> String {
    String::from("Shortcuts | ^Q: Quit ^R: Reset N: Step to next instruction ↕: Scroll memory view M: Return memory view to PC U/J: Inc/Dec Frequency P: Pause")
}

fn gen_reg_view(state: &Chip8State) -> Vec<Row> {
    let mut row1 = vec![];
    let mut row2 = vec![];

    for i in 0..=15 {
        let register = Reg::from(i);
        row1.push(Cell::from(format!("{register:?}")));
        row2.push(Cell::from(format!("{:02X?}", state.registers[register as usize])));
    }

    row1.push(Cell::from("Idx"));
    row2.push(Cell::from(format!("{:04X?}", state.index)));

    vec![Row::new(row1), Row::new(row2)]
}

fn render_display(state: &Chip8State) -> Vec<Line> {
    let mut spans = vec![];
    for y in 0..32 {
        let mut inner_spans = vec![];
        for x in 0..64 {
            inner_spans.push(if state.framebuffer[(8 * y) + (x / 8)] & 0x80 >> (x % 8) == 0 {Span::raw(" ")} else {Span::raw("█")});
        }
        spans.push(Line::from(inner_spans));
    }

    spans
}

fn gen_timer_view(state: &Chip8State) -> Vec<Line> {
    let mut spans = vec![];

    let val = state.dt;
    let input = state.input;
    spans.push(Line::from(vec![
        Span::raw(format!("Input: {input:016b}  Delay Timer: {val:02X?}  "))
    ]));
    let val = state.st;
    spans.push(Line::from(vec![
        Span::raw(format!("Sound Timer: {val:02X?}  "))
    ]));

    spans
}

fn gen_sp_view(state: &Chip8State) -> Vec<Line> {
    let mut spans = vec![];

    let val = state.sp;
    spans.push(Line::from(vec![
        Span::styled(format!(" SP: {val:02X?}"), style_warn_overrun(val, 64)),
    ]));
    let val = state.pc;
    spans.push(Line::from(vec![
        Span::styled(format!(" PC: {val:03X?}"), style_warn_overrun(val, 4096)),
    ]));

    spans
}

fn gen_stack_view(state: &Chip8State) -> Vec<ListItem> {
    let mut items = vec![];
    for i in 0..64 {
        let val = state.stack[i];
        items.push(ListItem::new(format!("{val:02X?}")).style(style_fade_default(val)));
    }

    items
}

fn gen_mem_view(state: &Chip8State) -> Vec<Row> {
    let mut rows = vec![];

    for y in 0..256 {
        let mut row = vec![];
        for x in 0..16 {
            let val = state.memory[(16 * y as usize) + x as usize];
            let style = style_fade_default(val).add_modifier(if state.pc / 16 == y && (state.pc % 16 == x || state.pc % 16 == x.saturating_sub(1)) { Modifier::REVERSED } else { Modifier::empty() });
            row.push(Cell::from(format!("{val:02X?}")).style(style));
        }
        rows.push(Row::new(row));
    }

    rows
}

fn style_fade_default<T: Default + PartialEq + Copy>(val: T) -> Style {
    if val == T::default() {
        Style::default().add_modifier(Modifier::DIM)
    } else {
        Style::default()
    }
}

fn style_warn_overrun<T: PartialOrd + Copy>(val: T, limit: T) -> Style {
    if val >= limit {
        Style::default().bg(Color::Red)
    } else {
        Style::default()
    }
}