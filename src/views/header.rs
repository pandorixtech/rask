use ratatui::{layout::*, style::*, text::*, widgets::*, Frame};

use crate::controllers::header_controller::HeaderCtrl;

pub fn ui_menu<'a>(menu: &Vec<String>, app_title: &str) -> Tabs<'a> {
    let tab_titles = menu
        .iter()
        .map(|t| Line::from(
                Span::styled(t.clone(), 
                             Style::default().fg(Color::Green)
                             )
                )
            )
        .collect();
    //let tabs =
    Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{}", app_title)),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
}

pub fn header_ui(parent_controller: &mut HeaderCtrl, f: &mut Frame, area: Rect) {
    let header = ui_menu(
        &parent_controller
            .menu
            .iter()
            .map(|m| -> String { m.name.clone() })
            .collect(),
        parent_controller.title.as_str(),
    )
    .select(parent_controller.active_item);

    f.render_widget(header, area);
}
