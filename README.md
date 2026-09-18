# bingo_prod

A command-line tool to generate bingo grids from a list of elements.

## Usage

Elements can be passed directly on the command line:

```sh
bingo apple banana cherry date elder fig grape honey kiwi lemon mango nectarine \
      olive papaya quince raspberry strawberry tangerine ugli vanilla watermelon \
      ximenia yuzu zucchini almond
```

...or read from a file, one element per line:

```sh
bingo --input elements.txt
```

### Options

| Flag | Description | Default |
| --- | --- | --- |
| `-i, --input <FILE>` | Read elements from a file instead of the positional arguments | - |
| `-s, --size <SIZE>` | Grid size (produces an `N x N` grid) | `5` |
| `-c, --count <COUNT>` | Number of bingo cards to generate | `1` |
| `-f, --free-space` | Insert a free space in the center (requires an odd size) | off |
| `--free-space-text <TEXT>` | Text shown in the free space | `FREE` |
| `--seed <SEED>` | Seed the RNG for reproducible output | random |
| `-F, --format <FORMAT>` | Output format: `text`, `csv`, or `markdown` | `text` |
| `-o, --output <FILE>` | Write output to a file instead of stdout | stdout |

A grid needs `size * size` elements (or `size * size - 1` with `--free-space`);
the tool errors out if not enough elements are given.

## Examples

Generate a classic 5x5 card with a free center space, reproducibly:

```sh
bingo --input elements.txt --free-space --seed 42
```

Generate 4 printable cards as Markdown tables:

```sh
bingo --input elements.txt --count 4 --format markdown --output cards.md
```

## Development

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
```
