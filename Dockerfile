# Dōbutsu Shōgi tablebase explorer — single always-on container for Railway.
# serve.py (Python stdlib HTTP) fronts ctbprobe (Rust), which holds the compact
# tablebase in memory (~400 MB resident) and answers each /api query over a pipe.
#
# The 333 MB dobutsu.ctb is git-ignored, so it is NOT in the build context. It
# is fetched at build time from a public GitHub release asset (override the URL
# with --build-arg CTB_URL=... when the tablebase is re-cut).

# ---- build the probe ----
FROM rust:1-slim AS build
WORKDIR /src
COPY solver/Cargo.toml solver/Cargo.lock ./solver/
COPY solver/src ./solver/src
RUN cd solver && cargo build --release --bin ctbprobe

# ---- runtime ----
FROM python:3.12-slim
WORKDIR /app
COPY --from=build /src/solver/target/release/ctbprobe /app/solver/target/release/ctbprobe
COPY explorer /app/explorer
COPY assets/diagrams/emoji /app/assets/diagrams/emoji

ARG CTB_URL=https://github.com/brianhliou/dobutsu-shogi/releases/download/tb-v1/dobutsu.ctb
ADD ${CTB_URL} /app/solver/dobutsu.ctb

ENV PORT=8080
EXPOSE 8080
CMD ["python3", "/app/explorer/serve.py"]
