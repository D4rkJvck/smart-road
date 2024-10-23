use smart_road::{views::Interface, App};
use std::process;

/// As usual, make sure to minimize
/// the main function's responsibility.
/// In this case, the main will just
/// run the program and exit it when
/// needed.
fn main() -> Result<(), String> {
    let mut app = App::new(Interface::new()?)?;

    loop {
        if let Err(err) = app.run() {
            dbg!(err);
            process::exit(0)
        }
    }
}
