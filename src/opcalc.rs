pub mod op_calc {
    pub struct OptionResults {
        pub call: f64,
        pub put: f64,
    }

    pub fn calculate_option_values(&option: &crate::BSOption) -> OptionResults {
        crate::utils::set_panic_hook();

        // calculate call value
        let asset_price_factor = (-option.div_continuous() * option.time_to_maturity).exp();
        let discounted_asset_price = option.asset_price * asset_price_factor;
        //  call_pt1 = S_t * N(d1)
        let call_pt1 = discounted_asset_price * crate::BSOption::normdist(option.d1());

        let strike_factor = (-option.r_continuous() * option.time_to_maturity).exp();
        //  call_pt2 = K * e^(-r*t) * N(d2)
        let call_pt2 = option.strike * strike_factor * crate::BSOption::normdist(option.d2());

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

    pub fn calculate_deltas(&option: &crate::BSOption) -> OptionResults {
        crate::utils::set_panic_hook();

        let delta_factor = -option.div_continuous() * option.time_to_maturity;
        let call_delta = delta_factor.exp() * crate::BSOption::normdist(option.d1());
        let put_delta = call_delta - delta_factor.exp();

        OptionResults {
            call: call_delta,
            put: put_delta,
        }
    }

    pub fn calculate_gammas(&option: &crate::BSOption) -> OptionResults {
        crate::utils::set_panic_hook();

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

    pub fn calculate_vegas(&option: &crate::BSOption) -> OptionResults {
        crate::utils::set_panic_hook();

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

    pub fn calculate_thetas(&option: &crate::BSOption) -> OptionResults {
        crate::utils::set_panic_hook();

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
    use crate::*;

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
