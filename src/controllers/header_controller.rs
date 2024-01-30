use super::{get_controller_from_registry, ControllerRegistry, CtrObj};
use crate::app::AppState;
use crate::views::header::header_ui;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};
use std::io::Error;

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub name: String,
    ctrl: ControllerRegistry,
}

#[derive(Debug, Clone)]
pub struct HeaderCtrl {
    pub menu: Vec<MenuItem>,
    pub title: String,
    pub active_item: usize,
    pub item_changed: bool,
}

impl Default for HeaderCtrl {
    fn default() -> HeaderCtrl {
        HeaderCtrl {
            menu: vec![
                MenuItem {
                    name: String::from("Dashboard"),
                    ctrl: ControllerRegistry::Dashboard,
                },
                MenuItem {
                    name: String::from("Tasks"),
                    ctrl: ControllerRegistry::Task,
                },
                MenuItem {
                    name: String::from("Projects"),
                    ctrl: ControllerRegistry::Project,
                },
                //                MenuItem {
                //                   name: String::from("Contacts"),
                //                    ctrl: ControllerRegistry::Contact,
                //                },
            ],
            title: String::from("Rask, your task list manager"),
            active_item: 0,
            item_changed: false,
        }
    }
}

impl CtrObj for HeaderCtrl {
    fn init_data(&mut self) {}
    fn display(&mut self, f: &mut Frame, area: Rect) -> Result<(), Error> {
        header_ui(self, f, area);
        Ok(())
    }

    fn key_event_handler(&mut self, key: &KeyEvent) -> AppState {
        match key.code {
            KeyCode::Char('q') => return AppState::Closing,
            KeyCode::Tab => {
                self.set_next_active();
                return AppState::Running;
            }
            _ => {
                println!("key pressed '{:?}'", key.code);
            }
        }

        AppState::Running
    }
}

impl HeaderCtrl {
    pub fn set_next_active(&mut self) {
        self.active_item = self.active_item + 1;
        if self.active_item == self.menu.len() {
            self.active_item = 0;
        }
        //new menu item selected so a new controller should be loaded by
        //App
        self.item_changed = true;
    }

    pub fn controller_has_changed(&mut self) -> bool {
        self.item_changed
    }

    pub fn get_active_type(&mut self) -> ControllerRegistry {
        let active_menu = self.menu[self.active_item].clone();
        active_menu.ctrl
    }
    pub fn get_main_controller(&mut self) -> Box<dyn CtrObj> {
        self.item_changed = false; // since a controller has been requested we must consider that
                                   // the controller has bee updated and item_changed is now the
                                   // default
        get_controller_from_registry(self.get_active_type())
    }
}
