pub mod builder;
mod opcalc;

use crate::utils;
use opcalc::op_calc;
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
