pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

pub struct Tuple(u32, f32, f64);

impl ThreeValues for Tuple {
    fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }

    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }
}

pub struct Array([f64; 3]);

impl ThreeValues for Array {
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

pub trait ThreeValues {
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

#[cfg(test)]
mod tests {
    use crate::{Array, Item, ThreeValues, Tuple};

    #[test]
    fn test_sum() {
        assert_eq!(3.0, set_values_and_calc_sum(Array::default_values()));
        assert_eq!(3.0, set_values_and_calc_sum(Tuple::default_values()));
    }

    #[test]
    fn test_default_values() {
        assert!(is_default(Array::default_values()));
        assert!(is_default(Tuple::default_values()));
    }

    #[test]
    fn test_set() {
        assert!(check_set(Array::default_values()));
        assert!(check_set(Tuple::default_values()));
    }

    #[test]
    fn test_get() {
        assert!(all_values_zero(Array::default_values()));
        assert!(all_values_zero(Tuple::default_values()));
    }

    fn all_values_zero(x: impl ThreeValues) -> bool {
        let zero = 0.0;
        zero == x.get_item(Item::First)
            && zero == x.get_item(Item::Second)
            && zero == x.get_item(Item::Third)
    }

    fn check_set(mut x: impl ThreeValues) -> bool {
        let one = 1.0;
        x.set_item(Item::First, one);
        x.set_item(Item::Second, one);
        x.set_item(Item::Third, one);

        one == x.get_item(Item::First)
            && one == x.get_item(Item::Second)
            && one == x.get_item(Item::Third)
    }

    fn is_default(x: impl ThreeValues) -> bool {
        x.is_default()
    }

    fn set_values_and_calc_sum(mut x: impl ThreeValues) -> f64 {
        x.set_item(Item::First, 1_f64);
        x.set_item(Item::Second, 1_f64);
        x.set_item(Item::Third, 1_f64);
        x.sum()
    }
}
