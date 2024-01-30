use rusqlite::{params, Connection, Result};
use std::env;
use std::path::Path;
pub mod app;
pub mod controllers;
pub mod models;
pub mod views;
/*
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::{sync::mpsc, thread, time::Duration};

pub fn setup_terminal() -> Terminal<CrosstermBackend<io::Stdout>> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).unwrap()
}

pub fn destruct_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) {
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
    terminal.show_cursor().unwrap();
}
*/

/**
 * database helpers fn
 **/

pub fn init_db(conn: &Connection) -> Result<usize> {
    let dbtables = vec![
    "CREATE TABLE IF NOT EXISTS person(id INTEGER PRIMARY KEY, first_name TEXT NOT NULL, last_name TEXT NOT NULL, email TEXT NOT NULL)",
    "CREATE TABLE IF NOT EXISTS project(id INTEGER PRIMARY KEY,reference TEXT NOT NULL, name TEXT NOT NULL, description TEXT, created_by INTEGER NOT NULL, start_date TEXT, end_date TEXT, created_at INTEGER, updated_at INTEGER )",
    "CREATE TABLE IF NOT EXISTS task (id INTEGER PRIMARY KEY,project_id INTEGER, parent_id INTEGER, name TEXT NOT NULL, description NOT NULL, weight INTEGER, status INTEGER, created_by INTEGER, created_at INTEGER, updated_at INTEGER)",
    "CREATE TABLE IF NOT EXISTS task_status (id INTEGER UNIQUE PRIMARY KEY, name TEXT NOT NULL)",
    "INSERT OR IGNORE INTO task_status (id, name) VALUES ('1', 'BACKLOG'),('2', 'WIP'),('3', 'DONE')",

    ];
    let tbiter = dbtables.iter();
    let mut rv = conn.execute("select (1+1)", params![]);
    let mut br = 0;
    for val in tbiter {
        rv = conn.execute(val, params![]);
        /*
        match rv {
            Ok(n) => println!("Success {}", n),
            Err(ref e) => br=1
        }
        */
        match rv {
            Ok(_n) => (),
            Err(ref _e) => br = 1,
        }

        if br > 0 {
            break;
        }
    }

    rv
}

pub fn db_open(db_path: String) -> Result<Connection> {
    Connection::open(db_path)
}

pub fn start_db(home_dir: String) -> Result<Connection> {
    let db_path = Path::new(home_dir.as_str()).join("rask.db");
    let mut do_init_db = false;
    if !db_path.exists() {
        do_init_db = true;
    }
    let path_str = match db_path.to_str() {
        Some(s) => String::from(str::replace(s, "\"", "")),
        None => panic!("invalid db path"),
    };

    let connection_rv = db_open(String::from(path_str));
    match connection_rv {
        Ok(conn) => {
            if do_init_db {
                let _init_result = init_db(&conn);
            }

            Ok(conn)
        }
        Err(e) => {
            println!("Failed to connect to db: {:?}", e);
            Err(e)
        }
    }
}

pub trait UtilFns: Clone {
    fn get_homedir() -> String {
        format!(
            "{:?}",
            env::current_dir().expect("No home dir defined in this system")
        )
        .to_string()
    }

    fn get_db_connection() -> Result<Connection, rusqlite::Error> {
        start_db(Self::get_homedir())
    }
}
