extern crate statrs;
extern crate web_sys;

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

struct OptionTimeDefinition {
    time_curr: u32,
    time_maturity: u32,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
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
        op_calc::calculate_option_values(self).call_value
    }

    pub fn call_delta(&self) -> f64 {
        op_calc::calculate_deltas(self).call_delta
    }

    pub fn call_gamma(&self) -> f64 {
        op_calc::calculate_gammas(self).call_gamma
    }

    pub fn call_vega(&self) -> f64 {
        op_calc::calculate_vegas(self).call_vega
    }

    pub fn call_theta(&self) -> f64 {
        op_calc::calculate_thetas(self).call_theta
    }

    pub fn put_value(&self) -> f64 {
        op_calc::calculate_option_values(self).put_value
    }

    pub fn put_delta(&self) -> f64 {
        op_calc::calculate_deltas(self).put_delta
    }

    pub fn put_gamma(&self) -> f64 {
        op_calc::calculate_gammas(self).put_gamma
    }

    pub fn put_vega(&self) -> f64 {
        op_calc::calculate_vegas(self).put_vega
    }

    pub fn put_theta(&self) -> f64 {
        op_calc::calculate_thetas(self).put_theta
    }

    /// Option int·rinsic property getters

    pub fn time_curr(&self) -> u32 {
        return self.time_curr;
    }

    pub fn time_maturity(&self) -> u32 {
        return self.time_maturity;
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

mod op_calc {
    pub struct OptionValueResults {
        pub call_value: f64,
        pub put_value: f64,
    }

    pub struct DeltaResults {
        pub call_delta: f64,
        pub put_delta: f64,
    }

    pub struct GammaResults {
        pub call_gamma: f64,
        pub put_gamma: f64,
    }

    pub struct VegaResults {
        pub call_vega: f64,
        pub put_vega: f64,
    }

    pub struct ThetaResults {
        pub call_theta: f64,
        pub put_theta: f64,
    }

    pub fn calculate_option_values(&option: &super::BSOption) -> OptionValueResults {
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

        OptionValueResults {
            call_value,
            put_value,
        }
    }

    pub fn calculate_deltas(&option: &super::BSOption) -> DeltaResults {
        super::utils::set_panic_hook();

        let delta_factor = -option.div_continuous() * option.time_to_maturity;
        let call_delta = delta_factor.exp() * super::BSOption::normdist(option.d1());
        let put_delta = call_delta - delta_factor.exp();

        DeltaResults {
            call_delta,
            put_delta,
        }
    }

    pub fn calculate_gammas(&option: &super::BSOption) -> GammaResults {
        super::utils::set_panic_hook();

        // minimum price movement unit
        const PRICE_DELTA: f64 = 0.001;

        let mut option_prime = option;
        option_prime.set_asset_price(option.asset_price() + PRICE_DELTA);

        let call_gamma = (option_prime.call_delta() - option.call_delta()) / PRICE_DELTA;
        let put_gamma = (option_prime.put_delta() - option.put_delta()) / PRICE_DELTA;

        GammaResults {
            call_gamma,
            put_gamma,
        }
    }

    pub fn calculate_vegas(&option: &super::BSOption) -> VegaResults {
        super::utils::set_panic_hook();

        const VOLATILITY_DELTA: f64 = 0.1;
        const PRICE_DELTA: f64 = 0.001;

        let mut option_prime = option;
        option_prime.set_asset_price(option.asset_price() + PRICE_DELTA);

        let call_vega = (option_prime.call_value() - option.call_value()) / VOLATILITY_DELTA;
        let put_vega = (option_prime.call_value() - option.call_value()) / VOLATILITY_DELTA;

        VegaResults {
            call_vega,
            put_vega,
        }
    }

    pub fn calculate_thetas(&option: &super::BSOption) -> ThetaResults {
        super::utils::set_panic_hook();

        const TIMESTAMP_ONE_DAY: u32 = 86_400_000;

        let mut option_prime = option;
        option_prime.set_time_maturity(option.time_maturity() + TIMESTAMP_ONE_DAY);

        let call_theta = option_prime.call_value() - option.call_value();
        let put_theta = option_prime.put_value() - option.put_value();

        ThetaResults {
            call_theta,
            put_theta,
        }
    }
}
