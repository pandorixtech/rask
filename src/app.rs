//use std::env;
use crate::controllers::footer_controller::FooterCtrl;
use crate::controllers::header_controller::HeaderCtrl;
use crate::controllers::*;
use crate::views::{destruct_terminal, init_terminal, masterview};
use crossterm::event::{self, Event};
use ratatui::Frame;
use std::io::Error;

pub enum AppState {
    Running,
    Closing,
    MoveOn,
}

pub struct App {
    header: HeaderCtrl,
    footer: FooterCtrl,
}

impl Default for App {
    fn default() -> App {
        App {
            header: HeaderCtrl::default(),
            footer: FooterCtrl::default(),
        }
    }
}

impl App {
    pub fn init(&mut self) {
        //@TODO load all shareable parts like db connection, terminal render and others
    }

    pub fn render(&mut self) -> Result<(), Error> {
        match init_terminal() {
            Ok(mut term) => {
                let mut main_controller = self.header.get_main_controller();

                loop {
                    if self.header.controller_has_changed() {
                        main_controller = self.header.get_main_controller();
                    }
                    self.footer.set_active_item(self.header.get_active_type());
                    main_controller.init_data();
                    term.draw(|f| self.ui_constructor(f, &mut main_controller))
                        .unwrap();

                    if let Event::Key(key) = event::read()? {
                        match main_controller.key_event_handler(&key) {
                            AppState::MoveOn => {}
                            _ => {
                                match self.header.key_event_handler(&key) {
                                    AppState::Closing => {
                                        destruct_terminal(term);
                                        return Ok(());
                                    }
                                    _ => {} // we do nothing
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => panic!("Problem initiating UI: {:?}", e),
        }
    }

    pub fn ui_constructor(&mut self, f: &mut Frame, main_controller: &mut Box<dyn CtrObj>) {
        let ui_zones = masterview(f);
        self.header.display(f, ui_zones[0]).unwrap();

        self.footer.display(f, ui_zones[2]).unwrap();

        main_controller.display(f, ui_zones[1]).unwrap();
    }
}
