#[derive(Error, Debug)]
pub enum GenericComponentError {
    #[error("A font error occurred\n- {0:#}")]
    Font(#[from] FontError),

    #[error("A texture error ocurred\n- {0:#}")]
    Texture(#[from] TextureValueError),

    #[error("A generic SDL error ocurred\n- {0}")]
    Sdl(String),
}

impl From<String> for GenericComponentError {
    fn from(value: String) -> Self {
        Self::Sdl(value)
    }
}

macro_rules! handle_app_error {
    ($expr:expr) => {
        match { $expr } {
            Ok(v) => v,
            Err(e) => {
                ::log::error!("FATAL ERROR: {:#}", e);
                ::std::process::exit(-1);
            },
        }
    };
}

pub(crate) use handle_app_error;
use sdl2::render::TextureValueError;
use sdl2::ttf::FontError;
use thiserror::Error;
