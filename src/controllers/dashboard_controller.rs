use super::{CtrObj, CtrlActions};
use crate::app::AppState;
use crate::models::{project::Project, task::Task, DbObj};
use crate::views::dashboard::*;
use crate::UtilFns;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    widgets::{ListState, TableState},
    Frame,
};
use rusqlite::Error as RuError;
use std::{collections::HashMap, io::Error};

#[derive(Debug, Clone)]
pub enum PopupDashboardType {
    NoPopup,
    ProjectList,
}
#[derive(Debug, Clone)]
pub struct DashboardCtrl {
    pub projects: Project,
    pub action: CtrlActions,
    pub active_item: usize,
    pub field_idx: u32,
    pub scrum_col_focus: i32,
    pub scrum_col_count: usize,
    pub scrum_col_list: Vec<HashMap<String, String>>,
    pub input: String,
    pub l_state: ListState,
    pub l_scrum_state: ListState,
    pub record_count: usize,
    pub projects_vec: Vec<HashMap<String, String>>,
    pub tasks_vec: Vec<HashMap<String, String>>,
    pub task_status_vec: Vec<HashMap<String, String>>,
    pub show_popup: bool,
    pub popup_type: PopupDashboardType,
}

impl Default for DashboardCtrl {
    fn default() -> DashboardCtrl {
        DashboardCtrl {
            projects: Project::default(),
            action: CtrlActions::List,
            active_item: 0,
            field_idx: 0,
            scrum_col_focus: 0,
            scrum_col_count: 0,
            scrum_col_list: vec![],
            input: String::new(),
            l_state: ListState::default(),
            l_scrum_state: ListState::default(),
            record_count: 0,
            show_popup: false,
            projects_vec: vec![],
            tasks_vec: vec![],
            task_status_vec: vec![],
            popup_type: PopupDashboardType::NoPopup,
        }
    }
}

impl UtilFns for DashboardCtrl {}

impl CtrObj for DashboardCtrl {
    fn init_data(&mut self) {
        match self.project_list() {
            Ok(list) => self.projects_vec = list.clone(),
            Err(_e) => self.projects_vec = vec![],
        }
        match self.task_status() {
            Ok(task_status) => self.task_status_vec = task_status.clone(),
            Err(_e) => self.task_status_vec = vec![],
        }
        if self.projects_vec.len() > 0 {
            match self.l_state.selected() {
                Some(_idx) => {}
                None => {
                    self.l_state.select(Some(0));
                    match self.projects_vec[0].get("id") {
                        Some(id) => self.load_selected_project(id.parse::<i32>().unwrap()),
                        None => {}
                    }
                }
            };
        }
    }

    fn display(&mut self, f: &mut Frame, area: Rect) -> Result<(), Error> {
        match self.action {
            CtrlActions::List => list_view(self, f, area),
            CtrlActions::Edit => {}
            CtrlActions::Del => {}
            CtrlActions::Detail => detail_view(self, f, area),
        }
        Ok(())
    }

    fn key_event_handler(&mut self, key: &KeyEvent) -> AppState {
        match self.action {
            CtrlActions::List => return self.list_key_event(key),
            CtrlActions::Edit => return self.edit_key_event(key),
            CtrlActions::Detail => return self.detail_key_event(key),
            CtrlActions::Del => return self.del_key_event(key),
        }
    }
}

impl DashboardCtrl {
    pub fn task_list(&mut self) -> Result<Vec<HashMap<String, String>>, RuError> {
        let custom_query = "SELECT t.*, p.name AS 'project_name' FROM task AS t LEFT JOIN project AS p ON (t.project_id = p.id)  ORDER BY t.status ASC, t.weight DESC, t.name ASC ".to_string();
        match Self::get_db_connection() {
            Ok(conn) => match Task::query(&conn, custom_query) {
                Ok(list) => {
                    self.record_count = list.len();
                    Ok(list)
                }
                Err(e) => Err(e),
            },
            Err(e) => {
                //@TODO: show error popup
                println!("error getting connection: {}", e.to_string());
                return Err(RuError::InvalidQuery);
            }
        }
    }

    pub fn project_list(&mut self) -> Result<Vec<HashMap<String, String>>, RuError> {
        let custom_query = "select id, name from project order by name".to_string();
        match Self::get_db_connection() {
            Ok(conn) => return Task::query(&conn, custom_query),
            Err(e) => return Err(e),
        }
    }

    pub fn task_status(&mut self) -> Result<Vec<HashMap<String, String>>, RuError> {
        let custom_query = "select id, name from task_status order by id".to_string();
        match Self::get_db_connection() {
            Ok(conn) => return Task::query(&conn, custom_query),
            Err(e) => return Err(e),
        }
    }
    pub fn project_tasks(
        &mut self,
        project_id: i32,
    ) -> Result<Vec<HashMap<String, String>>, RuError> {
        let custom_query = format!(
            "select id, name, status from task where project_id='{}' order by name",
            project_id
        );
        match Self::get_db_connection() {
            Ok(conn) => return Task::query(&conn, custom_query),
            Err(e) => return Err(e),
        }
    }

