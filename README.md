# GraTeX

An TeX file converter to readable text file for grammer checkers.

## Grammarly integration

Pass `--grammarly` to `gratex` to launch the bundled `open-chrome-to-edit` helper so the converted text opens in Chrome with Grammarly enabled. By default the helper writes to a temporary file, but you can provide a fixed location via `--grammarly-path <file>`. The script is executed through `uv run` when available and falls back to `python3`/`python`. To use a custom helper path set `GRATEX_OPEN_CHROME_SCRIPT=/path/to/open-chrome-to-edit`.

## Updating test fixtures

The integration tests under `tests/` compare converter output against snapshots in `tests/fixtures/*.out`. After changing the converter, regenerate them with:

```bash
cargo run --bin update-fixtures
```

This runs the dedicated `update-fixtures` helper for each `.tex` fixture and overwrites the matching `.out` file with the new output.
