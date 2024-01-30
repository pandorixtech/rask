use crate::app::AppState;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use std::io::Error;

pub mod contact_controller;
pub mod dashboard_controller;
pub mod footer_controller;
pub mod header_controller;
pub mod project_controller;
pub mod task_controller;

pub trait CtrObj: std::fmt::Debug {
    fn display(&mut self, f: &mut Frame, area: Rect) -> Result<(), Error>;

    fn key_event_handler(&mut self, key: &KeyEvent) -> AppState;

    fn init_data(&mut self);
}

#[derive(Debug, Clone)]
pub enum CtrlActions {
    List,
    Edit,
    Del,
    Detail,
}

#[derive(Debug, Clone)]
pub enum ControllerRegistry {
    Project,
    Contact,
    Task,
    Dashboard,
}

pub fn get_controller_from_registry(reference: ControllerRegistry) -> Box<dyn CtrObj> {
    match reference {
        ControllerRegistry::Dashboard => {
            return Box::new(dashboard_controller::DashboardCtrl::default())
        }
        ControllerRegistry::Task => return Box::new(task_controller::TaskCtrl::default()),
        ControllerRegistry::Project => return Box::new(project_controller::ProjectCtrl::default()),
        ControllerRegistry::Contact => return Box::new(contact_controller::ContactCtrl::default()),
    }
}
