# GraTeX

An TeX file converter to readable text file for grammer checkers.

## Updating test fixtures

The integration tests under `tests/` compare converter output against snapshots in `tests/fixtures/*.out`. After changing the converter, regenerate them with:

```bash
cargo run --bin update-fixtures
```

This runs the dedicated `update-fixtures` helper for each `.tex` fixture and overwrites the matching `.out` file with the new output.
