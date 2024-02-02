use ratatui::{layout::*, style::*, text::*, widgets::*, Frame};

use crate::controllers::contact_controller::ContactCtrl;
use crate::views::{string_min_size, titled_box};
pub fn list_view(parent_controller: &mut ContactCtrl, f: &mut Frame, area: Rect) {
    //@TODO: add a filter area with an input to search the list above the Table object
    let mut tablestate = parent_controller.t_state.clone();
    let content = list_project_records(parent_controller);

    f.render_stateful_widget(content, area, &mut tablestate);
}

pub fn edit_view(parent_controller: &mut ContactCtrl, f: &mut Frame, area: Rect) {
    let content = edit_project_form(parent_controller);

    f.render_widget(content, area);
}

pub fn del_view(parent_controller: &mut ContactCtrl, f: &mut Frame, area: Rect) {
    let content = show_delete_confirm(parent_controller);
    f.render_widget(content, area);
}

pub fn detail_view(parent_controller: &mut ContactCtrl, f: &mut Frame, area: Rect) {
    let content = show_detail_project(parent_controller);
    f.render_widget(content, area);
}

fn list_project_records(controller: &mut ContactCtrl) -> Table<'_> {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::LightYellow);
    let header_cells = ["Id", "First name", "Family name", "Email"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let mut rows: Vec<Row> = vec![];

    match controller.list() {
        Ok(list) => {
            rows = list
                .iter()
                .map(|item| -> Row<'_> {
                    let row_values: Vec<String> = vec![
                        format!("{}", item.id),
                        item.first_name.clone(),
                        item.last_name.clone(),
                        item.email.clone(),
                    ];
                    let height = row_values
                        .iter()
                        .map(|content| content.chars().filter(|c| *c == '\n').count())
                        .max()
                        .unwrap_or(0)
                        + 1;
                    let cells = row_values.iter().map(|c| Cell::from(c.clone()));
                    Row::new(cells).height(height as u16).bottom_margin(1)
                })
                .collect();
        }
        Err(_e) => {
            //@TODO display error
        }
    }
    Table::new(
        rows,
        [
            Constraint::Max(6),
            Constraint::Min(10),
            Constraint::Max(30),
            Constraint::Percentage(50),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Contacts"))
    .highlight_style(selected_style)
    .highlight_symbol(">> ")
}

fn get_project_form_fields(data: &ContactCtrl, show_selected: bool) -> Vec<Line<'_>> {
    let normal_bg = Color::Black;
    let higlight_bg = Color::Yellow;
    let str_min_size = 60;
    let lbl_min_size = 16;
    let record = &data.table;
    let fields: Vec<String> = vec![
        record.first_name.clone(),
        record.last_name.clone(),
        record.email.clone(),
    ];
    let labels = vec![
        "First name:".to_string(),
        "Family name :".to_string(),
        "Email :".to_string(),
    ];

    let mut form_parts = vec![Line::from(vec![Span::raw("")])];

    fields.iter().enumerate().for_each(|(i, x)| {
        let mut field = String::from(x);
        let mut label = String::from(&labels[i]);
        string_min_size(&mut field, str_min_size);
        string_min_size(&mut label, lbl_min_size);
        form_parts.push(Line::from(vec![
            Span::raw(label),
            Span::styled(
                field,
                Style::default().bg(if data.field_idx == i as u32 && show_selected {
                    higlight_bg
                } else {
                    normal_bg
                }),
            ),
        ]));
        form_parts.push(Line::from(vec![Span::raw("")]));
    });

    form_parts
}
fn edit_project_form(data: &ContactCtrl) -> Paragraph {
    let mut form_parts = get_project_form_fields(data, true);

    form_parts.push(Line::from(vec![Span::raw("")]));
    form_parts.push(Line::from(vec![Span::raw(
        "Press 'Tab' to switch fields, 'Enter' to Save, 'Esc' to cancel.",
    )]));

    Paragraph::new(form_parts)
        .alignment(Alignment::Center)
        .block(titled_box("Edit Contact".to_string()))
}

fn show_delete_confirm(data: &ContactCtrl) -> Paragraph {
    let mut form_parts = get_project_form_fields(data, false);

    form_parts.push(Line::from(vec![Span::raw("")]));
    form_parts.push(Line::from(vec![Span::raw(
        "Press 'Esc' to cancel or 'Enter' to confirm.",
    )]));

    Paragraph::new(form_parts)
        .alignment(Alignment::Center)
        .alignment(Alignment::Center)
        .block(titled_box("Confirm Deletion".to_string()))
}

fn show_detail_project(data: &ContactCtrl) -> Paragraph {
    let mut form_parts = get_project_form_fields(data, false);

    form_parts.push(Line::from(vec![Span::raw("")]));
    form_parts.push(Line::from(vec![Span::raw(
        "Press 'Esc' to close or 'Enter' to edit.",
    )]));

    Paragraph::new(form_parts)
        .alignment(Alignment::Center)
        .alignment(Alignment::Center)
        .block(titled_box("Contact details".to_string()))
}
