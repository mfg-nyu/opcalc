pub mod builder;
mod opcalc;

use self::opcalc::op_calc;
use crate::utils;
use statrs::distribution::{Normal, Univariate};
use wasm_bindgen::prelude::*;

pub enum OptionType {
    Call,
    Put,
}

#[wasm_bindgen]
pub struct OptionTimeDefinition {
    time_curr: u32,
    time_maturity: u32,
}

/// A Black-Scholes option.
///
/// Contains properties such as maturity time, strike price, and more necessary
/// for Black-Scholes calculation.
///
/// See the static `new()` method and `create_option()` for how to instantiate
/// a `BSOption` instance.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct BSOption {
    time_curr: u32,
    time_maturity: u32,
    time_to_maturity: f64,
    asset_price: f64,
    strike: f64,
    interest: f64,
    volatility: f64,
    payout_rate: f64,
}

#[wasm_bindgen]
impl BSOption {
    /// Creates a BSOption instance.
    ///
    /// Also see `create_option()` for a more convenient option creation process.
    ///
    /// # Arguments
    ///
    /// - `time_curr`: A timestamp in seconds. This specifies the time when
    ///      option calculations should be based on.
    ///
    /// - `time_maturity`: A timestamp in seconds. This specifies the maturity
    ///     time of this option.
    ///
    /// - `asset_price`: Specifies the underlying's current price.
    ///
    /// - `strike`: Specifies the option's strike price.
    ///
    /// - `interest`: Specifies the prevailing interest rate under which this
    ///      option should be priced. The number should be provided in a
    ///      decimal form (e.g. `0.006` for '0.6%').
    ///
    /// - `volatility`: Specify an implied volatility used in pricing this
    ///      option. The number should be provided in a decimal form (e.g.
    ///      provide `0.2398` for '23.98%').
    ///
    /// - `payout_rate`: Optional. Specify a payout rate used in pricing this
    ///      option. Defaults to `0.0`.
    ///
    /// # Examples
    ///
    /// ## Creating an option in rust
    ///
    /// ```rust
    /// use opcalc::option::BSOption;
    ///
    /// let option = BSOption::new(
    ///   1_606_780_800, // time_curr, 2020/12/01 00:00:00
    ///   1_610_668_800, // time_maturity, 2021/01/15 00:00:00
    ///   100.0,         // asset_price
    ///   105.0,         // strike
    ///   0.005,         // interest
    ///   0.23,          // volatility
    ///   0.0            // payout_rate
    /// );
    /// ```
    ///
    /// ## Creating an option in JS
    ///
    /// ```js
    /// // import opcalc asynchronously
    ///
    /// const option = opcalc.BSOption.new(
    ///   1606780800,    // time_curr, 2020/12/01 00:00:00
    ///   1610668800,    // time_maturity, 2021/01/15 00:00:00
    ///   100,           // asset_price
    ///   105,           // strike
    ///   0.005,         // interest
    ///   0.23,          // volatility
    ///   0              // payout_rate
    /// )
    /// ```
    pub fn new(
        time_curr: u32,
        time_maturity: u32,
        asset_price: f64,
        strike: f64,
        interest: f64,
        volatility: f64,
        payout_rate: f64,
    ) -> BSOption {
        utils::set_panic_hook();

        BSOption {
            time_curr,
            time_maturity,
            time_to_maturity: Self::calc_time_to_maturity(OptionTimeDefinition {
                time_curr,
                time_maturity,
            }),
            asset_price,
            strike,
            interest,
            volatility,
            payout_rate,
        }
    }

    pub fn call_value(&self) -> f64 {
        op_calc::calculate_option_values(self).call
    }

    pub fn call_delta(&self) -> f64 {
        op_calc::calculate_deltas(self).call
    }

    pub fn call_gamma(&self) -> f64 {
        op_calc::calculate_gammas(self).call
    }

    pub fn call_vega(&self) -> f64 {
        op_calc::calculate_vegas(self).call
    }

    pub fn call_theta(&self) -> f64 {
        op_calc::calculate_thetas(self).call
    }

    pub fn put_value(&self) -> f64 {
        op_calc::calculate_option_values(self).put
    }

    pub fn put_delta(&self) -> f64 {
        op_calc::calculate_deltas(self).put
    }

    pub fn put_gamma(&self) -> f64 {
        op_calc::calculate_gammas(self).put
    }

    pub fn put_vega(&self) -> f64 {
        op_calc::calculate_vegas(self).put
    }

    pub fn put_theta(&self) -> f64 {
        op_calc::calculate_thetas(self).put
    }

    pub fn time_curr(&self) -> u32 {
        self.time_curr
    }

    pub fn time_maturity(&self) -> u32 {
        self.time_maturity
    }

    pub fn time_to_maturity(&self) -> f64 {
        self.time_to_maturity
    }

    pub fn asset_price(&self) -> f64 {
        self.asset_price
    }

    pub fn strike(&self) -> f64 {
        self.strike
    }

    pub fn interest(&self) -> f64 {
        self.interest
    }

    pub fn volatility(&self) -> f64 {
        self.volatility
    }

    pub fn payout_rate(&self) -> f64 {
        self.payout_rate
    }

    pub fn set_time_curr(&mut self, new_time_curr: u32) {
        self.time_curr = new_time_curr;
        self.time_to_maturity = Self::calc_time_to_maturity(OptionTimeDefinition {
            time_maturity: self.time_maturity,
            time_curr: self.time_curr,
        });
    }

    pub fn set_time_maturity(&mut self, new_time_maturity: u32) {
        self.time_maturity = new_time_maturity;
        self.time_to_maturity = Self::calc_time_to_maturity(OptionTimeDefinition {
            time_maturity: self.time_maturity,
            time_curr: self.time_curr,
        });
    }

    pub fn set_asset_price(&mut self, new_asset_price: f64) {
        self.asset_price = new_asset_price;
    }

    pub fn set_strike(&mut self, new_strike: f64) {
        self.strike = new_strike;
    }

    pub fn set_volatility(&mut self, new_volatility: f64) {
        self.volatility = new_volatility;
    }

    pub fn set_payout_rate(&mut self, new_payout_rate: f64) {
        self.payout_rate = new_payout_rate;
    }

    fn d1(&self) -> f64 {
        let s_k_ratio = self.asset_price / self.strike;
        let vol_factor = self.volatility.powi(2) / 2.0;
        let rates = self.r_continuous() - self.div_continuous() + vol_factor;

        let num = s_k_ratio.ln() + rates * self.time_to_maturity;
        let den = self.volatility * self.time_to_maturity.powf(0.5);

        num / den
    }

    fn d2(&self) -> f64 {
        self.d1() - self.volatility * self.time_to_maturity.powf(0.5)
    }

    fn r_continuous(&self) -> f64 {
        self.interest.ln_1p()
    }

    fn div_continuous(&self) -> f64 {
        self.payout_rate.ln_1p()
    }

    fn normdist(target: f64) -> f64 {
        let normdist = Normal::new(0.0, 1.0).unwrap();
        normdist.cdf(target)
    }

    fn calc_time_to_maturity(time_def: OptionTimeDefinition) -> f64 {
        const TIMESTAMP_ONE_YEAR: f64 = 31_536_000.0;

        ((time_def.time_maturity - time_def.time_curr) as f64) / TIMESTAMP_ONE_YEAR
    }
}
