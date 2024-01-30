use super::{CtrObj, CtrlActions};
use crate::app::AppState;
use crate::models::{task::Task, DbObj};
use crate::views::task::*;
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
pub enum PopupTaskType {
    NoPopup,
    ProjectList,
    TaskStatusList,
}
#[derive(Debug, Clone)]
pub struct TaskCtrl {
    pub task_table: Task,
    pub action: CtrlActions,
    pub active_item: usize,
    pub field_idx: u32,
    pub input: String,
    pub t_state: TableState,
    pub l_state: ListState,
    pub record_count: usize,
    pub show_popup: bool,
    pub popup_type: PopupTaskType,
}

impl Default for TaskCtrl {
    fn default() -> TaskCtrl {
        TaskCtrl {
            task_table: Task::default(),
            action: CtrlActions::List,
            active_item: 0,
            field_idx: 0,
            input: String::new(),
            t_state: TableState::default(),
            l_state: ListState::default(),
            record_count: 0,
            show_popup: false,
            popup_type: PopupTaskType::NoPopup,
        }
    }
}

impl UtilFns for TaskCtrl {}

impl CtrObj for TaskCtrl {
    fn init_data(&mut self) {}

    fn display(&mut self, f: &mut Frame, area: Rect) -> Result<(), Error> {
        match self.action {
            CtrlActions::List => list_view(self, f, area),
            CtrlActions::Edit => edit_view(self, f, area),
            CtrlActions::Del => del_view(self, f, area),
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

impl TaskCtrl {
    pub fn task_list(&mut self) -> Result<Vec<HashMap<String, String>>, RuError> {
        let custom_query = "select t.*, p.name as 'project_name', ts.name as 'status_name' from task as t left join project as p on (t.project_id = p.id) LEFT JOIN task_status as ts ON (t.status = ts.id)".to_string();
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

    pub fn task_status_list(&mut self) -> Result<Vec<HashMap<String, String>>, RuError> {
        let custom_query = "select id, name from task_status".to_string();
        match Self::get_db_connection() {
            Ok(conn) => return Task::query(&conn, custom_query),
            Err(e) => return Err(e),
        }
    }

    pub fn update_field(&mut self) {
        match self.field_idx {
            0 => self.task_table.project_id = self.input.parse().unwrap(),
            1 => self.task_table.name = self.input.clone(),
            2 => self.task_table.description = self.input.clone(),
            3 => self.task_table.weight = self.input.parse().unwrap_or_default(),
            4 => self.task_table.status = self.input.parse().unwrap(),

            _ => {}
        }
    }

    pub fn get_status_name(&self) -> String {
        match Self::get_db_connection() {
            Ok(conn) => match self.task_table.get_status(&conn) {
                Some(p) => return p.name.clone(),
                None => return "--".to_string(),
            },
            Err(_e) => return "--".to_string(),
        }
    }

    pub fn get_project_name(&self) -> String {
        match Self::get_db_connection() {
            Ok(conn) => match self.task_table.get_project(&conn) {
                Some(p) => return p.name.clone(),
                None => return "--".to_string(),
            },
            Err(_e) => return "--".to_string(),
        }
    }

    pub fn del_task(&mut self) {
        if self.task_table.id > 0 {
            match Self::get_db_connection() {
                Ok(conn) => {
                    match self.task_table.del(&conn) {
                        Ok(()) => {
                            self.task_table = Task::default();
                            self.go_back();
                        }
                        Err(e) => {
                            //@TODO: show popup error
                            println!("faile to save record {}", e.to_string());
                        }
                    }
                }
                Err(_e) => {
                    println!("ERROR NO DATABASE CONN");
                    //@TODO: show popup error
                }
            }
        }
    }
    pub fn save_task(&mut self) {
        //@TODO : validate fields before saving a show apropriate messages

        match Self::get_db_connection() {
            Ok(conn) => {
                match self.task_table.save(&conn) {
                    Ok(task) => {
                        self.task_table = task.clone();
                        self.go_back();
                    }
                    Err(e) => {
                        //@TODO: show popup error
                        println!("faile to save record {}", e.to_string());
                    }
                }
            }
            Err(_e) => {
                println!("ERROR NO DATABASE CONN");
                //@TODO: show popup error
            }
        }
    }

    pub fn previous_row(&mut self) {
        if self.record_count > 0 {
            let r = match self.t_state.selected() {
                Some(idx) => {
                    if idx == 0 {
                        self.record_count - 1
                    } else {
                        idx - 1
                    }
                }
                None => 0,
            };
            self.t_state.select(Some(r));
        }
    }

    pub fn next_row(&mut self) {
        if self.record_count > 0 {
            let r = match self.t_state.selected() {
                Some(idx) => {
                    if idx >= self.record_count - 1 {
                        0
                    } else {
                        idx + 1
                    }
                }
                None => 0,
            };
            self.t_state.select(Some(r));
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
        }
    }

    pub fn set_selected_record(&mut self) {
        match self.t_state.selected() {
            Some(idx) => {
                match self.task_list() {
                    Ok(tasks) => {
                        self.task_table = Task::from_hashmap(&mut tasks[idx].clone()).unwrap();
                        //.clone();
                    }
                    Err(_e) => {
                        //@TODO: show error message
                    }
                }
            }
            None => {
                //@TODO: show message "no record selected"
            }
        }
    }

    pub fn list_key_event(&mut self, key: &KeyEvent) -> AppState {
        match key.code {
            KeyCode::Char('n') => {
                self.action = CtrlActions::Edit;
                self.task_table = Task::default();
                return AppState::MoveOn;
            }
            KeyCode::Char('e') => {
                self.set_selected_record();
                self.action = CtrlActions::Edit;
                return AppState::MoveOn;
            }

            KeyCode::Char('d') => {
                self.set_selected_record();
                self.action = CtrlActions::Del;
                return AppState::MoveOn;
            }

            KeyCode::Char('s') => {
                self.set_selected_record();
                self.action = CtrlActions::Detail;
                return AppState::MoveOn;
            }

            KeyCode::Up => {
                self.previous_row();
                return AppState::MoveOn;
            }

            KeyCode::Down => {
                self.next_row();
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
                self.del_task();
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
                match self.popup_type {
                    PopupTaskType::ProjectList => match self.l_state.selected() {
                        Some(idx) => match self.project_list() {
                            Ok(results) => {
                                let project_id =
                                    results[idx].get("id").unwrap().parse::<i32>().unwrap();
                                self.task_table.project_id = project_id;
                                self.show_popup = false;
                                self.popup_type = PopupTaskType::NoPopup;
                            }
                            Err(e) => {}
                        },
                        None => {}
                    },
                    PopupTaskType::TaskStatusList => match self.l_state.selected() {
                        Some(idx) => match self.task_status_list() {
                            Ok(results) => {
                                let status_id =
                                    results[idx].get("id").unwrap().parse::<i32>().unwrap();
                                self.task_table.status = status_id;
                                self.show_popup = false;
                                self.popup_type = PopupTaskType::NoPopup;
                            }
                            Err(e) => {}
                        },
                        None => {}
                    },
                    _ => {}
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
                self.save_task();
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
                            self.popup_type = PopupTaskType::ProjectList;
                            self.show_popup = true;
                        }
                    }

                    4 => {
                        // show popup with project lists to select from and set project_id to the
                        // project.id from selected project
                        if c == ' ' {
                            match self.task_status_list() {
                                Ok(results) => self.record_count = results.len(),
                                Err(_e) => self.record_count = 0,
                            }
                            self.popup_type = PopupTaskType::TaskStatusList;
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
                    //0 => self.task_table.project_id.to_string(),
                    //1 => self.task_table.parent_id.clone(),
                    0 => "".to_string(),
                    1 => self.task_table.name.clone(),
                    2 => self.task_table.description.clone(),
                    3 => self.task_table.weight.to_string(),
                    4 => "".to_string(),
                    _ => "".to_string(),
                }
            }

            _ => {}
        }
    }
}
