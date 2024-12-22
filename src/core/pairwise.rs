use crate::core::Pair::*;

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum Pair<T> {
    First(T, T),
    Middle(T, T),
    Last(T, T)
}

pub struct PairwiseResults<'a, T, I> {
    prev: Option<&'a T>,
    current: Option<&'a T>,
    iterator: I
}

impl<'a, T, I: Iterator<Item = &'a T>> Iterator for PairwiseResults<'a, T, I> {
    type Item = Pair<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        // on first iteration call the iterator to populate prev and current
        let is_first = self.prev.is_none();
        let prev = match self.prev.take() {
            Some(prev) => prev,
            None => self.iterator.next()?
        };
        let current = match self.current.take() {
            Some(current) => current,
            None => self.iterator.next()?
        };

        match self.iterator.next() {
            None => Some(Last(prev, current)),
            Some(next) => {
                self.prev = Some(current);
                self.current = Some(next);
                match is_first {
                    true => Some(First(prev, current)),
                    false => Some(Middle(prev, current))
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
            current: None,
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
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_two_elements() {
        let input = vec![1, 2];
        let result = call_pairwise(&input);
        assert_eq!(result, vec![Last(&1, &2)]);
    }

    #[test]
    fn test_three_elements() {
        let input = vec![1, 2, 3];
        let result = call_pairwise(&input);
        assert_eq!(result, vec![First(&1, &2), Last(&2, &3)]);
    }

    #[test]
    fn test_four_elements() {
        let input = vec![1, 2, 3, 4];
        let result = call_pairwise(&input);
        assert_eq!(result, vec![First(&1, &2), Middle(&2, &3), Last(&3, &4)]);
    }

    #[test]
    fn test_five_elements() {
        let input = vec![1, 2, 3, 4, 5];
        let result = call_pairwise(&input);
        assert_eq!(result, vec![First(&1, &2), Middle(&2, &3), Middle(&3, &4), Last(&4, &5)]);
    }
}
