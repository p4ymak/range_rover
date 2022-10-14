use std::collections::BTreeMap;
use std::ops::RangeInclusive;

/// Takes a random sequence of non-repeating values and produces a sorted vector of ranges.
pub fn range_rover<T: IntoIterator<Item = u32>>(input: T) -> Vec<RangeInclusive<u32>> {
    let mut map = BTreeMap::<u32, u32>::new(); // key: end, value: start
    for i in input {
        let e = i.saturating_add(1);
        if let Some(s) = map.remove(&i) {
            map.insert(e, s);
        } else {
            map.insert(e, i);
        }
    }
    let mut ranges = Vec::<RangeInclusive<u32>>::new();
    let mut map_iter = map.into_iter();
    let mut add_range = |from, to| ranges.push(from..=(to - (to != u32::MAX) as u32));
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

    #[test]
    fn zero_to_ten() {
        let input = vec![2, 0, 7, 10, 1, 3, 6, 4, 5, 9, 8];
        let result = range_rover(input);
        assert_eq!(result, vec![0..=10]);
    }
    #[test]
    fn zero_to_ten_splitted() {
        let input = vec![2, 0, 7, 10, 1, 3, 6, 4, 9, 8];
        let result = range_rover(input);
        assert_eq!(result, vec![0..=4, 6..=10]);
    }
    #[test]
    fn max_value() {
        let input = (u32::MAX - 10)..=u32::MAX;
        let result = range_rover(input);
        assert_eq!(result, vec![(u32::MAX - 10)..=u32::MAX]);
    }
}
