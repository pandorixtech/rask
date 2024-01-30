//use std::env;

//use std::result::Result;
use rask::app::App;
//use argh::FromArgs;
use std::io;
//use std::{ io, time::Duration};
//use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
//use tui::{backend::termion::TermionBackend, Terminal};
//use notify_rust::Notification;


#[allow(deprecated)]
fn main() -> Result<(), io::Error> {
    /*
    println!("Rask your life!!!");
    let mut db_path = "".to_string();
    
    match env::home_dir() {
        Some(path) => db_path = format!("{}", path.display()).to_string(),
        None => println!("Gaita no Home dir!!!!")
    }
    //let conn = Connection::open_in_memory()?;
    let conn = start_db(db_path)?;
    init_db(&conn)?;
    let me = person::Person::new(-1,"Xico".to_string(), "Fininho".to_string(), "xf@garrgle.info".to_string());
    let saved_me = me.save(&conn)?;
    let the_list = person::Person::list(&conn, "".to_string())?;
    
    println!("me saved {:?}", saved_me);
    println!("the list: {:?}", the_list);
    */
    /*
    Notification::new()
        .summary("Hello world")
        .body("This is an example notification.")
        .icon("dialog-information")
        .show().unwrap(); 
    */
    //run_ui();
    //
    let mut app = App::default();
    app.init();

    app.render()


}


//#[cfg(target_os = "macos")]
//fn main() -> Result<(), Box<dyn std::error::Error>> {
/*
    Notification::new()
        .summary("Critical Error")
        .body("Just <b>kidding</b>, this is just the notificationexample.")
        .icon("dialog-error")
        .show()?;
    Ok(())
*/
//    use notify_rust::{
//        get_bundle_identifier_or_default, set_application, Notification,
//    };

 //   let safari_id = get_bundle_identifier_or_default("Terminal"); // get_bundle_identifier_or_default("Rask");
//    println!("app id = {:?}", safari_id);
//    set_application(&safari_id)?;
//   set_application(&safari_id)?;
/*
    Notification::new()
        .summary("Rask TEST")
        .body("This is a test to show notifications from terminal.")
        .appname("Rask")
        .icon("Safari")
        .show()?;

    Ok(())
*/
//}
