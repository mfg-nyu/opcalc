//! Implements a Builder to create `BSOption` more conveniently.

use crate::option::{BSOption, OptionTimeDefinition};
use std::fmt;
use wasm_bindgen::prelude::*;

/// An error that indicates the caller did not call all required build
/// functions for `BSOptionBuilder`.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct OptionMissingBuildStepError {
    missing_step_name: String,
}

impl fmt::Display for OptionMissingBuildStepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Did not call {} before creating BSOption.",
            self.missing_step_name
        )
    }
}

/// Builds a `BSOption` instance.
///
/// This struct provides useful build step functions to create a BSOption,
/// progressively defining a `BSOption`'s properties as more build step
/// functions are called.
///
/// *Note*: **Rust only**. This API is exposed only for Rust. To use a builder
/// in JavaScript, use `create_option()`.
///
/// ## Using in Rust
///
/// ```rust
/// use opcalc::option::builder::BSOptionBuilder;
///
/// let option = BSOptionBuilder::new()
///     .with_asset_price(100.0)
///     .with_strike(105.0)
///     .with_interest(0.008)
///     .with_volatility(0.23)
///     .with_current_time(1_606_780_800) // timestamp in seconds: 2020/12/01 00:00:00
///     .with_maturity_time(1_610_668_800) // timestamp in seconds: 2021/01/15 00:00:00
///     .finalize()
///     .unwrap();  // safely unwrap here because all required builder steps are taken
///
/// // then, use this option to obtain calculation results
/// let gamma = option.call_gamma();
/// // ...
/// ```
#[derive(Default, Debug, Copy, Clone)]
pub struct BSOptionBuilder {
    time_curr: Option<u32>,
    time_maturity: Option<u32>,
    time_to_maturity: Option<f64>,
    asset_price: Option<f64>,
    strike: Option<f64>,
    interest: Option<f64>,
    volatility: Option<f64>,
    payout_rate: f64,
}

impl BSOptionBuilder {
    /// Create a `BSOptionBuilder` instance.
    pub fn new() -> BSOptionBuilder {
        BSOptionBuilder {
            payout_rate: 0.0,
            ..Default::default()
        }
    }

    /// Set the option's asset price.
    pub fn with_asset_price(self, asset_price: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            asset_price: Some(asset_price),
            ..self
        }
    }

    /// Set the option's strike price.
    pub fn with_strike(self, strike: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            strike: Some(strike),
            ..self
        }
    }

    /// Set the time at which option calculations will be based.
    ///
    /// **Argument**:
    ///
    /// - `time_curr`: a timestamp, in seconds.
    pub fn with_current_time(self, time_curr: u32) -> BSOptionBuilder {
        match self.time_maturity {
            Some(time_maturity) => BSOptionBuilder {
                time_curr: Some(time_curr),
                time_to_maturity: Some(BSOption::calc_time_to_maturity(OptionTimeDefinition {
                    time_curr,
                    time_maturity,
                })),
                ..self
            },
            None => BSOptionBuilder {
                time_curr: Some(time_curr),
                ..self
            },
        }
    }

    /// Set the option's maturity time.
    ///
    /// **Argument**:
    ///
    /// - `time_maturity`: a timestamp, in seconds.
    pub fn with_maturity_time(self, time_maturity: u32) -> BSOptionBuilder {
        match self.time_curr {
            Some(time_curr) => BSOptionBuilder {
                time_maturity: Some(time_maturity),
                time_to_maturity: Some(BSOption::calc_time_to_maturity(OptionTimeDefinition {
                    time_curr,
                    time_maturity,
                })),
                ..self
            },
            None => BSOptionBuilder {
                time_maturity: Some(time_maturity),
                ..self
            },
        }
    }

    /// Set a volatility that will be used for option calculation.
    pub fn with_volatility(self, volatility: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            volatility: Some(volatility),
            ..self
        }
    }

    /// Set an interest rate that will be used for option calculation.
    pub fn with_interest(self, interest: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            interest: Some(interest),
            ..self
        }
    }

    /// Set a payout rate that will be used for option calculation.
    /// This setting is optional.
    pub fn with_payout_rate(self, payout_rate: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            payout_rate,
            ..self
        }
    }

    /// Returns a `BSOption` if all required build steps are called, or
    /// an `OptionMissingBuildStepError`.
    pub fn finalize(self) -> Result<BSOption, OptionMissingBuildStepError> {
        match self {
            BSOptionBuilder {
                time_curr: None, ..
            } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_current_time".to_string(),
            }),

            BSOptionBuilder {
                time_maturity: None,
                ..
            } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_maturity_time".to_string(),
            }),

            BSOptionBuilder {
                time_to_maturity: None,
                ..
            } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_maturity_time | with_current_time".to_string(),
            }),

            BSOptionBuilder {
                asset_price: None, ..
            } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_asset_price".to_string(),
            }),

            BSOptionBuilder { strike: None, .. } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_strike".to_string(),
            }),

            BSOptionBuilder { interest: None, .. } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_interest".to_string(),
            }),

            BSOptionBuilder {
                volatility: None, ..
            } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_volatility".to_string(),
            }),

            BSOptionBuilder {
                time_curr: Some(time_curr),
                time_maturity: Some(time_maturity),
                time_to_maturity: Some(time_to_maturity),
                asset_price: Some(asset_price),
                strike: Some(strike),
                interest: Some(interest),
                volatility: Some(volatility),
                ..
            } => Ok(BSOption {
                time_curr,
                time_maturity,
                time_to_maturity,
                asset_price,
                strike,
                interest,
                volatility,
                payout_rate: self.payout_rate,
            }),
        }
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct WasmBSOptionBuilder {
    _inner_builder: BSOptionBuilder,
}

