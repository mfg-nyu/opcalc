# Contributing

## Packaging

After building the package and before publishing, remember to add this line
under 'files' in 'pkg/package.json': `"*_bg.js"`.

This is a known issue in `wasm-pack`, which neglects to publish any `*_bg.js`
files.

See: <https://github.com/rustwasm/wasm-pack/issues/837>.
