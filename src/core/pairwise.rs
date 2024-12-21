use crate::core::Pair::*;

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum Pair<T> {
    First(T),
    Middle(T, T),
    Last(T)
}

pub struct PairwiseResults<'a, T, I> {
    prev: Option<&'a T>,
    iterator: I
}

impl<'a, T, I: Iterator<Item = &'a T>> Iterator for PairwiseResults<'a, T, I> {
    type Item = Pair<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            None => {
                let result = self.prev.map(Last);
                self.prev = None;
                result
            },
            Some(next) => {
                let temp = self.prev;
                self.prev = Some(next);
                match temp {
                    None => Some(First(next)),
                    Some(prev) => Some(Middle(prev, next))
                }
            }
        }
    }
}

pub trait Pairwise {
    type Item;

    fn pairwise(self) -> impl Iterator<Item = Pair<Self::Item>>;
}

impl<'a, T: 'a, I: Iterator<Item = &'a T>> Pairwise for I {
    type Item = &'a T;

    fn pairwise(self) -> impl Iterator<Item = Pair<Self::Item>> {
        PairwiseResults {
            prev: None,
            iterator: self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_pairwise(vec: &Vec<i32>) -> Vec<Pair<&i32>> {
        vec.iter().pairwise().take(10).collect()
    }

    #[test]
    fn test_empty() {
        let input = vec![];
        let result = call_pairwise(&input);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_singleton() {
        let input = vec![1];
        let result = call_pairwise(&input);
        assert_eq!(result, vec![First(&1), Last(&1)]);
    }

    #[test]
    fn test_two_elements() {
        let input = vec![1, 2];
        let result = call_pairwise(&input);
        assert_eq!(result, vec![First(&1), Middle(&1, &2), Last(&2)]);
    }

    #[test]
    fn test_three_elements() {
        let input = vec![1, 2, 3];
        let result = call_pairwise(&input);
        assert_eq!(result, vec![First(&1), Middle(&1, &2), Middle(&2, &3), Last(&3)]);
    }
}
