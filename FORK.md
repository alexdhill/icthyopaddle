# reaprs fork of oarfish

Base: `oarfish` 0.11.0 from crates.io (COMBINE-lab/oarfish, BSD-3, see `LICENSE`).
Vendored 2026-11-09 for expansion-plan P4. Not yet a git remote; to match
`crates/pareaplenty`, push it to a fork and replace this directory with a submodule.

## Changes against 0.11.0

1. `src/main.rs` split. Everything after argument parsing is now
   `oarfish::quantify(args, set_filter)` in `src/lib.rs`; `src/main.rs` is the
   tracing setup plus one call. Behaviour is unchanged otherwise.
   - the library installs no tracing subscriber and no global allocator;
   - `--quiet`, `--verbose` and the single-cell log levels go through the
     `set_filter` hook (a closure that ignores them is fine);
   - an already-built global rayon pool is kept with a warning, not an error: the
     caller owns the pool size, so `--threads` then bounds only oarfish's own pools;
   - a failing `set_filter` hook is warned about, never fatal;
   - `oarfish::parse_args` parses a command line without exiting the process;
   - `prog_opts` is `pub` (the `Args` struct).
2. `src/util/write_function.rs`: the `meta_info.json` start time is recorded per
   call (a `Mutex`), not once per process (upstream's `OnceLock`), so repeated
   in-process runs report their own start.
3. `Cargo.toml`: `[lib]` target added.
4. `Cargo.toml`: `niffler` drops its default `lzma` feature (gz, bgz, bz2, zstd
   kept; their default sub-features are enabled through direct `bgzip`, `bzip2`,
   `flate2`, `zstd` dependencies, because Cargo rejects `dep/default` entries in a
   dependency's feature list). Reason: niffler 3.0.1+ pulls `liblzma` 0.4 while
   needletail pulls 0.3, so two `liblzma-sys` claim `links = "lzma"` as soon as
   spearfish or salmon's paraseq is in the same tree. xz-compressed reads are not
   supported by this fork.
5. `Cargo.lock`: upstream's lock, with only the dependency edges those two
   changes alter. No crate version moved.

## Use from reaprs

`oarfish = { path = "crates/oarfish", default-features = false }`. `default-features
= false` drops oarfish's mimalloc; reaprs installs its own allocator.

## Known issues

- `rammap-core-temp` 1.2.0 stays in the dependency tree (the rammap backend and
  genome mode use it; the spearfish read path does not). Its manifest says MIT; the
  published crate ships no licence file. Expansion-plan D18 covers gating it.
- Multi-thread runs are not byte-reproducible (EM summation order, about 1e-13
  on NumReads, same as upstream). Single-thread runs are.
