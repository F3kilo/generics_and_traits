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

pub trait Has3Items {
    fn default_values() -> Self;
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);
    fn is_default(&self) -> bool
    where
        Self: Sized,
    {
        let default_values = Self::default_values();
        default_values.get_item(Item::First) == self.get_item(Item::First)
            && default_values.get_item(Item::Second) == self.get_item(Item::Second)
            && default_values.get_item(Item::Third) == self.get_item(Item::Third)
    }
    fn sum(&self) -> f64 {
        self.get_item(Item::First) + self.get_item(Item::Second) + self.get_item(Item::Third)
    }
}

#[derive(PartialEq, Debug)]
pub struct Tuple(u32, f32, f64);

impl Has3Items for Tuple {
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

#[derive(PartialEq, Debug)]
pub struct Array([f64; 3]);

impl Has3Items for Array {
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

pub fn is_default<T: Has3Items>(t: &T) -> bool {
    t.is_default()
}

pub fn sum<T: Has3Items>(t: &T) -> f64 {
    t.sum()
}

pub fn get_item<T: Has3Items>(t: &T, i: Item) -> f64 {
    t.get_item(i)
}

pub fn set_item<T: Has3Items>(t: &mut T, i: Item, v: f64) {
    t.set_item(i, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_default_works() {
        let t1 = Tuple::default_values();
        let t2 = Tuple(1, 2.0, 3.0);

        assert_eq!(true, is_default(&t1));
        assert_eq!(false, is_default(&t2));

        let a1 = Array::default_values();
        let a2 = Array([1.0, 2.0, 3.0]);

        assert_eq!(true, is_default(&a1));
        assert_eq!(false, is_default(&a2));
    }

    #[test]
    fn sum_works() {
        let t = Tuple(1, 2.0, 3.0);
        assert_eq!(6.0, sum(&t));

        let a = Array([10.0, 20.0, 30.0]);
        assert_eq!(60.0, sum(&a));
    }

    #[test]
    fn get_item_works() {
        let t = Tuple(1, 2.0, 3.0);
        assert_eq!(1.0, get_item(&t, Item::First));
        assert_eq!(2.0, get_item(&t, Item::Second));
        assert_eq!(3.0, get_item(&t, Item::Third));

        let a = Array([1.0, 2.0, 3.0]);
        assert_eq!(1.0, get_item(&a, Item::First));
        assert_eq!(2.0, get_item(&a, Item::Second));
        assert_eq!(3.0, get_item(&a, Item::Third));
    }

    #[test]
    fn set_item_works() {
        let mut t = Tuple(1, 2.0, 3.0);
        set_item(&mut t, Item::First, 10.0);
        set_item(&mut t, Item::Second, 20.0);
        set_item(&mut t, Item::Third, 30.0);

        assert_eq!(Tuple(10, 20.0, 30.0), t);

        let mut a = Array([1.0, 2.0, 3.0]);
        set_item(&mut a, Item::First, 10.0);
        set_item(&mut a, Item::Second, 20.0);
        set_item(&mut a, Item::Third, 30.0);

        assert_eq!(Array([10.0, 20.0, 30.0]), a);
    }
}
