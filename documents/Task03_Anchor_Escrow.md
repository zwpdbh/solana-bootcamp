# Anchor Escrow -- 托管服务

## References 

- [Anchor Escrow -- 托管服务](https://learn.blueshift.gg/en/challenges/anchor-escrow)
- [Programs with Anchor (default choice)](https://github.com/solana-foundation/solana-dev-skill/blob/main/skill/programs-anchor.md)
- [Solana/更多开发者工具/Anchor 里的简单数据存储合约](https://accu.cc/content/solana/tool_anchor_ss/)
  - [github repo -- Pxsol simple storage with Anchor](https://github.com/mohanson/pxsol-ss-anchor/blob/master/programs/pxsol-ss-anchor/src/lib.rs)

## Troubleshooting 

Compatibility Notes for Anchor 0.32.0


When run build `anchor build`, it shows error: 

```sh 
error: failed to parse manifest at `/home/zw/.cargo/registry/src/index.crates.io-6f17d22bba15001f/constant_time_eq-0.4.2/Cargo.toml`

Caused by:
  feature `edition2024` is required

  The package requires the Cargo feature called `edition2024`, but that feature is not stabilized in this version of Cargo (1.84.0 (12fe57a9d 2025-04-07)).
  Consider trying a more recent nightly release.
  See https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#edition-2024 for more information about the status of this feature.
```

solution:

```sh
cargo update base64ct --precise 1.6.0
cargo update blake3 --precise 1.5.5

# (this one I didn't run successfully, but still solved the problem)
cargo update constant_time_eq --precise 0.4.1
```
