use std::sync::OnceLock;

use sdl2::ttf::{Sdl2TtfContext, init as ttf_init, InitError};


static TTF_CONTEXT: OnceLock<Sdl2TtfContext> = OnceLock::new();

pub fn ttf_context() -> Result<&'static Sdl2TtfContext, InitError> {
    if let Some(context) = TTF_CONTEXT.get() {
        return Ok(context);
    }

    let context = ttf_init()?;
    Ok(TTF_CONTEXT.get_or_init(|| context))
}
