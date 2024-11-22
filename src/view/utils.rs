use sdl2::{render::Canvas, video::Window, VideoSubsystem};

use crate::controller::Statistics;

pub fn new_window(
    video_subsys: &mut VideoSubsystem,
    title: &str,
    width: i32,
    height: i32,
) -> Result<Canvas<Window>, String> {
    Ok(video_subsys
        .window(title, width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap()
        .into_canvas()
        .build()
        .unwrap())
}
