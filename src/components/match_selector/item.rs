use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

use crate::utils::errors::GenericComponentError;
use crate::utils::vector_matrix::{Vector2, Vector2I, Vector2U};

pub struct PagerItem<'f> {
    font: &'f Font<'f, 'f>,
    text: String,

    height: u32,
    padding: Vector2I,

    text_color: Color,
    highlight_color: Color,
    highlighted_text_color: Color,

    position: Vector2I,
}

impl<'f> PagerItem<'f> {
    pub(crate) fn new(font: &'f Font) -> Self {
        Self {
            font,
            text: " ".into(),

            height: 0,
            padding: Vector2::new(0, 0),

            text_color: Color::WHITE,
            highlight_color: Color::BLUE,
            highlighted_text_color: Color::WHITE,

            position: Vector2::new(0, 0),
        }
    }

    #[inline]
    pub fn set_position(&mut self, position: Vector2I) {
        self.position
            .set_x(position.x());
        self.position
            .set_y(position.y());
    }

    #[inline]
    pub const fn set_highlight_color(&mut self, color: Color) {
        self.highlight_color = color;
    }

    #[inline]
    pub const fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    #[inline]
    pub const fn set_padding(&mut self, padding: Vector2I) {
        self.padding = padding;
    }

    #[inline]
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    #[inline]
    pub const fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    #[inline]
    pub fn get_text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub const fn set_highlighted_text_color(&mut self, color: Color) {
        self.highlighted_text_color = color;
    }

    #[inline]
    pub fn get_size(&mut self) -> Result<Vector2U, GenericComponentError> {
        Ok(self
            .font
            .size_of(&self.text)
            .map(|s| s.into())?)
    }

    pub fn draw(
        &self,
        renderer: &mut Canvas<Window>,
        selected: bool,
    ) -> Result<(), GenericComponentError> {
        let texture_creator = renderer.texture_creator();
        let text_surface = self
            .font
            .render(&self.text)
            .blended(if selected { self.highlighted_text_color } else { self.text_color })?;
        let texture = texture_creator.create_texture_from_surface(&text_surface)?;

        if selected {
            let prev_draw_color = renderer.draw_color();
            renderer.set_draw_color(self.highlight_color);

            renderer.fill_rect(Rect::new(
                self.position.x(),
                self.position.y(),
                text_surface.width() + self.padding.x() as u32,
                text_surface.height() + self.padding.y() as u32 + self.height,
            ))?;

            renderer.set_draw_color(prev_draw_color);
        }

        renderer.copy(
            &texture,
            None,
            Some(Rect::new(
                self.position.x() + self.padding.x() / 2,
                self.position.y() + self.padding.y() / 2 + (self.height / 2) as i32
                    - (text_surface.height() / 2) as i32,
                text_surface.width(),
                text_surface.height(),
            )),
        )?;

        Ok(())
    }
}
