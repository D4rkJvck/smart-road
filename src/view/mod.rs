mod render;
mod stats;

use sdl2::{
    render::{Canvas, TextureCreator},
    ttf::{self, Sdl2TtfContext},
    video::{Window, WindowContext},
    EventPump,
};

/// The Interface between the user
/// and the program.
/// Holds all the necessary tools
/// to interact with the user.
pub struct Interface {
    pub(super) canvas: Canvas<Window>,
    pub(super) event_pump: EventPump,
    pub(super) ttf_ctx: Sdl2TtfContext,
    pub(super) texture_creator: TextureCreator<WindowContext>,
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
        let video_subsys = sdl_ctx.video()?;

        let window = video_subsys
            .window(title, width as u32, height as u32)
            .position_centered()
            .build()
            .map_err(|err| format!("Window! -> {}", err))?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|err| format!("Canvas! -> {}", err))?;

        let event_pump = sdl_ctx
            .event_pump()
            .map_err(|err| format!("Event Pump! -> {}", err))?;

        let ttf_ctx = ttf::init().map_err(|err| format!("Context! -> {}", err))?;

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            canvas,
            event_pump,
            ttf_ctx,
            texture_creator,
        })
    }
}
