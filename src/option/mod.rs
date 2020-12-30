//! Option calculation based on Black-Scholes.

pub mod builder;
mod opcalc;

use self::opcalc::op_calc;
use crate::utils;
use statrs::distribution::{Normal, Univariate};
use wasm_bindgen::prelude::*;

/// An enumeration of the different supported option types.
pub enum OptionType {
    /// A call option.
    Call,
    /// A put option.
    Put,
}

/// Specifies the two timestamps required for option calculation:
/// option maturity as well as the current time.
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

    /// Get the option's call value.
    pub fn call_value(&self) -> f64 {
        op_calc::calculate_option_values(self).call
    }

    /// Get the option's call delta value.
    pub fn call_delta(&self) -> f64 {
        op_calc::calculate_deltas(self).call
    }

    /// Get the option's call gamma value.
    pub fn call_gamma(&self) -> f64 {
        op_calc::calculate_gammas(self).call
    }

    /// Get the option's call vega value.
    pub fn call_vega(&self) -> f64 {
        op_calc::calculate_vegas(self).call
    }

    /// Get the option's call theta value.
    pub fn call_theta(&self) -> f64 {
        op_calc::calculate_thetas(self).call
    }

    /// Get the option's put value.
    pub fn put_value(&self) -> f64 {
        op_calc::calculate_option_values(self).put
    }

    /// Get the option's put delta value.
    pub fn put_delta(&self) -> f64 {
        op_calc::calculate_deltas(self).put
    }

    /// Get the option's put gamma value.
    pub fn put_gamma(&self) -> f64 {
        op_calc::calculate_gammas(self).put
    }

    /// Get the option's put vega value.
    pub fn put_vega(&self) -> f64 {
        op_calc::calculate_vegas(self).put
    }

    /// Get the option's put theta value.
    pub fn put_theta(&self) -> f64 {
        op_calc::calculate_thetas(self).put
    }

    /// Get the option's time at which calculation is based.
    /// The time's unit is second-based timestamp.
    pub fn time_curr(&self) -> u32 {
        self.time_curr
    }

    /// Get the option's specified maturity time.
    /// The time's unit is second-based timestamp.
    pub fn time_maturity(&self) -> u32 {
        self.time_maturity
    }

    /// Get the option's time to maturity.
    /// Time to maturity is specified as a fraction of 365 days.
    /// For instance, 33 days to maturity has a time to maturity of `0.090410959`.
    pub fn time_to_maturity(&self) -> f64 {
        self.time_to_maturity
    }

    /// Get the option's specified asset price.
    pub fn asset_price(&self) -> f64 {
        self.asset_price
    }

    /// Get the option's specified strike price.
    pub fn strike(&self) -> f64 {
        self.strike
    }

    /// Get the option's specified interest rate.
    pub fn interest(&self) -> f64 {
        self.interest
    }

    /// Get the option's specified implied volatility.
    pub fn volatility(&self) -> f64 {
        self.volatility
    }

    /// Get the option's specified payout rate.
    pub fn payout_rate(&self) -> f64 {
        self.payout_rate
    }

    /// Update the time at which the option's calculation is based.
    ///
    /// **Arguments:**
    ///
    /// - `new_time_curr`: a timestamp, in seconds, that represents the updated
    ///     time to perform option calcultions at.
    pub fn set_time_curr(&mut self, new_time_curr: u32) {
        self.time_curr = new_time_curr;
        self.time_to_maturity = Self::calc_time_to_maturity(OptionTimeDefinition {
            time_maturity: self.time_maturity,
            time_curr: self.time_curr,
        });
    }

    /// Update the option's maturity time.
    ///
    /// **Arguments:**
    ///
    /// - `new_time_maturity`: a timestamp, in seconds, that is the option's
    ///      time of maturity.
    pub fn set_time_maturity(&mut self, new_time_maturity: u32) {
        self.time_maturity = new_time_maturity;
        self.time_to_maturity = Self::calc_time_to_maturity(OptionTimeDefinition {
            time_maturity: self.time_maturity,
            time_curr: self.time_curr,
        });
    }

    /// Update the option's asset price.
    ///
    /// **Arguments:**
    ///
    /// - `new_asset_price`: the option's new asset price.
    pub fn set_asset_price(&mut self, new_asset_price: f64) {
        self.asset_price = new_asset_price;
    }

    /// Update the option's strike price.
    ///
    /// **Arguments:**
    ///
    /// - `new_strike`: the option's new strike price.
    pub fn set_strike(&mut self, new_strike: f64) {
        self.strike = new_strike;
    }

    /// Update the option's volatility that will be used for calculation.
    ///
    /// **Arguments:**
    ///
    /// - `new_volatility`: the option's new volatility.
    pub fn set_volatility(&mut self, new_volatility: f64) {
        self.volatility = new_volatility;
    }

    /// Update the option's payout rate.
    ///
    /// **Arguments:**
    ///
    /// - `new_payout_rate`: the option's new payout rate.
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