/// Creates a `BSOption` instance.
///
/// The order of invoking the `with_*()` build step methods does not matter.
///
/// *Note*: **JS only**. This API is created for the library's WebAssembly
/// bindings. To use a builder to create an option in Rust,
/// see `BSOptionBuilder`.
///
/// ## Using in JS
///
/// ```javascript
/// // import opcalc here
///
/// const option = opcalc.create_option()
///     .with_asset_price(100)
///     .with_strike(105)
///     .with_interest(0.008)
///     .with_volatility(0.23)
///     .with_current_time(1606780800) // timestamp in seconds: 2020/12/01 00:00:00
///     .with_maturity_time(1610668800) // timestamp in seconds: 2021/01/15 00:00:00
///     .finalize();
///
/// // then, use this option to obtain calculation results
/// const gamma = option.call_gamma();
/// // ...
/// ```
#[wasm_bindgen]
pub fn create_option() -> WasmBSOptionBuilder {
    WasmBSOptionBuilder::new()
}

#[wasm_bindgen]
impl WasmBSOptionBuilder {
    pub fn new() -> WasmBSOptionBuilder {
        WasmBSOptionBuilder {
            _inner_builder: BSOptionBuilder::new(),
        }
    }

    pub fn with_asset_price(mut self, asset_price: f64) -> WasmBSOptionBuilder {
        self._inner_builder = self._inner_builder.with_asset_price(asset_price);
        self
    }

    pub fn with_strike(mut self, strike: f64) -> WasmBSOptionBuilder {
        self._inner_builder = self._inner_builder.with_strike(strike);
        self
    }

    pub fn with_current_time(mut self, time_curr: u32) -> WasmBSOptionBuilder {
        self._inner_builder = self._inner_builder.with_current_time(time_curr);
        self
    }

    pub fn with_maturity_time(mut self, time_maturity: u32) -> WasmBSOptionBuilder {
        self._inner_builder = self._inner_builder.with_maturity_time(time_maturity);
        self
    }

    pub fn with_volatility(mut self, volatility: f64) -> WasmBSOptionBuilder {
        self._inner_builder = self._inner_builder.with_volatility(volatility);
        self
    }

    pub fn with_interest(mut self, interest: f64) -> WasmBSOptionBuilder {
        self._inner_builder = self._inner_builder.with_interest(interest);
        self
    }

    pub fn with_payout_rate(mut self, payout_rate: f64) -> WasmBSOptionBuilder {
        self._inner_builder = self._inner_builder.with_payout_rate(payout_rate);
        self
    }

    pub fn finalize(self) -> Result<BSOption, JsValue> {
        match self._inner_builder.finalize() {
            Ok(option) => Ok(option),
            Err(e) => Err(JsValue::from(e)),
        }
    }
}
