use num::{Bounded, Integer, One, Saturating};
use std::{mem, ops::RangeInclusive};

#[derive(Debug, Clone)]
pub struct RangeTree<T> {
    range: RangeInclusive<T>,
    less: Option<Box<RangeTree<T>>>,
    more: Option<Box<RangeTree<T>>>,
}

impl<A: Copy + Clone + Integer + Saturating + One + Bounded> FromIterator<A> for RangeTree<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut input = iter.into_iter();
        if let Some(first) = input.next() {
            let mut tree = RangeTree::new(first);
            for value in input {
                tree.insert(value);
            }
            tree
        } else {
            RangeTree::<A>::new(A::zero())
        }
    }
}

impl<T: Copy + Clone + Integer + Saturating + One + Bounded> RangeTree<T> {
    pub fn new(value: T) -> Self {
        RangeTree {
            range: value..=value,
            less: None,
            more: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.range.contains(&value) {
            return;
        }
        if self.range.start() > &value {
            if self.less.is_none() {
                self.less = Some(Box::new(RangeTree::new(value)));
            }
            if let Some(less) = &mut self.less {
                less.insert(value);
                if less.range.end().saturating_add(T::one()) == *self.range.start() {
                    self.range = *less.range.start()..=*self.range.end();
                    self.less = mem::take(&mut less.less);
                }
            }
        } else if self.range.end() < &value {
            if self.more.is_none() {
                self.more = Some(Box::new(RangeTree::new(value)));
            }
            if let Some(more) = &mut self.more {
                more.insert(value);
                if self.range.end().saturating_add(T::one()) == *more.range.start() {
                    self.range = *self.range.start()..=*more.range.end();
                    self.more = mem::take(&mut more.more);
                }
            }
        }
    }

    pub fn to_vec(&self) -> Vec<RangeInclusive<T>> {
        let mut vec = Vec::<RangeInclusive<T>>::new();
        self.to_vec_req(&mut vec);
        vec
    }

    fn to_vec_req(&self, vec: &mut Vec<RangeInclusive<T>>) {
        if let Some(less) = &self.less {
            less.to_vec_req(vec);
        }
        if let Some(last) = vec.last_mut() {
            if last.end().saturating_add(T::one()) == *self.range.start() {
                *last = *last.start()..=*self.range.end();
            } else {
                vec.push(self.range.clone());
            }
        } else {
            vec.push(self.range.clone());
        }

        if let Some(more) = &self.more {
            more.to_vec_req(vec);
        }
    }
}

/// Takes a custom sequence of integers and produces a sorted vector of ranges.
/// # Examples
/// ```
/// use range_rover::range_rover;
/// let input = vec![2, 0, 7, 10, 4, 1, 3, 6, 4, 0, 5, 9, 8];
/// let result = range_rover(input);
/// assert_eq!(result, vec![0..=10]);
/// ```
///
/// ```
/// use range_rover::range_rover;
/// let input = vec![-1, -2, 2, 0, 7, 10, -4, 1, 3, 6, -3, 10, 4, 9, 8, -2];
/// let result = range_rover(input);
/// assert_eq!(result, vec![-4..=4, 6..=10]);
/// ```
pub fn range_rover<I, T>(input: I) -> Vec<RangeInclusive<T>>
where
    I: IntoIterator<Item = T>,
    T: Copy + Clone + Integer + Saturating + One + Bounded,
{
    let mut ranges = vec![];
    let mut input = input.into_iter();
    if let Some(first) = input.next() {
        let mut tree = RangeTree::new(first);
        for value in input {
            tree.insert(value);
        }
        ranges = tree.to_vec();
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn max_value() {
        let input = (u16::MIN..=u16::MAX).rev();
        let result = range_rover(input);
        assert_eq!(result, vec![(u16::MIN)..=u16::MAX]);
    }

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        let mut random_numbers = [0_usize; 1024 * 4];
        rng.fill(&mut random_numbers);

        let mut input_sorted = random_numbers.to_vec();
        input_sorted.sort();

        let ranges = range_rover(random_numbers);
        let mut output: Vec<usize> = ranges
            .into_iter()
            .flat_map(|r| r.collect::<Vec<usize>>())
            .collect();
        output.sort();

        assert_eq!(input_sorted, output);
    }
}
