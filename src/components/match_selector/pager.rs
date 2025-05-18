use std::cmp::min;
use std::ops::Sub;
use std::ptr::eq as ptr_eq;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

use super::cursor::PagerCursor;
use super::item::PagerItem;
use crate::utils::errors::GenericComponentError;
use crate::utils::vector_matrix::{Vector2, Vector2I, Vector2U};

pub struct Pager<'f> {
    provided_entries: Vec<String>,
    computed_entries: Vec<Vec<PagerItem<'f>>>,

    caret_position: usize,
    last_matched: Option<String>,

    font: &'f Font<'f, 'f>,
    text_color: Color,

    highlight_color: Color,
    highlighted_text_color: Color,

    rect: Rect,
}

impl<'f> Pager<'f> {
    pub fn new(mut entries: Vec<String>, font: &'f Font) -> Self {
        entries.sort_by_key(|e| e.to_lowercase());

        Self {
            computed_entries: Vec::with_capacity(entries.len()),
            provided_entries: entries,

            caret_position: 0,
            last_matched: None,

            font,
            text_color: Color::WHITE,

            highlight_color: Color::WHITE,
            highlighted_text_color: Color::WHITE,

            rect: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn compute_text(&mut self, text: &str) -> Result<(), GenericComponentError> {
        if let Some(last_matched) = &self.last_matched {
            if last_matched == text {
                return Ok(());
            }
        }

        self.last_matched = Some(text.to_string());
        self.computed_entries
            .clear();

        const RIGHT_PAD: u32 = 150;
        const LEFT_PAD: i32 = 10;

        let mut x_offset = 0;
        let mut current_page = Vec::new();
        for entry_text in &self.provided_entries {
            if !entry_text.starts_with(text) {
                continue;
            }

            let mut entry = PagerItem::new(&self.font);
            entry.set_text(entry_text);

            let entry_size = entry.get_size()?;

            if entry_size.x() + x_offset > self.rect.width() - RIGHT_PAD {
                self.computed_entries
                    .push(current_page);

                current_page = Vec::new();
                x_offset = 0;
            }

            entry.set_position(Vector2::new(
                LEFT_PAD + self.rect.x() + x_offset as i32,
                self.rect.y(),
            ));
            entry.set_highlighted_text_color(self.highlighted_text_color);
            entry.set_highlight_color(self.highlight_color);
            entry.set_text_color(self.text_color);
            entry.set_padding(Vector2::new(20, 0));
            entry.set_height(self.rect.height());

            current_page.push(entry);

            x_offset += entry_size.x() as u32 + 20;
        }

        if !current_page.is_empty() {
            self.computed_entries
                .push(current_page);
        }

        self.caret_position = 0;

        Ok(())
    }

    pub fn advance_caret(&mut self) {
        self.caret_position = min(
            self.caret_position + 1,
            self.computed_entries
                .iter()
                .map(|page| page.len())
                .sum::<usize>()
                .sub(1),
        );
    }

    #[inline]
    pub fn retreat_caret(&mut self) {
        if self.caret_position > 0 {
            self.caret_position -= 1;
        }
    }

    #[inline]
    pub const fn is_caret_at_start(&self) -> bool {
        self.caret_position == 0
    }

    #[inline]
    pub(super) const fn caret_position(&self) -> usize {
        self.caret_position
    }

    #[inline]
    pub(super) const fn computed_entries(&self) -> &Vec<Vec<PagerItem>> {
        &self.computed_entries
    }

    #[inline]
    pub fn get_selected_entry(&self) -> Option<PagerCursor> {
        PagerCursor::from_instance(self)
    }

    #[inline]
    pub fn keycode_interaction(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Right => self.advance_caret(),

            Keycode::Left => self.retreat_caret(),

            _ => {},
        }
    }

    #[inline]
    pub fn set_text_color(&mut self, text_color: Color) {
        self.text_color = text_color;
    }

    #[inline]
    pub fn set_highlight_color(&mut self, select_color: Color) {
        self.highlight_color = select_color;
    }

    #[inline]
    pub fn set_position(&mut self, position: Vector2I) {
        self.rect
            .set_x(position.x());
        self.rect
            .set_y(position.y());
    }

    #[inline]
    pub fn set_size(&mut self, size: Vector2U) {
        self.rect
            .set_width(size.x());
        self.rect
            .set_height(size.y());
    }

    #[inline]
    pub const fn set_highlighted_text_color(&mut self, color: Color) {
        self.highlighted_text_color = color;
    }

    pub fn draw(&self, renderer: &mut Canvas<Window>) -> Result<(), GenericComponentError> {
        let Some(selected) = self.get_selected_entry() else { return Ok(()) };

        let arrow_colors = Color::RGB(
            (self.text_color.r as f32 * 0.9) as u8,
            (self.text_color.g as f32 * 0.9) as u8,
            (self.text_color.b as f32 * 0.9) as u8,
        );

        let texture_creator = renderer.texture_creator();

        if selected.page_index() > 0 {
            let arrow_left = self
                .font
                .render_char('<')
                .blended(arrow_colors)?;

            let arrow_left_texture = texture_creator.create_texture_from_surface(arrow_left)?;

            renderer.copy(
                &arrow_left_texture,
                None,
                Some(Rect::new(self.rect.x(), self.rect.y(), 5, self.rect.height())),
            )?;
        }

        if selected.page_index()
            < self
                .computed_entries
                .len()
                - 1
        {
            let arrow_right = self
                .font
                .render_char('>')
                .blended(arrow_colors)?;

            let arrow_right_texture = texture_creator.create_texture_from_surface(arrow_right)?;

            renderer.copy(
                &arrow_right_texture,
                None,
                Some(Rect::new(
                    self.rect.x() + self.rect.width() as i32 - 20,
                    self.rect.y(),
                    5,
                    self.rect.height(),
                )),
            )?;
        }

        for entry in selected.page() {
            entry.draw(renderer, ptr_eq(entry, selected.item()))?;
        }

        Ok(())
    }
}
