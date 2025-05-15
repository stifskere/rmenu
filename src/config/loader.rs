use std::env::var;
use std::fs::{read_to_string, write as fs_write};
use std::io::Error as IoError;
use std::path::Path;

use log::{info, warn};
use thiserror::Error;
use toml_edit::{DocumentMut, Item as TomlItem, TomlError};

use super::enums::{ConfigColor, ConfigValueError, ConfigVector2, WindowPosition};
use crate::config::enums::ConfigNumber;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid value '{key}' {message:#}.")]
    InvalidValue {
        key: &'static str,
        message: ConfigValueError,
    },

    #[error("Io error occurred: {0:#}")]
    Io(#[from] IoError),

    #[error("Toml parse error: {0:#}")]
    Toml(#[from] TomlError),

    #[error("Invalid path '{path}', not a file")]
    NotAFile { path: String },
}

pub struct Config {
    // Whether the window spawns
    // on top or on the bottom.
    position: WindowPosition,

    // The separation between window borders, x for
    // left/right and y for the closest border
    // depending on the set window position.
    padding: ConfigVector2,

    /// The launcher bar height.
    height: ConfigNumber,

    // The whole window color.
    background_color: ConfigColor,

    // The text color, arrows and completion are
    // that multiplied by 0.9.
    text_color: ConfigColor,

    // The color of the cursor in the command
    // completion right menu.
    selection_color: ConfigColor,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let string_config_path = var("RMENU_CONFIG_PATH").unwrap_or("./.rmenu.toml".into());
        let config_path = Path::new(&string_config_path);

        if config_path.is_dir() {
            return Err(ConfigError::NotAFile { path: string_config_path });
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
    pub const fn position(&self) -> WindowPosition {
        self.position
    }

    #[inline]
    pub const fn padding(&self) -> &ConfigVector2 {
        &self.padding
    }

    #[inline]
    pub const fn height(&self) -> &ConfigNumber {
        &self.height
    }

    #[inline]
    pub const fn background_color(&self) -> &ConfigColor {
        &self.background_color
    }

    #[inline]
    pub const fn text_color(&self) -> &ConfigColor {
        &self.text_color
    }

    #[inline]
    pub const fn selection_color(&self) -> &ConfigColor {
        &self.selection_color
    }
}

impl TryFrom<DocumentMut> for Config {
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
            position: handle_value!(position: WindowPosition | WindowPosition::Top),
            padding: handle_value!(padding: ConfigVector2 | ConfigVector2::new(0.0, 0.0)),
            height: handle_value!(height: ConfigNumber | ConfigNumber::new(6.0)),

            text_color: handle_value!(text_color: ConfigColor | ConfigColor::new(255, 255, 255)),
            selection_color: handle_value!(selection_color: ConfigColor | ConfigColor::new(102, 102, 102)),
            background_color: handle_value!(background_color: ConfigColor | ConfigColor::new(41, 41, 41)),
        })
    }
}
