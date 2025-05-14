use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::Window;

use crate::utils::errors::GenericComponentError;
use crate::utils::vector_matrix::{Vector2, Vector2I, Vector2U};

pub struct PagerItem<'f> {
    text_surface: Surface<'f>,
    original_text: String,

    padding: Vector2I,

    selected_background: Color,
    position: Vector2I,
}

impl<'f> PagerItem<'f> {
    pub(crate) fn new(
        font: &'f Font,
        text: &str,
        color: Color,
    ) -> Result<Self, GenericComponentError> {
        Ok(Self {
            text_surface: font
                .render(text)
                .blended(color)?,
            original_text: text.to_string(),

            padding: Vector2::new(0, 0),

            selected_background: Color::BLUE,
            position: Vector2::new(0, 0),
        })
    }

    #[inline]
    pub fn set_position(&mut self, position: Vector2I) {
        self.position
            .set_x(position.x());
        self.position
            .set_y(position.y());
    }

    #[inline]
    pub fn set_selected_background(&mut self, color: Color) {
        self.selected_background = color;
    }

    #[inline]
    pub fn set_padding(&mut self, padding: Vector2I) {
        self.padding = padding;
    }

    #[inline]
    pub fn get_original_text(&self) -> &str {
        &self.original_text
    }

    #[inline]
    pub fn get_size(&self) -> Vector2U {
        self.text_surface
            .size()
            .into()
    }

    pub fn draw(
        &self,
        renderer: &mut Canvas<Window>,
        selected: bool,
    ) -> Result<(), GenericComponentError> {
        let texture_creator = renderer.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&self.text_surface)?;

        if selected {
            let prev_draw_color = renderer.draw_color();
            renderer.set_draw_color(self.selected_background);

            renderer.fill_rect(Rect::new(
                self.position.x(),
                self.position.y(),
                self.text_surface
                    .width()
                    + self.padding.x() as u32,
                self.text_surface
                    .height()
                    + self.padding.y() as u32,
            ))?;

            renderer.set_draw_color(prev_draw_color);
        }

        renderer.copy(
            &texture,
            None,
            Some(Rect::new(
                self.position.x() + self.padding.x() / 2,
                self.position.y() + self.padding.y() / 2,
                self.text_surface
                    .width(),
                self.text_surface
                    .height(),
            )),
        )?;

        Ok(())
    }
}
