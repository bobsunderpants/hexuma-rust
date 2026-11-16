# Hexuma: The Eye of Kal — English remake

A playable English reconstruction of the 1992 German graphic text adventure
*Hexuma: Das Auge des Kal*, written in Rust. It uses the original game's
pictures, fonts and room-to-picture table, and rebuilds the world, parser and
puzzle chain in English.

The original was developed by **Weltenschmiede** and published by
**Software 2000** in 1992 for MS-DOS and Amiga. This is a fan preservation
project, not an official release.

## Status

The game is playable from beginning to end. All seven chapters are implemented,
including the four coloured worlds, the mine and volcano forge, and the final
confrontation with Kal. The scripted playthrough reaches **1595 of 1617 points**
and ends aboard the spaceship with five gem splinters and the Eye of Kal.

The original's maximum is 1617. The remaining 22 points are optional or
undocumented actions that the known walkthrough does not cover; the game keeps
the original maximum in its score display.

## Building

You need a Rust toolchain (2021 edition) and the usual Linux desktop
development libraries required by `minifb` (X11 or Wayland).

```sh
cargo build --release
```

The executable is `target/release/hexuma`.

## Running

Run from this directory so the `assets/` folder is found automatically:

```sh
cargo run --release
# or
./target/release/hexuma
```

To load assets from another directory, pass it as the only argument:

```sh
./target/release/hexuma /path/to/assets
```

For a terminal-only session (useful for scripted playtesting or a headless
machine):

```sh
./target/release/hexuma --text
```

Pictures are the original 320×200-era `.SEQ` files, scaled into the top part of
a 960×720 window; the scrolling text log occupies the lower part.

## Controls

- Type a command and press **Enter**.
- **Backspace** deletes a character.
- **Esc** or closing the window quits.

The parser is deliberately forgiving: short forms (`n`, `s`, `e`, `w`, `l`, `x`,
`i`) and a handful of German synonyms are accepted alongside English commands.

Common commands:

```text
LOOK / EXAMINE <thing>      TAKE <thing>          DROP <thing>
OPEN / CLOSE <thing>         READ <thing>          PUSH / PULL <thing>
TURN <thing>                 USE <thing>           ATTACH <a> TO <b>
DETACH <thing>               CUT <thing>           THROW <thing>
GIVE <thing> TO <person>     ASK <person>          SAY <word>
CLIMB / JUMP / SWIM / DIVE   LIGHT / EAT / WEAR    WAIT
INVENTORY / SCORE            JOURNAL / HINT        HELP
```

Movement: `NORTH`, `SOUTH`, `EAST`, `WEST`, `UP`, `DOWN` (and abbreviations).

`JOURNAL` collects the diary clues you find; `HINT` gives a chapter-aware nudge
without solving the puzzle outright.

## Assets

`assets/` contains 138 files taken from the original DOS release: room pictures
(`BILDnnn.SEQ`), the `BILDDAT.SEQ` room-to-picture map, the bitmap fonts, and
the menu/title artwork. Room numbers in the engine are the original room IDs, so
the picture lookup stays faithful to the game's own table.

All original artwork, names and game design remain the property of their
respective rights holders. They are included here for preservation and study.
If you own the rights and want them removed, please ask.

## Tests

The `tests/playthrough.rs` integration test replays the walkthrough scripts in
`tests/scripts/` and checks the score milestones and gem-splinter count for each
chapter:

```sh
cargo test
```

Expected checkpoints:

| Script | Reaches | Score | Splinters |
| --- | --- | ---: | ---: |
| `ch1.txt` | USA construction site (room 120) | 275 | 1 |
| `ch2.txt` | end of USA chapter | 500 | 2 |
| `ch3.txt` | end of ice world | 615 | 3 |
| `ch4.txt` | end of shipwreck chapter | 895 | 4 |
| `ch5.txt` | end of prehistory | 1020 | 5 |
| `ch6.txt` | Eye of Kal forged | 1245 | 5 |
| `ch7.txt` | spaceship (room 238) | 1595 | 5 |

## Implementation notes and limitations

- This is a reconstruction, not a byte-faithful port. The original story text is
  stored in compressed `TEXT*.REL` files; that format was not decoded, so the
  English room descriptions, dialogue and item prose were rewritten from the
  German walkthrough and the game's visible structure rather than translated
  line by line from the executable.
- Puzzles, map connections, item flow, scoring events and chapter gating follow
  the original room IDs and the walkthrough's documented sequence.
- Graphics are verified structurally (dimensions, palette, picture selection)
  rather than visually, because the development environment had no display.
- The window is fixed at 960×720. There is no save/load system; a playthrough is
  intended to be completed in one session, as in the original.

## Layout

```text
src/main.rs      window, picture/text rendering, input, --text mode
src/game.rs      game state, rooms, descriptions, movement, parser, journal
src/puzzles.rs   verbs, puzzle logic, world-specific mechanics
src/assets.rs    SEQ/BILDDAT loaders
tests/           scripted end-to-end regression tests
assets/          original pictures, fonts and menu artwork
```
