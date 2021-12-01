# Running

- Make sure you have Rust installed via [rust-lang.org](https://www.rust-lang.org/learn/get-started).

- Install the `cargo-aoc` runner via `cargo install cargo-aoc`.

  - This allows you to run `cargo aoc`, which will automatically download and cache the input

- Run the latest implemented day with `cargo aoc`, or run a specific day with `cargo aoc -d <day>`. Automatic benchmarking can be done with `cargo aoc bench -d <day>`, or `cargo aoc bench` for the latest implemented day.

# Disclaimer

Using the benchmarks associated with this repository is a fool's errand. They're uncontrolled microbenchmarks intended to squeeze performance out of my personal computer with a specific, small dataset for each function. They cannot be used for anything else.

# Observations

## Day 1 - Sonar Sweep

As usual Advent of Code starts off with an easy challenge. But that's not to say there aren't performance pitfalls abound.

### Parsing input

Parsing the input for this day's challenge was, luckily, trivial.

```rust
pub fn parse(input: &str) -> Vec<usize> {
    input.lines().flat_map(|s| s.parse::<usize>()).collect()
}
```

Using `.flat_map` with a closure that returns a `Result<usize, _>` silently ignores the parsing errors that might be encountered. Since the AOC input is well-formed and known this doesn't bother me much. I return an allocated vector of pointer-sized unsized integers, and each solution borrows the vector as a slice.

### Part 1

The first part wants us to compare each pair of subsequent numbers from the sonar sweep. This kind of sliding view is called a _window_, and for slices in Rust a convenient method called exactly that exists. We simply get a sliding window of size 2, and count the ones where the second element is larger than the first.

```rust
pub fn iterators(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}
```

This implementation gets us to **_470 nanoseconds_** to find the solution for part 1. The input has exactly 2000 entries. I tried testing whether a simple for-loop could be faster since it logically would do the same comparisons and computations.

```rust
pub fn loop_based(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 1 {
        if input[i] < input[i + 1] {
            count += 1;
        }
    }
    count
}
```

This, counter-intuitively, is about 75% slower than the iterator-based solution. Looking at the generated assembly code for each function it seems the iterator-based solution does 4 pairs per iteration, and then picks up the difference in a small loop run at most 3 times. The reason for this optimization being in one implementation while missing in another is simple. The optimization requires that bounds checks can be elided, and in this case the Rust compiler doesn't think they can. Further, for `input[i] < input[i + 1]` it'll first check `i < input.len()` and then `i + 1 < input.len()`. Re-arranging the check to `input[i + 1] > input[i]` helps the compiler realize that only `i + 1 < input.len()` really needs checking. The full, updated, function looks as follows.

```rust
pub fn reverse_loop_based(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 1 {
        if input[i + 1] > input[i] {
            count += 1;
        }
    }
    count
}
```

This gives a speedup of over 40% and brings the loop-based implementation completely in line with the iterator-based implementation.

| function                      | runtime (ns) |
| ----------------------------- | ------------ |
| iterators                     | 474.70 ns    |
| loop-based                    | 845.12 ns    |
| reverse comparison loop-based | 473.28 ns    |
