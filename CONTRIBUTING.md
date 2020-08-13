# Contributing

## Packaging

After building the package and before publishing, remember to add this line
under 'files' in 'pkg/package.json':

```txt
    "*_bg.js",
    "*_bg.d.ts",
```

This is a known issue in `wasm-pack`, which neglects to publish any `*_bg.js`
files.

See: <https://github.com/rustwasm/wasm-pack/issues/837>.
