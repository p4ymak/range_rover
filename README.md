## Function to pack integers into ranges.

Takes a custom sequence of integers and produces a sorted vector of ranges.

### Example:
```rust
use range_rover::range_rover;

let input = vec![-1, -2, 2, 0, 7, 10, -4, 1, 3, 6, 10, -3, 4, 9, 8];
let result = range_rover(input);
assert_eq!(result, vec![-4..=4, 6..=10]);
```
