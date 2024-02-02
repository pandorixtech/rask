use ratatui::{
    layout::*,
    style::*,
    text::*,
    widgets::{calendar::*, *},
    Frame,
};
use std::collections::HashMap;

use crate::controllers::dashboard_controller::DashboardCtrl;
use crate::views::task::project_list_ui;
use crate::views::{list_ui, listitems_from_id_name, string_min_size, titled_box};
pub fn list_view(parent_controller: &mut DashboardCtrl, f: &mut Frame, area: Rect) {
    let subareas = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    project_list_col(parent_controller, f, subareas[0]);
    project_detail_col(parent_controller, f, subareas[1]);

    //f.render_stateful_widget(content, area, &mut tablestate);
}

fn scrum_col_ui(list: Vec<HashMap<String, String>>, title: String) -> List<'static> {
    list_ui(listitems_from_id_name(list), title)
}

fn project_detail_col(ctrl: &mut DashboardCtrl, f: &mut Frame, area: Rect) {
    let mut project_name = "Project Details (Scrumboard)".to_string();

    if ctrl.projects.id > 0 {
        project_name = ctrl.projects.name.clone();
    }

    let mut scrum_cols_constraint = vec![];
    let mut scrum_cols_content_count = vec![];
    let mut scrum_cols_content = vec![];
    let mut scrum_list_views = vec![];
    let constraint_percent = 100 / ctrl.task_status_vec.len();
    ctrl.task_status_vec.iter().for_each(|tstatus| {
        scrum_cols_constraint.push(Constraint::Percentage(constraint_percent as u16));
        let status_id = tstatus.get("id").unwrap().parse::<i32>().unwrap();
        let status_content: Vec<HashMap<String, String>> = ctrl
            .tasks_vec
            .iter()
            .map(|task| task.clone())
            .filter(|task| task.get("status").unwrap().parse::<i32>().unwrap() == status_id)
            .collect();
        let col_focus = ctrl.scrum_col_focus;
        let col_id = tstatus.get("id").unwrap().parse::<i32>().unwrap();
        let col_name: String;
        if col_focus == col_id {
            col_name = format!("<{}>", tstatus.get("name").unwrap().clone());
        } else {
            col_name = tstatus.get("name").unwrap().clone();
        }
        scrum_cols_content_count.push(status_content.len());
        scrum_cols_content.push(status_content.clone());
        scrum_list_views.push(scrum_col_ui(status_content.clone(), col_name.clone()));
    });
    let project_details = titled_box(project_name);

    let scrumcols = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(scrum_cols_constraint)
        .split(project_details.inner(area));

    scrum_list_views.reverse();

    scrumcols.iter().enumerate().for_each(|(i, col_area)| {
        //f.render_widget(col_box, *col_area);
        if i as i32 == (ctrl.scrum_col_focus - 1) {
            ctrl.scrum_col_count = scrum_cols_content_count[i];
            ctrl.scrum_col_list = scrum_cols_content[i].clone();
            f.render_stateful_widget(
                scrum_list_views.pop().unwrap(),
                *col_area,
                &mut ctrl.l_scrum_state,
            );
        } else {
            f.render_widget(scrum_list_views.pop().unwrap(), *col_area);
        }
    });
    f.render_widget(project_details, area);
}

fn project_list_col(ctrl: &mut DashboardCtrl, f: &mut Frame, area: Rect) {
    ctrl.record_count = ctrl.projects_vec.len();
    let project_col = project_list_ui(ctrl.projects_vec.clone());
    f.render_stateful_widget(project_col, area, &mut ctrl.l_state);
}

pub fn detail_view(parent_controller: &mut DashboardCtrl, f: &mut Frame, area: Rect) {
    //let content = show_detail_project(parent_controller);
    //f.render_widget(content, area);
}
