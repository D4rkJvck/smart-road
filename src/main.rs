use smart_road::App;
use std::process;

/// As usual, make sure to minimize
/// the main function's responsibility.
/// In this case, the main will just
/// run the program and exit it when
/// needed.
fn main() -> Result<(), String> {
    loop {
        if let Err(msg) = App::new()?.run() {
            println!("{}", msg);
            process::exit(0)
        }
    }
}
