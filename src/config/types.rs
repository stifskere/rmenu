use std::ops::Deref;

use sdl2::pixels::Color;
use thiserror::Error;
use toml_edit::Item as TomlItem;

use crate::utils::vector_matrix::{Vector2, Vector2F};

macro_rules! conf_err {
    (expected types: $($ty:ty),*) => {
        $crate::config::types::ConfigValueError::InvalidType { possible_types: vec![$(stringify!($ty)),*] }
    };

    (expected values: $($val:literal),*) => {
        $crate::config::types::ConfigValueError::InvalidValue { possible: vec![$($val),*] }
    }
}

fn join_possible(possible: &[&'static str]) -> String {
    let possible = possible.join(",");
    match possible.rfind(',') {
        Some(pos) => format!("{} or {}", &possible[..pos], &possible[pos + 1..]),
        None => possible,
    }
}

#[derive(Error, Debug)]
pub enum ConfigValueError {
    #[error(
        "Expected any of this types {possible}",
        possible = join_possible(.possible_types)
    )]
    InvalidType { possible_types: Vec<&'static str> },

    #[error(
        "Value should be {possible}",
        possible = join_possible(.possible)
    )]
    InvalidValue { possible: Vec<&'static str> },
}

#[derive(Clone, Copy, Debug)]
pub enum WindowPosition {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub struct ConfigVector2 {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ConfigColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct ConfigNumber(f64);

#[derive(Debug, Clone)]
pub struct ConfigString(String);

impl TryFrom<TomlItem> for WindowPosition {
    type Error = ConfigValueError;

    fn try_from(value: TomlItem) -> Result<Self, Self::Error> {
        if let Some(value) = value.as_str() {
            match value
                .to_lowercase()
                .as_str()
            {
                "top" => Ok(Self::Top),
                "bottom" => Ok(Self::Bottom),
                _ => Err(ConfigValueError::InvalidValue { possible: vec!["top", "bottom"] }),
            }
        } else {
            Err(conf_err!(expected types: String))
        }
    }
}

impl ConfigVector2 {
    #[inline]
    pub(super) const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl TryFrom<TomlItem> for ConfigVector2 {
    type Error = ConfigValueError;

    fn try_from(value: TomlItem) -> Result<Self, Self::Error> {
        if let Some(value) = value.as_array() {
            macro_rules! e_type { () => { conf_err!(expected values: "[decimal, decimal]") };}

            if value.len() != 2 {
                return Err(e_type!());
            }
            let Some(x) = value.get(0) else {
                return Err(e_type!());
            };
            let Some(y) = value.get(1) else {
                return Err(e_type!());
            };

            macro_rules! handle_value {
                ($v:ident) => {
                    if let Some(f) = $v.as_float() {
                        f
                    } else if let Some(i) = $v.as_integer() {
                        i as f64
                    } else {
                        return Err(e_type!());
                    }
                };
            }

            Ok(Self { x: handle_value!(x), y: handle_value!(y) })
        } else {
            Err(conf_err!(expected types: Vec<f64>, Vec<i64>))
        }
    }
}

impl Into<Vector2F> for ConfigVector2 {
    #[inline]
    fn into(self) -> Vector2F {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

impl ConfigColor {
    #[inline]
    pub(super) const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl TryFrom<TomlItem> for ConfigColor {
    type Error = ConfigValueError;

    fn try_from(value: TomlItem) -> Result<Self, Self::Error> {
        if let Some(integer) = value.as_integer() {
            return Ok(Self {
                r: ((integer >> 16) & 0xFF) as u8,
                g: ((integer >> 8) & 0xFF) as u8,
                b: (integer & 0xFF) as u8,
            });
        }

        if let Some(array) = value.as_array() {
            macro_rules! e_type { () => { conf_err!(expected values: "[integer, integer, integer]") };}

            if array.len() != 3 {
                return Err(e_type!());
            }

            let Some(r) = array.get(0) else {
                return Err(e_type!());
            };
            let Some(g) = array.get(1) else {
                return Err(e_type!());
            };
            let Some(b) = array.get(2) else {
                return Err(e_type!());
            };

            return Ok(Self {
                r: r.as_integer()
                    .ok_or(e_type!())? as u8,
                g: g.as_integer()
                    .ok_or(e_type!())? as u8,
                b: b.as_integer()
                    .ok_or(e_type!())? as u8,
            });
        }

        Err(conf_err!(expected types: Vec<u8>, u32))
    }
}

impl Into<Color> for ConfigColor {
    #[inline]
    fn into(self) -> Color {
        Color::RGB(self.r, self.g, self.b)
    }
}

impl ConfigNumber {
    #[inline]
    pub(crate) const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl TryFrom<TomlItem> for ConfigNumber {
    type Error = ConfigValueError;

    fn try_from(value: TomlItem) -> Result<Self, Self::Error> {
        Ok(Self(if let Some(as_float) = value.as_float() {
            as_float
        } else if let Some(as_int) = value.as_integer() {
            as_int as f64
        } else {
            return Err(conf_err!(expected types: f64));
        }))
    }
}

impl Deref for ConfigNumber {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<TomlItem> for ConfigString {
    type Error = ConfigValueError;

    fn try_from(value: TomlItem) -> Result<Self, Self::Error> {
        value
            .as_str()
            .map(|s| Self(s.to_string()))
            .ok_or(conf_err!(expected types: String))
    }
}

impl Deref for ConfigString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
