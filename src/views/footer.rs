use ratatui::{layout::*, style::*, text::*, widgets::*, Frame};

use crate::controllers::footer_controller::FooterCtrl;

pub fn footer_ui(_parent_controller: &mut FooterCtrl, f: &mut Frame, area: Rect) {
    let content = vec![
        Span::raw("Press "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to exit, "),
        Span::styled("up | down", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to navigate, "),
        Span::styled("n", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to create new, "),
        Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to delete, "),
        //        Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
        //        Span::raw(" to config, "),
        Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to edit selected record."),
    ];
    let footer_style = Style::default(); //.add_modifier(Modifier::RAPID_BLINK);
    let mut text = Text::from(Line::from(content));
    text.patch_style(footer_style);

    let footer_message = Paragraph::new(text);

    f.render_widget(footer_message, area);
}
pub fn dashboard_footer_ui(_parent_controller: &mut FooterCtrl, f: &mut Frame, area: Rect) {
    let content = vec![
        Span::raw("Press "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to exit, "),
        Span::styled(
            "up | down | left | right",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" to navigate, "),
        Span::styled("N", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to move scrumboard item to next column, "),
        Span::styled("P", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to move scrumboard item to previous colum, "),
        //        Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
        //        Span::raw(" to config, "),
        Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to go to next menu item."),
    ];
    let footer_style = Style::default(); //.add_modifier(Modifier::RAPID_BLINK);
    let mut text = Text::from(Line::from(content));
    text.patch_style(footer_style);

    let footer_message = Paragraph::new(text);

    f.render_widget(footer_message, area);
}
