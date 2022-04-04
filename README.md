# SSG Rust Workshop - cross-rs

This crate builds a single binary target `magic-8-ball` that prints a random message when it is executed.
We want to deploy this on ARM and x64 Linux and Windows, because reasons.

To do this we will employ the [cross](https://github.com/cross-rs/cross) toolchain.

## Aims

- Become familiar with [cross](https://github.com/cross-rs/cross).
- Install cross and use it to build a linux binary with static linked MUSL libc.
- Using GitHub Actions, build binaries for Linux ARM64, Linux x64 and Windows. Store the binaries as artefacts, download them and verify they work.

([solution](https://github.com/DanNixon/ssg-rust-workshop-2022-04-05-cross-rs/tree/solution))
