# explorer/ — Dōbutsu Shōgi tablebase explorer (prototype)

A board + a lichess-style move panel: for the current position, every legal move
with its result under perfect play (Win/Loss/Draw + distance in plies). Click a
move to follow it and explore the whole solved game.

## Architecture

```
index.html   self-contained page: SVG board (our own tiles + Twemoji glyphs) + move panel
serve.py     local server: holds one probe process, serves /api, /emoji, and the page
probe.c      (tools/probe.c) stdin/stdout JSON probe over the tablebase
```

The data backend is currently **clausecker/dobutsu** (BSD), via the probe. It is a
**drop-in seam**: once our own Rust tablebase compiles to WASM, the page can query
that instead of the local server, and the explorer becomes static/hostable.

## Run it

```sh
# 1. build the probe inside the clausecker checkout (needs its objects + the tablebase)
cd external/clausecker-dobutsu
cp ../../tools/probe.c .
export PKG_CONFIG_PATH="$(brew --prefix xz)/lib/pkgconfig:$PKG_CONFIG_PATH"
c99 $(pkg-config --cflags liblzma) -O2 -c probe.c -o probe.o
c99 -o probe probe.o tbaccess.o poscode.o moves.o position.o notation.o validation.o unmoves.o \
    $(pkg-config --libs-only-l liblzma) $(pkg-config --libs-only-L --libs-only-other liblzma)
cd ../..

# 2. start the server
python3 explorer/serve.py        # -> http://localhost:41234/
```

(The clausecker checkout and the generated `dobutsu.tb` live in `external/`, which is
git-ignored; rebuild them per `research/reproduction.md`.)

## API

`GET /api?pos=<position-string>` returns:

```json
{"pos":"S/gle/-c-/-C-/ELG/-","side":"S","value":{"result":"loss","dtm":78},
 "moves":[{"move":"Cb3xb2","result":"loss","dtm":76,"to":"G/gle/-C-/---/ELG/C"}, ...]}
```

Position strings are clausecker's format: `side / rank1 / rank2 / rank3 / rank4 / hand`.
result/dtm are from the side-to-move's view; dtm is in plies; a move's `to` is empty when
the move ends the game (lion capture / Try).
