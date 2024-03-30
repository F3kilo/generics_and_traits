use crate::item::Item;
pub trait TripleWorks {
    fn default_values() -> Self;
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);
    fn is_default(&self) -> bool {
        self.get_item(Item::First) == 0.0
            && self.get_item(Item::Second) == 0.0
            && self.get_item(Item::Third) == 0.0
    }
    fn sum(&self) -> f64 {
        self.get_item(Item::First) + self.get_item(Item::Second) + self.get_item(Item::Third)
    }
}
