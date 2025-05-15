use super::item::PagerItem;
use super::pager::Pager;

pub struct PagerCursor<'i> {
    page_index: usize,
    page: &'i Vec<PagerItem<'i>>,

    item_index: usize,
    item: &'i PagerItem<'i>,
}

impl<'i> PagerCursor<'i> {
    pub(super) fn from_instance(instance: &'i Pager) -> Option<Self> {
        let mut count = 0;

        for (page_index, page) in instance
            .computed_entries()
            .iter()
            .enumerate()
        {
            if instance.caret_position() < count + page.len() {
                let item_index = instance.caret_position() - count;

                return Some(Self {
                    page,
                    page_index,

                    item: page.get(item_index)?,
                    item_index,
                });
            }

            count += page.len();
        }

        None
    }

    #[inline]
    pub fn page_index(&self) -> usize {
        self.page_index
    }

    #[inline]
    pub fn page(&self) -> &Vec<PagerItem<'i>> {
        self.page
    }

    #[inline]
    #[allow(unused)]
    pub fn item_index(&self) -> usize {
        self.item_index
    }

    #[inline]
    pub fn item(&self) -> &PagerItem<'i> {
        self.item
    }
}
