use num::{Bounded, Integer, One, Saturating};
use std::{collections::BTreeMap, ops::RangeInclusive};

/// Takes a custom sequence of integers and produces a sorted vector of ranges.
/// # Examples
/// ```
/// use range_rover::range_rover;
/// let input = vec![2, 0, 7, 10, 1, 3, 6, 4, 5, 9, 8];
/// let result = range_rover(input);
/// assert_eq!(result, vec![0..=10]);
/// ```
///
/// ```
/// use range_rover::range_rover;
/// let input = vec![-1, -2, 2, 0, 7, 10, -4, 1, 3, 6, -3, 4, 9, 8];
/// let result = range_rover(input);
/// assert_eq!(result, vec![-4..=4, 6..=10]);
/// ```
pub fn range_rover<I, T>(input: I) -> Vec<RangeInclusive<T>>
where
    I: IntoIterator<Item = T>,
    T: Copy + Clone + Integer + Saturating + One + Bounded,
{
    let mut map = BTreeMap::<T, T>::new(); // key: end, value: start
    for i in input {
        let e = i.saturating_add(T::one());
        if let Some(s) = map.remove(&i) {
            map.insert(e, s);
        } else {
            map.insert(e, i);
        }
    }
    let mut ranges = Vec::<RangeInclusive<T>>::new();
    let mut map_iter = map.into_iter();
    let mut add_range = |from, to| {
        ranges.push(
            from..=(to
                - match to == T::max_value() {
                    true => T::zero(),
                    false => T::one(),
                }),
        )
    };
    if let Some((mut to, mut from)) = map_iter.next() {
        for r in map_iter {
            if to == r.1 {
                to = r.0;
            } else {
                add_range(from, to);
                from = r.1;
                to = r.0;
            }
        }
        add_range(from, to);
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn max_value() {
        let input = (u32::MAX - 10)..=u32::MAX;
        let result = range_rover(input);
        assert_eq!(result, vec![(u32::MAX - 10)..=u32::MAX]);
    }

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        let mut random_numbers = [0_usize; 1024];
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