    pub fn update_field(&mut self) {
        match self.field_idx {
            0 => { /* self.projects.project_id = self.input.parse().unwrap() */ }
            1 => self.projects.name = self.input.clone(),
            2 => self.projects.description = self.input.clone(),
            _ => {}
        }
    }

    pub fn load_selected_project(&mut self, id: i32) {
        match Self::get_db_connection() {
            Ok(conn) => match Project::get_by_id(&conn, id) {
                Ok(project_record) => self.projects = project_record,
                Err(_e) => {}
            },
            Err(_e) => {}
        }
        if self.projects.id > 0 {
            match self.project_tasks(self.projects.id) {
                Ok(tasks_list) => {
                    self.tasks_vec = tasks_list.clone();
                }
                Err(_e) => {
                    self.tasks_vec = vec![];
                }
            }
        }
    }

    pub fn previous_row(&mut self) {}

    pub fn next_row(&mut self) {}

    pub fn previous_task(&mut self) {
        if self.scrum_col_count > 0 {
            let itm = match self.l_scrum_state.selected() {
                Some(idx) => {
                    if idx == 0 {
                        self.scrum_col_count - 1
                    } else {
                        idx - 1
                    }
                }
                None => 0,
            };
            self.l_scrum_state.select(Some(itm));
            /*
            match self.projects_vec[itm].get("id") {
                Some(id) => self.load_selected_project(id.parse::<i32>().unwrap()),
                None => {}
            }
            */
        }
    }

    pub fn next_task(&mut self) {
        if self.scrum_col_count > 0 {
            let itm = match self.l_scrum_state.selected() {
                Some(idx) => {
                    if idx >= self.scrum_col_count - 1 {
                        0
                    } else {
                        idx + 1
                    }
                }
                None => 0,
            };
            self.l_scrum_state.select(Some(itm));
            /*
            match self.projects_vec[itm].get("id") {
                Some(id) => self.load_selected_project(id.parse::<i32>().unwrap()),
                None => {}
            }
            */
        }
    }

    pub fn previous_item(&mut self) {
        if self.record_count > 0 {
            let itm = match self.l_state.selected() {
                Some(idx) => {
                    if idx == 0 {
                        self.record_count - 1
                    } else {
                        idx - 1
                    }
                }
                None => 0,
            };
            self.l_state.select(Some(itm));
            match self.projects_vec[itm].get("id") {
                Some(id) => self.load_selected_project(id.parse::<i32>().unwrap()),
                None => {}
            }
        }
    }

    pub fn next_item(&mut self) {
        if self.record_count > 0 {
            let itm = match self.l_state.selected() {
                Some(idx) => {
                    if idx >= self.record_count - 1 {
                        0
                    } else {
                        idx + 1
                    }
                }
                None => 0,
            };
            self.l_state.select(Some(itm));
            match self.projects_vec[itm].get("id") {
                Some(id) => self.load_selected_project(id.parse::<i32>().unwrap()),
                None => {}
            }
        }
    }

    fn update_scrum_task(&mut self, task_id: i32, task_status: i32) {
        match Self::get_db_connection() {
            Ok(conn) => match Task::get_by_id(&conn, task_id) {
                Ok(mut task) => {
                    task.status = task_status;
                    match task.save(&conn) {
                        Ok(updated_task) => match self.project_tasks(updated_task.project_id) {
                            Ok(tasks_list) => self.tasks_vec = tasks_list.clone(),
                            Err(e) => {}
                        },
                        Err(e) => {}
                    }
                }
                Err(e) => {}
            },
            Err(e) => {}
        }
    }
    pub fn list_key_event(&mut self, key: &KeyEvent) -> AppState {
        match key.code {
            KeyCode::Char('n') => {
                self.action = CtrlActions::Edit;
                self.projects = Project::default();
                return AppState::MoveOn;
            }

            KeyCode::Char('P') => {
                if self.scrum_col_focus > 1 {
                    match self.l_scrum_state.selected() {
                        Some(idx) => {
                            let current_col = self.scrum_col_focus - 1;
                            let current_task = self.scrum_col_list[idx]
                                .get("id")
                                .unwrap()
                                .parse::<i32>()
                                .unwrap();
                            let new_task_status = self.task_status_vec[(current_col - 1) as usize]
                                .get("id")
                                .unwrap()
                                .parse::<i32>()
                                .unwrap();
                            self.update_scrum_task(current_task, new_task_status);
                        }

                        None => {}
                    }
                }
                return AppState::MoveOn;
            }

            KeyCode::Char('N') => {
                if self.scrum_col_focus > 0 && self.scrum_col_focus < 3 {
                    match self.l_scrum_state.selected() {
                        Some(idx) => {
                            let current_col = self.scrum_col_focus - 1;
                            let current_task = self.scrum_col_list[idx]
                                .get("id")
                                .unwrap()
                                .parse::<i32>()
                                .unwrap();
                            let new_task_status = self.task_status_vec[(current_col + 1) as usize]
                                .get("id")
                                .unwrap()
                                .parse::<i32>()
                                .unwrap();
                            self.update_scrum_task(current_task, new_task_status);
                        }

                        None => {}
                    }
                }
                return AppState::MoveOn;
            }

            KeyCode::Tab => {
                if self.scrum_col_focus == 0 {
                    return AppState::Running;
                } else {
                    return AppState::MoveOn;
                }
            }

            KeyCode::Left => {
                self.scrum_col_focus = match self.scrum_col_focus {
                    0 => 3,
                    _ => self.scrum_col_focus - 1,
                };

                if self.scrum_col_focus > 0 {
                    self.l_scrum_state.select(Some(0));
                } else {
                    self.l_scrum_state.select(None);
                }

                return AppState::MoveOn;
            }

            KeyCode::Right => {
                self.scrum_col_focus = self.scrum_col_focus + 1;
                if self.scrum_col_focus > 3 {
                    self.scrum_col_focus = 0;
                    self.l_scrum_state.select(None);
                } else {
                    self.l_scrum_state.select(Some(0));
                }
                return AppState::MoveOn;
            }

            KeyCode::Up => {
                if self.scrum_col_focus == 0 {
                    self.previous_item();
                } else {
                    self.previous_task()
                }

                return AppState::MoveOn;
            }

            KeyCode::Down => {
                if self.scrum_col_focus == 0 {
                    self.next_item();
                } else {
                    self.next_task();
                }
                return AppState::MoveOn;
            }

            _ => {
                //println!("key pressed '{:?}'", key.code);
            }
        }

        AppState::Running
    }

