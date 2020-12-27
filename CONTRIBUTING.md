# Contributing

## Installing `wasm-pack`

**Revisit this section when `wasm-pack` releases a version newer than `v0.9.1`**.

The latest `wasm-pack` version (`v0.9.1`) includes a bug that omits `*_bg.js`
and `*_bg.d.ts` files in the generated JS package.

This issue has been fixed in `wasm-pack`'s latest commits, but it has not been
published to a new version.

As a temporary fix, install `wasm-pack` from the latest GitHub source tree,
like so:

```sh
# do this
cargo install --git https://github.com/rustwasm/wasm-pack.git

# not this
cargo install wasm-pack
```

This is a known issue in `wasm-pack`.
See: <https://github.com/rustwasm/wasm-pack/issues/837>.
