# Advent of Code 2024

My solutions to the [AOC 2024](https://adventofcode.com/2024) problems in [Rust](https://www.rust-lang.org/).

## Solutions

| Day  | Task 1 | Task 2  |
| ---: | :----: | :-----: | 
|  1 | :heavy_check_mark: | :heavy_check_mark: |  
|  2 | :heavy_check_mark: | :heavy_check_mark: |  
|  3 | :heavy_check_mark: | :heavy_check_mark: |  
|  4 | :heavy_check_mark: | :heavy_check_mark: |  
|  5 | :heavy_check_mark: | :heavy_check_mark: |  
|  6 | :heavy_check_mark: | :heavy_check_mark: |  
|  7 | :heavy_check_mark: | :heavy_check_mark: |  
|  8 | :heavy_check_mark: | :heavy_check_mark: |  
|  9 | :heavy_check_mark: | :heavy_check_mark: |  
| 10 | :heavy_check_mark: | :heavy_check_mark: |  
| 11 | :heavy_check_mark: | :heavy_check_mark: |  

***

## Organization

Each day's solutions are implemented in a separate module such as `day_01.rs`. This module usually contains the examples that explain the problem as unit tests.

For each day, there is an integration test, named for example `day_01.rs` in the `tests` subdirectory which makes sure that the functionality in the different modules produce the correct solutions when applied to the provided input files.

To run the tests for a specific day, run for example

```sh
> cargo t --release --test day_01
```

All the puzzle answers can be shown by running the code in `src/main.rs` through:

```sh
> cargo r --release
```

## Benchmarking

`Criterion` is used for benchmarking.

It can be installed by running the command:

```sh
> cargo install cargo-criterion
```

Then the benchmarks can be run using:

```sh
> cargo criterion
```
