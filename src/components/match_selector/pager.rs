use std::cmp::min;
use std::ops::Sub;
use std::ptr::eq as ptr_eq;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

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

    select_color: Color,

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

            select_color: Color::WHITE,

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

        let mut x_offset = 0;
        let mut current_page = Vec::new();
        for entry in &self.provided_entries {
            if !entry.starts_with(text) {
                continue;
            }

            let mut entry = PagerItem::new(self.font, &entry, self.text_color)?;
            let entry_size = entry.get_size();

            if entry_size.x() + x_offset > self.rect.width() - 50 {
                self.computed_entries
                    .push(current_page);
                current_page = Vec::new();
                x_offset = 0;
            }

            entry.set_position(Vector2::new(self.rect.x() + x_offset as i32, self.rect.y()));
            entry.set_selected_background(self.select_color);
            entry.set_padding(Vector2::new(20, 0));

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
    pub fn is_caret_at_start(&self) -> bool {
        self.caret_position == 0
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
    pub fn set_select_color(&mut self, select_color: Color) {
        self.select_color = select_color;
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

    pub fn get_selected_entry(&self) -> Option<(&Vec<PagerItem>, &PagerItem)> {
        let mut count = 0;

        for (page_index, page) in self
            .computed_entries
            .iter()
            .enumerate()
        {
            if self.caret_position < count + page.len() {
                let page = self
                    .computed_entries
                    .get(page_index)?;

                return Some((page, page.get(self.caret_position - count)?));
            }

            count += page.len();
        }

        None
    }

    pub fn draw(&self, renderer: &mut Canvas<Window>) -> Result<(), GenericComponentError> {
        if let Some((page, selected_entry)) = self.get_selected_entry() {
            for entry in page {
                entry.draw(renderer, ptr_eq(entry, selected_entry))?;
            }
        }

        Ok(())
    }
}
