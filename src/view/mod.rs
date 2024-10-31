mod event;
mod render;

use crate::{HEIGHT, TITLE, WIDTH};
use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};

/// The Interface between the user
/// and the program.
/// Holds all the necessary tools
/// to interact with the user.
pub struct Interface {
    pub(super) canvas: Canvas<Window>,
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
    /// For this specific project, it will also initialize road limits.
    /// It finally initializes the instance of the `interface``.
    pub fn new() -> Result<Self, String> {
        // Initialize the SDL.
        let sdl_ctx = sdl2::init()?;
        let video_subsys = sdl_ctx.video()?;

        let window = video_subsys // Generate the window from the video subsystem
            .window(TITLE, WIDTH as u32, HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window // Turn the window into a canvas
            .into_canvas()
            .build()
            .unwrap();

        let texture_creator = canvas.texture_creator();

        let event_pump = sdl_ctx // Initialize a event pump to store user inputs
            .event_pump()
            .unwrap();

        Ok(Self {
            canvas,
            texture_creator,
            event_pump,
        })
    }
}
