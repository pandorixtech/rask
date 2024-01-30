use ratatui::{layout::*, style::*, text::*, widgets::*, Frame};

use crate::controllers::task_controller::{PopupTaskType, TaskCtrl};
use crate::views::{
    generic_popup_stateful, list_ui, listitems_from_id_name, string_min_size, titled_box,
    DEFAULT_BG, HIGHLIGHT_BG,
};
use std::collections::HashMap;

pub fn list_view(parent_controller: &mut TaskCtrl, f: &mut Frame, area: Rect) {
    //@TODO: add a filter area with an input to search the list above the Table object
    let mut tablestate = parent_controller.t_state.clone();
    let content = list_task_records(parent_controller);

    f.render_stateful_widget(content, area, &mut tablestate);
}

pub fn edit_view(parent_controller: &mut TaskCtrl, f: &mut Frame, area: Rect) {
    let content = edit_task_form(parent_controller);

    f.render_widget(content, area);
    if parent_controller.show_popup {
        show_task_popup(parent_controller, f);
    }
}

pub fn show_task_popup(data: &mut TaskCtrl, f: &mut Frame) {
    match data.popup_type {
        PopupTaskType::ProjectList => project_popup(data, f),
        PopupTaskType::TaskStatusList => task_status_popup(data, f),
        _ => {}
    }
}
pub fn del_view(parent_controller: &mut TaskCtrl, f: &mut Frame, area: Rect) {
    let content = show_delete_confirm(parent_controller);
    f.render_widget(content, area);
}

pub fn detail_view(parent_controller: &mut TaskCtrl, f: &mut Frame, area: Rect) {
    let content = show_detail_task(parent_controller);
    f.render_widget(content, area);
}

fn list_task_records(controller: &mut TaskCtrl) -> Table<'_> {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::LightYellow);
    let header_cells = ["Id", "Project", "Name", "Description"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let mut rows: Vec<Row> = vec![];

    match controller.task_list() {
        Ok(list) => {
            rows = list
                .iter()
                .map(|item| -> Row<'_> {
                    let row_values: Vec<String> = vec![
                        item.get("id").unwrap().to_string(),
                        item.get("project_name").unwrap().to_string(),
                        item.get("name").unwrap().to_string(),
                        item.get("description").unwrap().to_string(),
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
    Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Max(6),
            Constraint::Max(30),
            Constraint::Max(30),
            Constraint::Percentage(30),
        ])
}

fn get_task_form_fields(data: &TaskCtrl, show_selected: bool) -> Vec<Line<'_>> {
    let normal_bg = DEFAULT_BG;
    let higlight_bg = HIGHLIGHT_BG;

    let str_min_size = 60;
    let lbl_min_size = 16;
    let project_name = data.get_project_name();
    let status_name = data.get_status_name();
    let record = &data.task_table;

    let fields: Vec<String> = vec![
        //record.project_id.clone(),
        project_name.clone(),
        record.name.clone(),
        record.description.clone(),
        record.weight.to_string(),
        status_name.clone(),
    ];
    let labels = vec![
        "Project :".to_string(),
        "Name :".to_string(),
        "Description :".to_string(),
        "Weight :".to_string(),
        "Status :".to_string(),
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

pub fn project_list_ui(list: Vec<HashMap<String, String>>) -> List<'static> {
    list_ui(listitems_from_id_name(list), "".to_string())
}

pub fn task_status_list_ui(list: Vec<HashMap<String, String>>) -> List<'static> {
    list_ui(listitems_from_id_name(list), "".to_string())
}

fn project_popup(data: &mut TaskCtrl, f: &mut Frame) {
    match data.project_list() {
        Ok(list) => {
            let content = project_list_ui(list);

            generic_popup_stateful(
                "Project List".to_string(),
                content,
                "Press UP and Down to select, Enter to accept and Esc to cancel.".to_string(),
                f,
                &mut data.l_state,
            );
        }
        Err(e) => {}
    }
}

fn task_status_popup(data: &mut TaskCtrl, f: &mut Frame) {
    match data.task_status_list() {
        Ok(list) => {
            let content = task_status_list_ui(list);

            generic_popup_stateful(
                "Task Status List".to_string(),
                content,
                "Press UP and Down to select, Enter to accept and Esc to cancel.".to_string(),
                f,
                &mut data.l_state,
            );
        }
        Err(e) => {}
    }
}

fn edit_task_form(data: &TaskCtrl) -> Paragraph {
    let mut form_parts = get_task_form_fields(data, true);

    form_parts.push(Line::from(vec![Span::raw("")]));
    form_parts.push(Line::from(vec![Span::raw(
        "Press 'Tab' to switch fields, 'Enter' to Save, 'Esc' to cancel.",
    )]));

    Paragraph::new(form_parts)
        .alignment(Alignment::Center)
        .block(titled_box("Edit Task".to_string()))
}

fn show_delete_confirm(data: &TaskCtrl) -> Paragraph {
    let mut form_parts = get_task_form_fields(data, false);

    form_parts.push(Line::from(vec![Span::raw("")]));
    form_parts.push(Line::from(vec![Span::raw(
        "Press 'Esc' to cancel or 'Enter' to confirm.",
    )]));

    Paragraph::new(form_parts)
        .alignment(Alignment::Center)
        .alignment(Alignment::Center)
        .block(titled_box("Confirm Deletion".to_string()))
}

fn show_detail_task(data: &TaskCtrl) -> Paragraph {
    let mut form_parts = get_task_form_fields(data, false);

    form_parts.push(Line::from(vec![Span::raw("")]));
    form_parts.push(Line::from(vec![Span::raw(
        "Press 'Esc' to close or 'Enter' to edit.",
    )]));

    Paragraph::new(form_parts)
        .alignment(Alignment::Center)
        .alignment(Alignment::Center)
        .block(titled_box("Task details".to_string()))
}
