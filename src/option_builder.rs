use crate::BSOption;
use crate::OptionTimeDefinition;
use std::fmt;
use wasm_bindgen::prelude::*;

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

#[derive(Default, Copy, Clone)]
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
    pub fn new() -> BSOptionBuilder {
        BSOptionBuilder {
            payout_rate: 0.0,
            ..Default::default()
        }
    }

    pub fn with_asset_price(self, asset_price: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            asset_price: Some(asset_price),
            ..self
        }
    }

    pub fn with_strike(self, strike: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            strike: Some(strike),
            ..self
        }
    }

    pub fn with_time(self, opts: OptionTimeDefinition) -> BSOptionBuilder {
        match opts {
            OptionTimeDefinition {
                time_curr,
                time_maturity,
            } => BSOptionBuilder {
                time_curr: Some(time_curr),
                time_maturity: Some(time_maturity),
                time_to_maturity: Some(BSOption::calc_time_to_maturity(OptionTimeDefinition {
                    time_curr,
                    time_maturity,
                })),
                ..self
            },
        }
    }

    pub fn with_volatility(self, volatility: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            volatility: Some(volatility),
            ..self
        }
    }

    pub fn with_interest(self, interest: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            interest: Some(interest),
            ..self
        }
    }

    pub fn with_payout_rate(self, payout_rate: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            payout_rate,
            ..self
        }
    }

    pub fn create(self) -> Result<BSOption, OptionMissingBuildStepError> {
        match self {
            BSOptionBuilder {
                time_curr: None, ..
            } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_time".to_string(),
            }),

            BSOptionBuilder {
                time_maturity: None,
                ..
            } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_time".to_string(),
            }),

            BSOptionBuilder {
                time_to_maturity: None,
                ..
            } => Err(OptionMissingBuildStepError {
                missing_step_name: "with_time".to_string(),
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
pub struct WasmBSOptionBuilder {
    _inner_builder: BSOptionBuilder,
}

#[wasm_bindgen]
impl WasmBSOptionBuilder {
    pub fn new() -> WasmBSOptionBuilder {
        WasmBSOptionBuilder {
            _inner_builder: BSOptionBuilder::new(),
        }
    }

    pub fn with_asset_price(self, asset_price: f64) -> WasmBSOptionBuilder {
        self._inner_builder.with_asset_price(asset_price);
        self
    }

    pub fn with_strike(self, strike: f64) -> WasmBSOptionBuilder {
        self._inner_builder.with_strike(strike);
        self
    }

    pub fn with_time(self, opts: OptionTimeDefinition) -> WasmBSOptionBuilder {
        self._inner_builder.with_time(opts);
        self
    }

    pub fn with_volatility(self, volatility: f64) -> WasmBSOptionBuilder {
        self._inner_builder.with_volatility(volatility);
        self
    }

    pub fn with_interest(self, interest: f64) -> WasmBSOptionBuilder {
        self._inner_builder.with_interest(interest);
        self
    }

    pub fn with_payout_rate(self, payout_rate: f64) -> WasmBSOptionBuilder {
        self._inner_builder.with_payout_rate(payout_rate);
        self
    }

    pub fn create(self) -> Result<BSOption, JsValue> {
        match self._inner_builder.create() {
            Ok(option) => Ok(option),
            Err(e) => Err(JsValue::from(e)),
        }
    }
}
