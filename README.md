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

The first part wants us to compare each pair of subsequent numbers in a slice. This kind of sliding view is called a _window_, and for slices in Rust a convenient method called exactly that exists. We simply get a sliding window of size 2, and count the ones where the second element is larger than the first.

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

And finally, a handy comparison table for part 1.

| function                      | runtime (ns) |
| ----------------------------- | ------------ |
| iterators                     | 474.70 ns    |
| loop-based                    | 845.12 ns    |
| reverse comparison loop-based | 473.28 ns    |

### Part 2

As with most Advent of Code second-parts, the solution to day 1 part 2 is annoyingly close to part 1, but also has a devious twist. It's a test of your refactoring skills or your ability to write a general solution ready for changing conditions. The second part to day 1 wants you to figure out whether the sum of adjacent windows of size 3 are increasing or decreasing, and to count the number of increases. That is, in the list `[a, b, c, d, e]` we should check whether `a + b + c < b + c + d`, `b + c + d < c + d + e`, and so forth.

I liked the sliding window solution of part 1, and as we saw it was nicely performant too. I'd like to do something like that for part 2. First we'll get the sum of each window. That way we get a list of sums for which we can use the same approach as part 1...

```rust
input
    .windows(3)
    .map(|window| window.iter().sum::<usize>())
```

.. or not. It turns out only slices can use `.windows(usize)`. The standard library cannot do this for iterators because it requires an allocating iterator. These aren't implemented anywhere in the standard library. After a quick google I see that `itertools` has a `.tuple_windows(usize)` function that does something similar. A quick `cargo add -s itertools` command and we're off to the races!

```rust
pub fn iterators(input: &[usize]) -> usize {
    input
        .windows(3)
        .map(|window| window.iter().sum::<usize>())
        .tuple_windows()
        .map(|(prev, next)| if prev < next { 1 } else { 0 })
        .sum()
}
```

This works perfectly! Right answer input into the website and we can take the rest of the day off, right? Not quite. While the above solution works, pulling `itertools` in for that single function seems like an awful waste. I'll write an implementation that uses only functionality from `std` instead.

```rust
pub fn std_alloc(input: &[usize]) -> usize {
    let mut window_sums = vec![];
    for i in 0..input.len() - 2 {
        window_sums.push(input[i] + input[i + 1] + input[i + 2]);
    }

    part1::iterators(&window_sums) // use fastest solution from part 1
}
```

The first solution to include an allocation. The `.tuple_windows(usize)` implementation uses the `Clone` trait and statically sized arrays located on the stack, but doesn't allocate any heap memory for itself. Despite this, the allocating `std`-only implementation is faster. By just under 20%. To me this means one thing -- I'm missing something. They have the same number of additions performed, counted naively, so the performance difference is not down to the solution approach in itself. Instead, it must be that while `.tuple_windows(usize)` doesn't allocate it's machinery is heavy enough that an equivalent allocation is slightly faster. Further, we get to use the super fast counting implementation from part 1 directly as we have a `Vec` (and thus a slice), instead of a slightly less ergonomic iterator.

Are we done now? We found a faster implementation using nothing but `std`. But no, there's more. As I wrote earlier, we want to count all of the times `a + b + c < b + c + d` is true. This is math. Can we simplify it? Yes! `b + c` is components of sums either side of the _greater than_ symbol. We can remove those. Then we find out that we just have to check `a < d`. In programming terms, it means we have to check `input[i] < input[i + 3]`. That is so much easier, and doesn't require any pesky sums. First stab at it, using iterators:

```rust
pub fn cmp_iterators(input: &[usize]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(3))
        .map(|(prev, next)| if prev < next { 1 } else { 0 })
        .sum()
}
```

Now we're talking! A single run through the list, using the same approach as the fastest solution to part 1. How much faster is it? About 70% faster compared to the two original approaches. Just for fun, what does a loop-based version of this look like?

```rust
pub fn cmp_for(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 3 {
        if input[i] < input[i + 3] {
            count += 1;
        }
    }

    count
}
```

And the performance is... 40% faster than the iterator-based approach. My intuition tells me it's because `.zip()` is harder for the compiler to reason about than two bound checks, and the iterator-based approach does have an extra comparison and conditional jump in the hot loop that would back that up. The astute reader will might then wonder if the trick of reversing the expressions in the `if` condition would lead to a speed-up like it did it part 1.

```rust
pub fn cmp_for_rev(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 3 {
        if input[i + 3] > input[i] {
            count += 1;
        }
    }

    count
}
```

It does. That's another gain of around 40%. It would seem every conditional jump, whether it be bounds or the output of zipped iterators, slows the runtime by about 40%. We can test this by using some unsafe code that we know is actually safe.

```rust
pub fn cmp_for_rev_unsafe(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 3 {
        unsafe {
            if input.get_unchecked(i + 3) > input.get_unchecked(i) {
                count += 1;
            }
        }
    }

    count
}
```

Unfortunately, while this does produce assembly that looks close to that of the iterator-based approach from part 1, i.e. doing 4 comparisons per loop, it consistently performs slower than `cmp_for_rev` but faster than `cmp_for`.

And finally, here's the full comparison of all the implementations.

| function           | runtime (us) |
| ------------------ | ------------ |
| iterators          | 5.8372 us    |
| std_alloc          | 4.5266 us    |
| cmp_iterators      | 1.9049 us    |
| cmp_for            | 0.9506 us    |
| cmp_for_rev        | 0.5240 us    |
| cmp_for_rev_unsafe | 0.7064 us    |
