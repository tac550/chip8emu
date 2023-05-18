use chip8exe::{Reg, Chip8State};
use tui::{backend::Backend, Frame, layout::{Layout, Constraint, Rect, Direction, Alignment}, widgets::{Block, Borders, Row, Cell, Table, BorderType, Paragraph}, text::{Spans, Span}};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Min(36), Constraint::Length(4)].as_ref())
        .split(f.size());

    draw_status(f, app, chunks[0]);
    draw_mem_fb(f, app, chunks[1]);
    draw_reg_dis(f, app, chunks[2]);
}

fn draw_status<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let top_box = Paragraph::new(gen_status_view(app))
        .alignment(Alignment::Right)
        .block(Block::default().title(shortcuts_view()).borders(Borders::NONE));
    f.render_widget(top_box, area);
}

fn draw_mem_fb<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Length(12), Constraint::Min(0), Constraint::Length(64)])
        .direction(Direction::Horizontal)
        .split(area);

    draw_stack(f, app, chunks[0]);

    let block = Block::default()
        .title("Memory")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    let display = Paragraph::new(render_display(&app.chip_state))
        .block(Block::default().title("Display").borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(display, chunks[2]);
}

fn draw_stack<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Min(0), Constraint::Length(3)])
        .direction(Direction::Vertical)
        .split(area);

    let block = Block::default()
        .title("Stack")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let sp_area = Paragraph::new(gen_sp_view(&app.chip_state));
    f.render_widget(sp_area, chunks[1]);
}

fn draw_reg_dis<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Min(0), Constraint::Length(64)])
        .direction(Direction::Horizontal)
        .split(area);

    let table = Table::new(gen_reg_view(&app.chip_state))
        .block(Block::default().title("Registers").borders(Borders::LEFT).border_type(BorderType::Thick))
        .widths(&[Constraint::Percentage(5); 17]);
    f.render_widget(table, chunks[0]);

    let block = Block::default()
        .title("Disassembly")
        .borders(Borders::RIGHT)
        .border_type(BorderType::Thick);
    f.render_widget(block, chunks[1]);
}

fn gen_status_view<'a>(app: &'a App) -> Vec<Spans<'a>>{
    let mut spans = vec![];

    spans.push(Span::raw(app.disp_frequency()).into());

    spans
}

fn shortcuts_view() -> String {
    String::from("Shortcuts | ^Q: Quit")
}

fn gen_reg_view<'a>(state: &'a Chip8State) -> Vec<Row<'a>> {
    let mut row1 = vec![];
    let mut row2 = vec![];

    for i in 0..=15 {
        let register = Reg::from(i);
        row1.push(Cell::from(format!("{:?}", register)));
        row2.push(Cell::from(format!("{:X?}", state.registers[register as usize])));
    }

    row1.push(Cell::from("Idx"));
    row2.push(Cell::from(format!("{:X?}", state.index)));

    vec![Row::new(row1), Row::new(row2)]
}

fn render_display(state: &Chip8State) -> Vec<Spans> {
    let mut spans = vec![];
    for y in 0..32 {
        let mut inner_spans = vec![];
        for x in 0..64 {
            inner_spans.push(if state.framebuffer[(8 * y) + (x / 8)] & 0x80 >> x % 8 == 0 {Span::raw(" ")} else {Span::raw("█")})
        }
        spans.push(Spans::from(inner_spans));
    }

    spans
}

fn gen_sp_view(state: &Chip8State) -> Vec<Spans> {
    let mut spans = vec![];

    spans.push(Spans::from(vec![
        Span::raw(" SP: "),
        Span::raw(format!("{:X}", state.sp))
    ]));
    spans.push(Spans::from(vec![
        Span::raw(" PC: "),
        Span::raw(format!("{:X}", state.pc))
    ]));

    spans
}