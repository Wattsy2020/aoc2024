use std::collections::HashMap;
use std::hash::Hash;

pub trait Countable {
    type Item;
    fn counts(&mut self) -> HashMap<Self::Item, i32>;
}

impl<T: Eq + Hash, I: Iterator<Item = T>> Countable for I {
    type Item = T;

    fn counts(&mut self) -> HashMap<Self::Item, i32> {
        let mut counts = HashMap::new();
        for item in self {
            *counts.entry(item).or_insert(0) += 1;
        }
        counts
    }
}
