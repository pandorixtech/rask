use super::{ControllerRegistry, CtrObj};
use crate::app::AppState;
use crate::views::footer::{dashboard_footer_ui, footer_ui};
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use std::io::Error;

#[derive(Debug, Clone)]
pub struct FooterCtrl {
    pub active_item: ControllerRegistry,
}

impl Default for FooterCtrl {
    fn default() -> FooterCtrl {
        FooterCtrl {
            active_item: ControllerRegistry::Dashboard,
        }
    }
}

impl CtrObj for FooterCtrl {
    fn init_data(&mut self) {}
    fn display(&mut self, f: &mut Frame, area: Rect) -> Result<(), Error> {
        match self.active_item {
            ControllerRegistry::Dashboard => dashboard_footer_ui(self, f, area),
            _ => footer_ui(self, f, area),
        }

        Ok(())
    }

    fn key_event_handler(&mut self, _key: &KeyEvent) -> AppState {
        //@NOTE footer ctrl is only used to disply footer content
        AppState::Running
    }
}

impl FooterCtrl {
    pub fn set_active_item(&mut self, item: ControllerRegistry) {
        self.active_item = item;
    }
}
