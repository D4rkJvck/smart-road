use crate::views::Window;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Window::new()?,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.window.render()?;
        self.window.listen()
    }
}
