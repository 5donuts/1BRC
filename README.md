# 1BRC

My take on the [1 Billion Row Challenge](https://1brc.dev/) in Rust.

## Usage

I've made the [`create_measurements.py`](https://github.com/gunnarmorling/1brc/blob/main/src/main/python/create_measurements.py)
script available as a flake output, so you can generate the test input with:
```
$ nix run .#create-measurements -- 1_000_000_000 # you can specify smaller inputs, too
```
Note: the minimum input size for the script is `10_000`.
For details, see the [`build_test_data` function](https://github.com/gunnarmorling/1brc/blob/main/src/main/python/create_measurements.py#L108)

You detailed instructions on how to run these solutions, see the help text with:
```
$ nix run . -- --help

# Or, if you don't have Nix:
$ cargo run -- --help
```

## Results

Much like the official competition, results are taken by running each solution five times,
discarding the highest & lowest results.
This particular table of solutions was run on a system with an AMD Ryzen 7 2700X processor and
16GiB of memory.
Additionally, the 'delta' column represents the percentage change of a particular runner compared
to the baseline.

| Runner                                  | Runtime               | Delta  | Notes                                                                               |
| --------------------------------------- | --------------------- | ------ | ----------------------------------------------------------- |
| [Baseline](./src/runners/baseline.rs)   | 178s 985ms ± 0s 041ms | N/A    | Basic implementation; iterate through the file line-by-line |

## TO-DOs

- [ ] Use [Bytehound](https://github.com/koute/bytehound) to do some memory profiling?
- [ ] Do some sort of CPU & disk usage profiling
- [ ] Get a solution that runs in < 60s
- [ ] Get a solution that runs in < 10s
