use super::{CtrObj, CtrlActions};
use crate::app::AppState;
use crate::models::{person::Person, DbObj};
use crate::views::contact::*;
use crate::UtilFns;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, widgets::TableState, Frame};
use rusqlite::Error as RuError;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct ContactCtrl {
    pub table: Person,
    pub action: CtrlActions,
    pub active_item: usize,
    pub field_idx: u32,
    pub input: String,
    pub t_state: TableState,
    pub record_count: usize,
}

impl Default for ContactCtrl {
    fn default() -> ContactCtrl {
        ContactCtrl {
            table: Person::default(),
            action: CtrlActions::List,
            active_item: 0,
            field_idx: 0,
            input: String::new(),
            t_state: TableState::default(),
            record_count: 0,
        }
    }
}

impl UtilFns for ContactCtrl {}

impl CtrObj for ContactCtrl {
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

impl ContactCtrl {
    pub fn list(&mut self) -> Result<Vec<Person>, RuError> {
        match Self::get_db_connection() {
            Ok(conn) => match Person::list(&conn, "".to_string()) {
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

    pub fn update_field(&mut self) {
        match self.field_idx {
            0 => self.table.first_name = self.input.clone(),
            1 => self.table.last_name = self.input.clone(),
            2 => self.table.email = self.input.clone(),
            _ => {}
        }
    }

    pub fn del_record(&mut self) {
        if self.table.id > 0 {
            match Self::get_db_connection() {
                Ok(conn) => {
                    match self.table.del(&conn) {
                        Ok(()) => {
                            self.table = Person::default();
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
    pub fn save_record(&mut self) {
        //@TODO : validate fields before saving a show apropriate messages

        match Self::get_db_connection() {
            Ok(conn) => {
                match self.table.save(&conn) {
                    Ok(record) => {
                        self.table = record.clone();
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

    pub fn set_selected_record(&mut self) {
        match self.t_state.selected() {
            Some(idx) => {
                match self.list() {
                    Ok(record_list) => {
                        self.table = record_list[idx].clone();
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
                self.table = Person::default();
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
                self.del_record();
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

    pub fn edit_key_event(&mut self, key: &KeyEvent) -> AppState {
        match key.code {
            KeyCode::Tab => {
                self.set_next_active();
            }

            KeyCode::Esc => {
                self.go_back();
                //@TODO: reset all fields when to empty if we were creating a new project or to the
                //fields original values of the selected project
            }

            KeyCode::Backspace => {
                self.input.pop();
                self.update_field();
            }

            KeyCode::Enter => {
                self.save_record();
            }

            KeyCode::Char(c) => {
                self.input.push(c);
                self.update_field();
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
                    0 => self.table.first_name.clone(),
                    1 => self.table.last_name.clone(),
                    2 => self.table.email.clone(),
                    _ => "".to_string(),
                }
            }

            _ => {}
        }
    }
}
