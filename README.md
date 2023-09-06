# Legend of Legaia Arts Scanner

The Legend of Legaia Arts scanner is a utility for the masochists out there who,
for whatever reason don't want to look up the Miracle Art combos and instead
discover them themselves.

## Usage

```text
Miracle Art possibility scanner

Usage: arts-scanner [OPTIONS]

Options:
  -a, --arts <ARTS>
          File path to arts file

          The file should have one art per line, include no whitespace, and use the following characters: <: Left >: Right ^: High v: Low

  -m, --max-consecutive-repeats <MAX_CONSECUTIVE_REPEATS>
          The maximum number of consecutive identical moves

          [default: 9]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

The `--arts` option specifies a text file with known art combinations, and any
combos that include these arts should be discarded. There should be one art per
line, and no characters other than the move symbols. Only a single character's
arts should be in a file to enusure potentailly valid combos aren't removed.

The below example shows what a file with two known arts:

```text
^v^
v^^^
```

The `--max-consecutive-repeats` option specifies how many times a given move can
appear consecutively. If not specified, it defaults to 9, which allows
everything.
