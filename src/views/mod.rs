use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    collections::HashMap,
    io::{stdout, Error, Stdout},
    rc::Rc,
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::*,
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, StatefulWidget, Widget},
    Frame, Terminal,
};

pub mod contact;
pub mod dashboard;
pub mod footer;
pub mod header;
pub mod project;
pub mod task;

const DEFAULT_BG: Color = Color::Black;
const HIGHLIGHT_BG: Color = Color::Yellow;

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    match enable_raw_mode() {
        Ok(_) => {
            let mut stdout = stdout();

            match execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
                Ok(_) => {
                    let backend = CrosstermBackend::new(stdout);

                    Terminal::new(backend)
                }
                Err(v) => Err(v),
            }
        }
        Err(v) => Err(v),
    }
}

pub fn destruct_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) {
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
    terminal.show_cursor().unwrap();
}

pub fn masterview(f: &mut Frame) -> Rc<[Rect]> {
    //let ui_zones =
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(90),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size())
}

///helper function to generate box with title to wrap popup content
pub fn titled_box<'a>(title: String) -> Block<'a> {
    Block::default().title(title.clone()).borders(Borders::ALL)
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

/// helper function to make displayed string of size min if length is smaller
pub fn string_min_size(the_string: &mut String, min_size: usize) {
    while the_string.len() < min_size {
        the_string.push(' ');
    }
}

/// helper function to boostrap popup content
pub fn generic_popup_stateful<W>(
    title: String,
    content: W,
    instructions: String,
    f: &mut Frame,
    state: &mut <W as StatefulWidget>::State,
) where
    W: StatefulWidget,
{
    let area = f.size();
    let popup_box = titled_box(title);
    let popup_rect = centered_rect(60, 25, area);

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Max(1)].as_ref())
        .split(popup_box.inner(popup_rect));

    let popup_footer =
        Paragraph::new(vec![Line::from(instructions.clone())]).alignment(Alignment::Center);

    f.render_widget(Clear, popup_rect);
    f.render_widget(popup_box, popup_rect);
    f.render_stateful_widget(content, popup_layout[0], state);
    f.render_widget(popup_footer, popup_layout[1]);
}

/// helper function to boostrap popup content
pub fn generic_popup<W>(title: String, content: W, instructions: String, f: &mut Frame)
where
    W: Widget,
{
    let area = f.size();
    let popup_box = titled_box(title);
    let popup_rect = centered_rect(60, 25, area);

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Max(1)].as_ref())
        .split(popup_box.inner(popup_rect));

    let popup_footer =
        Paragraph::new(vec![Line::from(instructions.clone())]).alignment(Alignment::Center);

    f.render_widget(Clear, popup_rect);
    f.render_widget(popup_box, popup_rect);
    f.render_widget(content, popup_layout[0]);
    f.render_widget(popup_footer, popup_layout[1]);
}

pub fn popup_confirm(title: String, message: String, f: &mut Frame) {
    let content = Paragraph::new(Line::from(message));
    let instructions = "Press 'Enter' to confirm, 'Esc' to cancel.".to_string();
    generic_popup(title, content, instructions, f);
}

pub fn popup_info(title: String, message: String, f: &mut Frame) {
    let content = Paragraph::new(Line::from(message));
    let instructions = "Press 'Esc' to close.".to_string();
    generic_popup(title, content, instructions, f);
}

pub fn popup_error(message: String, f: &mut Frame) {
    let title = "Error!!".to_string();
    popup_info(title, message, f);
}

pub fn listitems_from_id_name(list: Vec<HashMap<String, String>>) -> Vec<ListItem<'static>> {
    list.iter()
        //.enumerate()
        .map(|p| {
            let item_content = vec![Line::from(format!(
                "{} - {}",
                p.get("id").unwrap().to_string(),
                p.get("name").unwrap().to_string()
            ))];
            ListItem::new(item_content)
        })
        .collect()
}

pub fn list_ui(data: Vec<ListItem<'static>>, title: String) -> List<'static> {
    List::new(data)
        .block(titled_box(title))
        .highlight_style(Style::default().bg(HIGHLIGHT_BG))
        .highlight_symbol("->")
}
