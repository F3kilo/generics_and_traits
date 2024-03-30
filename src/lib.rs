#[warn(dead_code)]
mod array;
mod item;
mod triple_work;
mod tuple;

#[cfg(test)]
mod tests {
    use crate::{triple_work::TripleWorks, tuple::Tuple};

    #[test]
    fn it_works() {
        let a: Tuple = Tuple::default_values();
        assert_eq!(a.is_default(), true);
    }
}
