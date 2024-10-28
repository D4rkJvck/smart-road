use crate::views::Window;

pub struct App {
    interface: Window,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            interface: Window::new()?,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.interface.render()?;
        self.interface.listen()
    }
}
