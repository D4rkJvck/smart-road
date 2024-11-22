mod event;
mod render;
mod utils;

use std::path::Path;

use sdl2::{
    render::{Canvas, TextureCreator},
    ttf::{self, Font},
    video::{Window, WindowContext},
    EventPump,
};
use utils::new_window;

/// The Interface between the user
/// and the program.
/// Holds all the necessary tools
/// to interact with the user.
pub struct Interface {
    pub(super) canvas: Canvas<Window>,
    pub(super) stats_canvas: Canvas<Window>,
    // pub(super) font: Font<_, 'static>,
    pub(super) texture_creator: TextureCreator<WindowContext>,
    pub(super) event_pump: EventPump,
}

impl Interface {
    /// This method holds all the logic around the `SDL` implementation.
    /// It initializes the "context" that will be then used to create
    /// a new "window" given a `title` and `dimensions`.
    /// Afterwards it turns the window to a "canvas".
    /// It also initializes a "event pump" from the context
    /// to get the `user` input `events`.
    /// For this specific project, it will also initialize intersection limits.
    /// It finally initializes the instance of the `interface``.
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        // Initialize the SDL.
        let sdl_ctx = sdl2::init()?;
        let mut video_subsys = sdl_ctx.video()?;

        let mut stats_canvas = new_window(&mut video_subsys, title, width / 2, height / 2)?;
        stats_canvas.window_mut().hide();

        let canvas = new_window(&mut video_subsys, title, width, height)?; // Generate a window from the video subsystem

        let ttf_ctx = ttf::init().map_err(|err| err.to_string())?;
        let font_path = Path::new("./assets/fonts/Doto-Bold.ttf");
        let font = ttf_ctx.load_font(font_path, 24)?;

        let texture_creator = canvas.texture_creator();

        let event_pump = sdl_ctx // Initialize a event pump to store user inputs
            .event_pump()
            .unwrap();

        Ok(Self {
            canvas,
            stats_canvas,
            texture_creator,
            event_pump,
        })
    }
}
