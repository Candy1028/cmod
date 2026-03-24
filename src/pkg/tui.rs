use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{io, time::Duration};
use std::fmt::Display;
use crossterm::event::MouseEventKind;
use ratatui::widgets::ListState;
use crate::error::error::*;
use crate::error::error::Error::BizError;
use crate::types::tui::*;


pub fn new_multiple_choice<T>(list:&Vec<T>) -> Result<Vec<T>>
where
    T:Display+Clone+CheckedInfo
{

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = TuiMultipleChoice::new(list);
    let res = run_multiple_choice(&mut terminal, &mut app);


    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

pub fn run_multiple_choice<B,T>(terminal: &mut Terminal<B>, app: &mut TuiMultipleChoice<T>) -> Result<Vec<T>>
where
    B: Backend,
    T:Display+Clone+CheckedInfo,
{
    let mut list_state = ListState::default();
    loop {
        list_state.select(Some(app.selected_index));
        terminal.draw(|f| multiple_choice_ui(f, app, &mut list_state))
            .map_err(|e| BizError(e.to_string()))?;
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(Vec::new()),
                    KeyCode::Enter => return Ok(app.get_checked()),
                    KeyCode::Down | KeyCode::Char('s') => app.next(),
                    KeyCode::Up | KeyCode::Char('w') => app.previous(),
                    KeyCode::Char(' ') | KeyCode::Tab => app.toggle_current(),
                    _ => {}
                },
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollDown => app.next(),
                    MouseEventKind::ScrollUp => app.previous(),
                    _ => {}
                },
                _ => {}
            }
        }

    }
}

pub fn multiple_choice_ui<T>(f: &mut Frame, app: &TuiMultipleChoice<T>,list_state: &mut ListState)
where
    T:Display+Clone+CheckedInfo
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(f.area());

    let items: Vec<ListItem> = app.items.iter()
        .enumerate()
        .map(|(i, item)| {
            let checkbox = if app.checked_items.contains(&i) {
                "[x]"
            } else {
                "[ ]"
            };
            let content = format!("{} {}", checkbox, item);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("多选菜单")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], list_state);

    let checked_names: Vec<String> = app.checked_items
        .iter()
        .map(|&i| app.items[i].info().to_string())
        .collect();

    let detail_text = format!(
        "Tab 或 Space 选择 \n\
         Enter 确定并退出 \n\
         q 或 Esc 退出 \n\n\
         当前选择: {}\n\n\
         已勾选的项目:\n{:#?}",
        app.items[app.selected_index].info(),
        checked_names
    );
    let detail = Paragraph::new(detail_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default()
            .title("详情").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)));
    f.render_widget(detail, chunks[1]);
}