use smart_road::App;

/// As usual, make sure to minimize
/// the main function's responsibility.
/// In this case, the main will just
/// run the program and exit it when
/// needed.
#[tokio::main]
async fn main() -> Result<(), String> {
    let mut app = App::new()?;

    if let Err(msg) = app.run().await {
        dbg!(msg);
    }

    Ok(())
}
