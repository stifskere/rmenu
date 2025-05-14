use std::cmp::min;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureValueError};
use sdl2::ttf::{Font, FontError};
use sdl2::video::{Window, WindowContext};
use thiserror::Error;

use crate::utils::keycode_to_char::keycode_to_char;
use crate::utils::vector_matrix::Vector2I;

#[derive(Error, Debug)]
pub enum TextInputError {
    #[error("A font error occurred\n- {0:#}")]
    Font(#[from] FontError),

    #[error("A texture error ocurred\n- {0:#}")]
    Texture(#[from] TextureValueError),

    #[error("A SDL error ocurred\n- {0}")]
    Sdl(String),
}

pub struct TextInput<'f> {
    caret_position: u16,
    buffer: Vec<char>,

    font: &'f Font<'f, 'f>,
    text_color: Color,

    position: Vector2I,
}

impl<'f> TextInput<'f> {
    #[inline]
    pub fn new(font: &'f Font) -> Self {
        Self {
            caret_position: 0,
            buffer: Vec::new(),

            font,
            text_color: Color::WHITE,

            position: Vector2I::new(0, 0),
        }
    }

    #[inline]
    pub fn insert_char_at_caret(&mut self, letter: char) {
        if self.buffer.len() < u16::MAX as usize {
            self.buffer
                .insert(self.caret_position as usize, letter);
            self.advance_caret();
        }
    }

    #[inline]
    pub fn remove_char_at_caret(&mut self) {
        if self.buffer.len() > 0 && self.caret_position > 0 {
            self.buffer
                .remove(self.caret_position as usize - 1);
            self.retreat_caret();
        }
    }

    #[inline]
    pub fn advance_caret(&mut self) {
        self.caret_position = min(self.caret_position + 1, self.buffer.len() as u16);
    }

    #[inline]
    pub fn retreat_caret(&mut self) {
        if self.caret_position > 0 {
            self.caret_position -= 1;
        }
    }

    #[inline]
    pub fn is_caret_at_end(&self) -> bool {
        self.caret_position as usize == self.buffer.len()
    }

    pub fn keycode_interaction(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Left => self.retreat_caret(),
            Keycode::Right => self.advance_caret(),

            _ => {},
        }
    }

    pub fn act_char_at_caret(&mut self, keycode: Keycode, shift: bool) {
        match keycode {
            Keycode::Backspace => self.remove_char_at_caret(),

            Keycode::Delete => {
                self.advance_caret();
                self.remove_char_at_caret();
            },

            keycode => {
                if let Some(char) = keycode_to_char(keycode, shift) {
                    self.insert_char_at_caret(char);
                }
            },
        }
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.text_color = color;
    }

    #[inline]
    pub fn set_position(&mut self, position: Vector2I) {
        self.position = position;
    }

    pub fn set_text(&mut self, text: &str) {
        self.buffer = text
            .chars()
            .collect();
        self.caret_position = self.buffer.len() as u16;
    }

    pub fn get_args(&self) -> Vec<String> {
        self.buffer
            .iter()
            .collect::<String>()
            .split(' ')
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
    }

    pub fn draw(
        &self,
        renderer: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<(), TextInputError> {
        let text = self
            .buffer
            .iter()
            .collect::<String>();

        let surface = self
            .font
            .render(&{ if text.is_empty() { " ".into() } else { text } })
            .blended(self.text_color)?;

        let texture = texture_creator.create_texture_from_surface(&surface)?;

        renderer
            .copy(
                &texture,
                None,
                Some(Rect::new(
                    self.position.x(),
                    self.position.y(),
                    surface.width(),
                    surface.height(),
                )),
            )
            .map_err(|e| TextInputError::Sdl(e))?;

        let carret_offset_x = if self.caret_position == 0 {
            0
        } else {
            let caret_str = self
                .buffer
                .iter()
                .take(self.caret_position as usize)
                .collect::<String>();

            self.font
                .size_of(&caret_str)
                .map(|(w, _)| w)
                .unwrap_or(0)
        };

        let curr_draw_color = renderer.draw_color();
        renderer.set_draw_color(self.text_color);

        renderer
            .fill_rect(Rect::new(
                self.position.x() as i32 + carret_offset_x as i32,
                self.position.y() as i32,
                2,
                surface.height(),
            ))
            .map_err(|e| TextInputError::Sdl(e))?;

        renderer.set_draw_color(curr_draw_color);

        Ok(())
    }
}
