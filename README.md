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

You can run my solutions with:
```
$ nix run . -- -r <runner> ./measurements.txt

# For additional information, run:
$ nix run . -- --help
```

## Results

Much like the official competition, results are taken by running each solution five times,
discarding the highest & lowest results.
This particular table of solutions was run on a system with an AMD Ryzen 7 2700X processor and
16GiB of memory.
Additionally, the 'delta' column represents the percentage change of a particular runner compared to the baseline.

| Runner                          | Runtime               | Delta  | Notes                                                                               |
| ------------------------------- | --------------------- | ------ | ----------------------------------------------------------------------------------- |
| [Baseline](./src/baseline.rs)   | 195s 179ms ± 6s 423ms | N/A    | Basic implementation; iterate through the file line-by-line                         |
| [Chunks](./src/chunks.rs)       |  69s 971ms ± 0s 094ms | -64.1% | Reduce the number of I/O operations by loading the file in large chunks into memory |
