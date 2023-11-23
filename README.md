# Branch Prediction Simulation

This program is meant to measure the performance of 4 branch prediction techniques:

- Always taken
- Always not taken
- [1-bit history](https://wikiless.tiekoetter.com/wiki/Branch_predictor?lang=en#One-level_branch_prediction)
- [2-bit history](https://wikiless.tiekoetter.com/wiki/Branch_predictor?lang=en#Two-level_predictor)

where the "history" techniques are in the first "not taken" state by default.

Each possible outcome of branch prediction is determined by a value:

- not taken ∈ `[0, 0.5)`
- taken ∈ `[0.5, 1)`

This simulation populates a vector with 10,000 values in these ranges and measures the performance of the 4 prediction techniques simulated in this program using these values.

Performance is measured as a percentage of correct predictions for each of the 4 techniques.

## Usage

``` console
$ cargo run
...
```

output will print to the screen. To print to an output file:

``` console
$ cargo run > [OUTPUT_FILENAME]
...
```

## prerequisites

- [rust](https://www.rust-lang.org/tools/install)

## Libraries used

- [`indoc`](https://github.com/dtolnay/indoc) to format the output
- [`rand`](https://rust-random.github.io/book/) for random number (outcome value) generation
