use crate::item::Item;
use crate::triple_work::TripleWorks;
#[derive(Debug)]
pub struct Array([f64; 3]);

impl TripleWorks for Array {
    fn default_values() -> Self {
        Self([0.0; 3])
    }
    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }
    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }
}
