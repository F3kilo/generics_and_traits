use crate::item::Item;
use crate::triple_work::TripleWorks;
#[derive(Debug)]
pub struct Tuple(u32, f32, f64);

impl TripleWorks for Tuple {
    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }
    fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }
    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }
}
