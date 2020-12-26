extern crate approx;
extern crate statrs;
extern crate web_sys;
use std::error::Error;

mod utils;

use statrs::distribution::{Normal, Univariate};
use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub enum OptionType {
    Call,
    Put,
}

pub struct OptionTimeDefinition {
    time_curr: u32,
    time_maturity: u32,
}

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

        let option = BSOption {
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
        };

        option
    }

    /// Option derived property getters (they're read-only)

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

    /// Option int·rinsic property getters

    pub fn time_curr(&self) -> u32 {
        return self.time_curr;
    }

    pub fn time_maturity(&self) -> u32 {
        return self.time_maturity;
    }

    pub fn time_to_maturity(&self) -> f64 {
        return self.time_to_maturity;
    }

    pub fn asset_price(&self) -> f64 {
        return self.asset_price;
    }

    pub fn strike(&self) -> f64 {
        return self.strike;
    }

    pub fn interest(&self) -> f64 {
        return self.interest;
    }

    pub fn volatility(&self) -> f64 {
        return self.volatility;
    }

    pub fn payout_rate(&self) -> f64 {
        return self.payout_rate;
    }

    /// Option intrinsic property setters

    #[allow(dead_code)]
    fn set_time_curr(&mut self, new_time_curr: u32) {
        self.time_curr = new_time_curr;
        self.time_to_maturity = Self::calc_time_to_maturity(OptionTimeDefinition {
            time_maturity: self.time_maturity,
            time_curr: self.time_curr,
        });
    }

    #[allow(dead_code)]
    fn set_time_maturity(&mut self, new_time_maturity: u32) {
        self.time_maturity = new_time_maturity;
        self.time_to_maturity = Self::calc_time_to_maturity(OptionTimeDefinition {
            time_maturity: self.time_maturity,
            time_curr: self.time_curr,
        });
    }

    #[allow(dead_code)]
    fn set_asset_price(&mut self, new_asset_price: f64) {
        self.asset_price = new_asset_price;
    }

    #[allow(dead_code)]
    fn set_strike(&mut self, new_strike: f64) {
        self.strike = new_strike;
    }

    #[allow(dead_code)]
    fn set_volatility(&mut self, new_volatility: f64) {
        self.volatility = new_volatility;
    }

    #[allow(dead_code)]
    fn set_payout_rate(&mut self, new_payout_rate: f64) {
        self.payout_rate = new_payout_rate;
    }

    /// Internal calculation methods

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
        let ttm = ((time_def.time_maturity - time_def.time_curr) as f64) / TIMESTAMP_ONE_YEAR;

        ttm
    }
}

#[derive(Default)]
pub struct BSOptionBuilder {
    time_curr: Option<u32>,
    time_maturity: Option<u32>,
    time_to_maturity: f64,
    asset_price: f64,
    strike: f64,
    interest: f64,
    volatility: f64,
    payout_rate: f64,
}

impl BSOptionBuilder {
    pub fn new() -> BSOptionBuilder {
        BSOptionBuilder {
            ..Default::default()
        }
    }

    pub fn with_asset_price(self, asset_price: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            asset_price,
            ..self
        }
    }

    pub fn with_strike(self, strike: f64) -> BSOptionBuilder {
        BSOptionBuilder { strike, ..self }
    }

    pub fn with_time(self, opts: OptionTimeDefinition) -> BSOptionBuilder {
        match opts {
            OptionTimeDefinition {
                time_curr,
                time_maturity,
            } => BSOptionBuilder {
                time_curr: Some(time_curr),
                time_maturity: Some(time_maturity),
                time_to_maturity: BSOption::calc_time_to_maturity(OptionTimeDefinition {
                    time_curr,
                    time_maturity,
                }),
                ..self
            },
        }
    }

    pub fn with_volatility(self, volatility: f64) -> BSOptionBuilder {
        BSOptionBuilder { volatility, ..self }
    }

    pub fn with_interest(self, interest: f64) -> BSOptionBuilder {
        BSOptionBuilder { interest, ..self }
    }

    pub fn with_payout_rate(self, payout_rate: f64) -> BSOptionBuilder {
        BSOptionBuilder {
            payout_rate,
            ..self
        }
    }

    pub fn create(self) -> Result<BSOption, Box<dyn Error>> {
        match self {
            BSOptionBuilder {
                time_curr: None, ..
            } => Err("Did not call `with_time` before creating BSOption.".into()),

            BSOptionBuilder {
                time_maturity: None,
                ..
            } => Err("Did not call `with_time` before creating BSOption.".into()),

            BSOptionBuilder {
                time_curr: Some(time_curr),
                time_maturity: Some(time_maturity),
                ..
            } => Ok(BSOption {
                time_curr,
                time_maturity,
                time_to_maturity: self.time_to_maturity,
                asset_price: self.asset_price,
                strike: self.strike,
                interest: self.interest,
                volatility: self.volatility,
                payout_rate: self.payout_rate,
            }),
        }
    }
}

mod op_calc {
    pub struct OptionResults {
        pub call: f64,
        pub put: f64,
    }

