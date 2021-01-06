# opcalc

**An easy-to-use black-scholes option calculator. Made for JS, built in Rust.**

## Getting started

### Using the library in JavaScript

You may configure an option calculation instance like so:

```js
// Import opcalc as a webassembly module

const option = opcalc
    .create_option()
    .with_asset_price(100)
    .with_strike(105)
    .with_volatility(0.2)
    .with_interest(0.005)
    .with_current_time(1606780800) // timestamp in seconds: 2020/12/01 00:00:00
    .with_maturity_time(1610668800) // timestamp in seconds: 2021/01/15 00:00:00
    .finalize();
```

After the option has been created, you may access its prices (call and put),
greeks (delta, gamma, and more) like so:

```js
const call = option.call_value();
const delta = option.call_delta();
// ...
```

### Using the library in Rust

Using the library in Rust is similar to the experience in JavaScript.

```rust
use opcalc::option::builder::BSOptionBuilder;

let option = BSOptionBuilder::new()
    .with_asset_price(100.0)
    .with_strike(105.0)
    .with_interest(0.008)
    .with_volatility(0.23)
    .with_current_time(1_606_780_800) // timestamp in seconds: 2020/12/01 00:00:00
    .with_maturity_time(1_610_668_800) // timestamp in seconds: 2021/01/15 00:00:00
    .finalize()
    .unwrap();  // safely unwrap here because all required builder steps are taken

// then, use this option to obtain calculation results
let gamma = option.call_gamma();
```
