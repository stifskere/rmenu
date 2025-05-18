use std::env::var;
use std::fs::{read_to_string, write as fs_write};
use std::io::Error as IoError;
use std::path::Path;
use std::u16;

use log::{info, warn};
use sdl2::pixels::Color;
use sdl2::rwops::RWops;
use sdl2::ttf::{InitError as TTFInitError, Font};
use thiserror::Error;
use toml_edit::{DocumentMut, Item as TomlItem, TomlError};

use super::types::{
    ConfigColor,
    ConfigNumber,
    ConfigString,
    ConfigValueError,
    ConfigVector2,
    WindowPosition,
};
use crate::utils::misc::ttf_context;
use crate::utils::vector_matrix::Vector2F;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid configuration entry '{key}': {message:#}.")]
    InvalidValue {
        key: &'static str,
        message: ConfigValueError,
    },

    #[error("Io error occurred: {0:#}")]
    Io(#[from] IoError),

    #[error("Toml parse error: {0:#}")]
    Toml(#[from] TomlError),

    #[error("Invalid path '{path}' as {path_use}, not a file")]
    NotAFile { path: String, path_use: String },

    #[error("TTF init error: {0:#}")]
    TTFError(#[from] TTFInitError),

    #[error("Font error ocurred: {message}")]
    GenericFontError {
        message: String
    }
}

pub struct Config<'f> {
    // Whether the window spawns
    // on top or on the bottom.
    window_position: WindowPosition,

    // The separation between window borders, x for
    // left/right and y for the closest border
    // depending on the set window position.
    window_padding: Vector2F,

    // The launcher bar height.
    window_height: u32,

    // The whole window color.
    window_background_color: Color,

    // The text color, arrows and completion are
    // that multiplied by 0.9.
    text_color: Color,

    // The color of the cursor in the command
    // completion right menu.
    highlight_color: Color,

    // The selected text color, under highlight
    // the original color might opaque.
    highlighted_text_color: Color,

    // The font that will render
    // all the text in the window.
    font: Option<Font<'f, 'f>>,
}

impl<'f> Config<'f> {
    pub fn load() -> Result<Self, ConfigError> {
        let string_config_path = var("RMENU_CONFIG_PATH")
            .unwrap_or("./.rmenu.toml".into());

        info!("Read 'RMENU_CONFIG_PATH', found value '{string_config_path}'");

        let config_path = Path::new(&string_config_path);

        if config_path.is_dir() {
            return Err(ConfigError::NotAFile {
                path: string_config_path,
                path_use: "config file".into(),
            });
        }

        if !config_path.exists() {
            fs_write(config_path, include_str!("../../assets/default_config.toml"))?;
            warn!(
                "A configuration file at '{string_config_path}' could not be found, it was \
                 created."
            );
        }

        read_to_string(config_path)?
            .parse::<DocumentMut>()?
            .try_into()
    }

    #[inline]
    pub const fn window_position(&self) -> WindowPosition {
        self.window_position
    }

    #[inline]
    pub const fn window_padding(&self) -> Vector2F {
        self.window_padding
    }

    #[inline]
    pub const fn window_height(&self) -> u32 {
        self.window_height
    }

    #[inline]
    pub const fn window_background_color(&self) -> Color {
        self.window_background_color
    }

    #[inline]
    pub const fn highlighted_text_color(&self) -> Color {
        self.highlighted_text_color
    }

    #[inline]
    pub const fn text_color(&self) -> Color {
        self.text_color
    }

    #[inline]
    pub const fn highlight_color(&self) -> Color {
        self.highlight_color
    }

    #[inline]
    pub const fn font(&self) -> Option<&Font<'f, 'f>> {
        self.font
            .as_ref()
    }
}

impl<'f> TryFrom<DocumentMut> for Config<'f> {
    type Error = ConfigError;

    fn try_from(value: DocumentMut) -> Result<Self, Self::Error> {
        macro_rules! handle_value {
            ($key:ident: $type:ty) => {
                value.get(stringify!($key))
            .map(|value| <$type as TryFrom<TomlItem>>::try_from(value.clone()))
                    .transpose()
                    .map_err(|err|
                        ConfigError::InvalidValue { key: stringify!($key), message: err }
                    )?
                    .inspect(|value| info!("Loaded {} as {:?}", stringify!($key), value))
            };

            ($key:ident: $type:ty | $fallback:expr) => {
                handle_value!($key: $type)
                    .unwrap_or_else(|| {
                        let fallback = $fallback;

                        warn!(
                            "'{}' was not found in the configuration, falling back to {:?}",
                            stringify!($key),
                            &fallback
                        );

                        fallback
                    })
            }
        }

        Ok(Self {
            window_position: handle_value!(window_position: WindowPosition | WindowPosition::Top),
            window_padding: handle_value!(window_padding: ConfigVector2 | ConfigVector2::new(0.0, 0.0))
                .into(),
            window_height: *handle_value!(window_height: ConfigNumber | ConfigNumber::new(6.0)) as u32,

            text_color: handle_value!(text_color: ConfigColor | ConfigColor::new(255, 255, 255))
                .into(),
            highlight_color: handle_value!(highlight_color: ConfigColor | ConfigColor::new(102, 102, 102))
                .into(),
            highlighted_text_color: handle_value!(highlighted_text_color: ConfigColor | ConfigColor::new(255, 255, 255))
                .into(),
            window_background_color: handle_value!(window_background_color: ConfigColor | ConfigColor::new(41, 41, 41))
                .into(),

            font: if let Some(font_path) = handle_value!(font_path: ConfigString) {
                Some({
                    let ttf_context = ttf_context()?;
                    let font_size = *handle_value!(font_size: ConfigNumber | ConfigNumber::new(14.0));

                    {
                        let font_path = Path::new(&*font_path);

                        if !font_path.exists() {
                            return Err(ConfigError::GenericFontError {
                                message: "The font file does not exist.".to_string()
                            });
                        }
                    }

                    ttf_context
                        .load_font_from_rwops(
                            RWops::from_file(&*font_path, "")
                                .map_err(|err| ConfigError::GenericFontError { message: err })?,
                            font_size.clamp(0.0, u16::MAX as f64) as u16
                        )
                        .map_err(|err| ConfigError::GenericFontError{ message: err })?
                })
            } else {
                None
            },
        })
    }
}
