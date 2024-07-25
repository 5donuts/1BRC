# 1BRC

My take on the [1 Billion Row Challenge](https://1brc.dev/) in Rust.

## Usage

I've made the [`create_measurements.py`](https://github.com/gunnarmorling/1brc/blob/main/src/main/python/create_measurements.py)
script available as a flake output, so you can generate the test input with:
```
$ nix run .#create-measurements -- 1_000_000_000 # you can specify smaller inputs, too
```

You can run my solutions with:
```
$ nix run . -- -r runner-name ./measurements.txt

# For additional information, run:
$ nix run . -- --help
```
