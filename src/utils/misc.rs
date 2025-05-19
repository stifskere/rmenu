use std::sync::OnceLock;

use sdl2::rect::Point;
use sdl2::sys::SDL_GetGlobalMouseState;
use sdl2::ttf::{InitError, Sdl2TtfContext, init as ttf_init};
use sdl2::VideoSubsystem;

static TTF_CONTEXT: OnceLock<Sdl2TtfContext> = OnceLock::new();

pub fn ttf_context() -> Result<&'static Sdl2TtfContext, InitError> {
    if let Some(context) = TTF_CONTEXT.get() {
        return Ok(context);
    }

    let context = ttf_init()?;
    Ok(TTF_CONTEXT.get_or_init(|| context))
}

pub fn find_mouse_monitor(video: &VideoSubsystem) -> Result<Option<i32>, String> {
    let mut x = 0;
    let mut y = 0;

    unsafe {
        SDL_GetGlobalMouseState(&mut x, &mut y);
    }

    for i in 0..video.num_video_displays()? {
        if let Ok(bounds) = video.display_bounds(i) {
            if bounds.contains_point(Point::new(x, y)) {
                return Ok(Some(i));
            }
        }
    }

    Ok(None)
}