    pub fn calculate_option_values(&option: &super::BSOption) -> OptionResults {
        super::utils::set_panic_hook();

        // calculate call value
        let asset_price_factor = (-option.div_continuous() * option.time_to_maturity).exp();
        let discounted_asset_price = option.asset_price * asset_price_factor;
        //  call_pt1 = S_t * N(d1)
        let call_pt1 = discounted_asset_price * super::BSOption::normdist(option.d1());

        let strike_factor = (-option.r_continuous() * option.time_to_maturity).exp();
        //  call_pt2 = K * e^(-r*t) * N(d2)
        let call_pt2 = option.strike * strike_factor * super::BSOption::normdist(option.d2());

        let call_value = call_pt1 - call_pt2;

        // calculate put value, which can be derived from call's value
        let put_pt1 = option.asset_price * (-option.div_continuous() * option.r_continuous()).exp();
        let put_pt2 = option.strike * (-option.r_continuous() * option.time_to_maturity).exp();

        let put_value = call_value - put_pt1 + put_pt2;

        OptionResults {
            call: call_value,
            put: put_value,
        }
    }

    pub fn calculate_deltas(&option: &super::BSOption) -> OptionResults {
        super::utils::set_panic_hook();

        let delta_factor = -option.div_continuous() * option.time_to_maturity;
        let call_delta = delta_factor.exp() * super::BSOption::normdist(option.d1());
        let put_delta = call_delta - delta_factor.exp();

        OptionResults {
            call: call_delta,
            put: put_delta,
        }
    }

    pub fn calculate_gammas(&option: &super::BSOption) -> OptionResults {
        super::utils::set_panic_hook();

        // minimum price movement unit
        const PRICE_DELTA: f64 = 0.001;

        let mut option_prime = option;
        option_prime.set_asset_price(option.asset_price() + PRICE_DELTA);

        let call_gamma = (option_prime.call_delta() - option.call_delta()) / PRICE_DELTA;
        let put_gamma = (option_prime.put_delta() - option.put_delta()) / PRICE_DELTA;

        OptionResults {
            call: call_gamma,
            put: put_gamma,
        }
    }

    pub fn calculate_vegas(&option: &super::BSOption) -> OptionResults {
        super::utils::set_panic_hook();

        const VOLATILITY_DELTA: f64 = 0.0001;

        let mut option_prime = option;
        option_prime.set_volatility(option.volatility() + VOLATILITY_DELTA);

        let call_vega = (option_prime.call_value() - option.call_value()) / 0.01;
        let put_vega = (option_prime.call_value() - option.call_value()) / 0.01;

        OptionResults {
            call: call_vega,
            put: put_vega,
        }
    }

    pub fn calculate_thetas(&option: &super::BSOption) -> OptionResults {
        super::utils::set_panic_hook();

        const TIMESTAMP_ONE_DAY: u32 = 86_400;

        let mut option_prime = option;
        option_prime.set_time_curr(option.time_curr() + TIMESTAMP_ONE_DAY);

        let call_theta = option_prime.call_value() - option.call_value();
        let put_theta = option_prime.put_value() - option.put_value();

        OptionResults {
            call: call_theta,
            put: put_theta,
        }
    }
}

#[cfg(test)]
mod opcalc_tests {
    use super::*;

    fn create_test_option() -> BSOption {
        let time_curr = 1606780800; // 2020/12/01 00:00:00
        let time_maturity = 1610668800; // 2021/01/15 00:00:00

        let asset_price = 100.0;
        let strike = 105.0;
        let interest = 0.005;
        let volatility = 0.23;
        let payout_rate = 0.0;

        BSOption::new(
            time_curr,
            time_maturity,
            asset_price,
            strike,
            interest,
            volatility,
            payout_rate,
        )
    }

    #[test]
    fn calculates_option_values() {
        let option_vals = op_calc::calculate_option_values(&create_test_option());

        approx::assert_abs_diff_eq!(option_vals.call, 1.402645442104692, epsilon = f64::EPSILON);
        approx::assert_abs_diff_eq!(option_vals.put, 6.338100538847982, epsilon = f64::EPSILON);
    }

    #[test]
    fn calculates_option_deltas() {
        let deltas = op_calc::calculate_deltas(&create_test_option());

        approx::assert_abs_diff_eq!(deltas.call, 0.2890519431809007, epsilon = f64::EPSILON);
        approx::assert_abs_diff_eq!(deltas.put, -0.7109480568190993, epsilon = f64::EPSILON);
    }

    #[test]
    fn calculates_option_gammas() {
        let gammas = op_calc::calculate_gammas(&create_test_option());

        // TODO: investigate whether gamma should be absolutely equal for calls and puts.
        approx::assert_abs_diff_eq!(gammas.call, 0.04232231027889721, epsilon = f64::EPSILON);
        approx::assert_abs_diff_eq!(gammas.put, 0.042322310279008235, epsilon = f64::EPSILON);
    }

    #[test]
    fn calculates_option_vegas() {
        let vegas = op_calc::calculate_vegas(&create_test_option());

        approx::assert_abs_diff_eq!(vegas.call, 0.12001554434952766, epsilon = f64::EPSILON);
        approx::assert_abs_diff_eq!(vegas.put, 0.12001554434952766, epsilon = f64::EPSILON);
    }

    #[test]
    fn calculates_option_thetas() {
        let thetas = op_calc::calculate_thetas(&create_test_option());

        approx::assert_abs_diff_eq!(thetas.call, -0.03115177341956965, epsilon = f64::EPSILON);
        approx::assert_abs_diff_eq!(thetas.put, -0.029717873380988635, epsilon = f64::EPSILON);
    }
}
