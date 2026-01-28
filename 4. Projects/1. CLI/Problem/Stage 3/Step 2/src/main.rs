use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

use std::io::stdout;
use std::io::Write;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut navigator = Navigator::new(Rc::clone(&db));
    loop {
        clearscreen::clear().unwrap();
        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        if let Some(page) = navigator.get_current_page() {
            // 2. render page
            if let Err(error) = &page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            };
            println!("what?");
            // 3. get user input
            let input = get_user_input();

            // 4. pass input to page's input handler
            // 5. if the page's input handler returns an action let the navigator process the action
            match page.handle_input(input.trim()).ok() {
                Some(action) => {
                    if let Some(action) = action {
                        if let Err(err) = navigator.handle_action(action) {
                            println!("someting went wrong: {}", err);
                            wait_for_key_press();
                        }
                    }
                }
                None => {
                    wait_for_key_press();
                }
            }
        } else {
            break;
        }
    }
}
