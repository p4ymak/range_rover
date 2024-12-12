## Function to pack integers into ranges.

[![Latest version](https://img.shields.io/crates/v/range_rover.svg)](https://crates.io/crates/range_rover)

Takes a custom sequence of integers and produces a sorted vector of ranges.

### Example:
```rust
use range_rover::range_rover;

let input = vec![-1, -2, 2, 0, 7, 10, -4, 1, 3, 6, 10, -3, 4, 9, 8];
let result = range_rover(input);
assert_eq!(result, vec![-4..=4, 6..=10]);
```

Takes a custom sequence of integers and range, produces a sorted vector of excluded ranges in range.

### Example:
```rust
use range_rover::missed_in_range;
let input = vec![-1, -2, 2, 0, 7, 10, -4, 1, 3, 6, -3, 10, 4, 9, 8, -2];
let missed = missed_in_range(input, -10..=20);
assert_eq!(missed, vec![-10..=-5, 5..=5, 11..=20]);
```
