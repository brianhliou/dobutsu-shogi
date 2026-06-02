# research/repro/ — reproduction artifacts

Self-contained pieces needed to rebuild the clausecker/dobutsu tablebase on macOS, so the
solve can be regenerated without committing third-party code or the ~168 MB tablebase.

- `pthread_barrier_shim.h` — drop-in POSIX-barrier implementation for macOS (Apple libc omits
  the optional barrier API). Copy into the clone and `#include` it in `tbgenerate.c`.

Full step-by-step (clone SHA, build flags, generate, validate, query) is in
[`../reproduction.md`](../reproduction.md). The clone itself lives in `external/` (git-ignored).