    pub fn go_back(&mut self) {
        self.input = "".to_string();
        self.field_idx = 0;
        self.action = CtrlActions::List;
    }

    pub fn del_key_event(&mut self, key: &KeyEvent) -> AppState {
        match key.code {
            KeyCode::Esc => {
                self.go_back();
            }

            KeyCode::Enter => {
                //self.del_task();
            }

            _ => {
                //@NOTE we do nothing!!!!
            }
        }

        AppState::MoveOn
    }
    pub fn detail_key_event(&mut self, key: &KeyEvent) -> AppState {
        match key.code {
            KeyCode::Esc => {
                self.go_back();
            }

            KeyCode::Enter => {
                self.action = CtrlActions::Edit;
            }

            _ => {
                //@NOTE we do nothing!!!!
            }
        }

        AppState::MoveOn
    }

    pub fn popup_key_event(&mut self, key: &KeyEvent) -> AppState {
        match key.code {
            KeyCode::Up => {
                self.previous_item();
                return AppState::MoveOn;
            }

            KeyCode::Down => {
                self.next_item();
                return AppState::MoveOn;
            }

            KeyCode::Esc => {
                self.show_popup = false;
                return AppState::MoveOn;
            }

            KeyCode::Enter => {
                match self.l_state.selected() {
                    Some(idx) => match self.project_list() {
                        Ok(results) => {
                            let project_id =
                                results[idx].get("id").unwrap().parse::<i32>().unwrap();
                            self.projects.id = project_id;
                            self.show_popup = false;
                            self.popup_type = PopupDashboardType::NoPopup;
                        }
                        Err(e) => {}
                    },
                    None => {}
                }
                return AppState::MoveOn;
            }

            _ => {
                return AppState::MoveOn;
            }
        }
    }
    pub fn edit_key_event(&mut self, key: &KeyEvent) -> AppState {
        if self.show_popup {
            return self.popup_key_event(key);
        }
        match key.code {
            KeyCode::Tab => {
                self.set_next_active();
            }

            KeyCode::Esc => {
                self.go_back();
                //@TODO: reset all fields when to empty if we were creating a new task or to the
                //fields original values of the selected task
            }

            KeyCode::Backspace => {
                self.input.pop();
                self.update_field();
            }

            KeyCode::Enter => {
                //self.save_task();
            }

            KeyCode::Char(c) => {
                match self.field_idx {
                    0 => {
                        // show popup with project lists to select from and set project_id to the
                        // project.id from selected project
                        if c == ' ' {
                            match self.project_list() {
                                Ok(results) => self.record_count = results.len(),
                                Err(_e) => self.record_count = 0,
                            }
                            self.popup_type = PopupDashboardType::ProjectList;
                            self.show_popup = true;
                        }
                    }
                    _ => {
                        self.input.push(c);
                        self.update_field();
                    }
                }
            }

            _ => {
                //println!("key pressed '{:?}'", key.code);
            }
        }

        AppState::MoveOn
    }

    pub fn set_next_active(&mut self) {
        match self.action {
            CtrlActions::Edit => {
                self.field_idx = self.field_idx + 1;
                if self.field_idx == 5 {
                    self.field_idx = 0;
                }
                self.input = match self.field_idx {
                    //0 => self.projects.project_id.to_string(),
                    //1 => self.projects.parent_id.clone(),
                    0 => "".to_string(),
                    1 => "".to_string(),
                    2 => self.projects.name.clone(),
                    3 => self.projects.description.clone(),
                    4 => "".to_string(),
                    //4 => self.projects.status.clone(),
                    _ => "".to_string(),
                }
            }

            _ => {}
        }
    }
}
