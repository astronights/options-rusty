use crate::{OptionPricingModel,OptionType};

pub struct BinomialModel {
    pub underlying: f64,
    pub strike: f64,
    pub maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
    pub steps: u32,
    pub option_type: OptionType
}

impl OptionPricingModel for BinomialModel {
    fn price(&self) -> f64 {
        let dt = self.maturity / self.steps as f64; // Δt: Time step size
        let u = f64::exp(self.volatility * f64::sqrt(dt)); // Up factor: u = e^(σ√Δt)
        let d = 1.0 / u; // Down factor: d = 1 / u
        let p = (f64::exp(self.risk_free_rate * dt) - d) / (u - d); // Risk-neutral probability

        // Vector to store option values at each node (initializing with 0s)
        let mut option_values: Vec<f64> = vec![0.0; (self.steps + 1) as usize];

        // Compute option values at maturity (step N)
        for i in 0..=self.steps {
            // Price of the underlying asset at node (N, i) is S * u^i * d^(N-i)
            let asset_price_at_maturity = self.underlying * u.powi(i as i32) * d.powi((self.steps - i) as i32);

            // Option payoff at maturity for a call option: max(S-K, 0)
            option_values[i as usize] = f64::max(0.0, asset_price_at_maturity - self.strike);
        }

        // Traverse backward through the tree, starting from the last time step
        for step in (0..self.steps).rev() {
            for i in 0..=step {
                // Option value at node (step, i) is the discounted value at the next step
                option_values[i as usize] = (p * option_values[(i + 1) as usize] 
                                            + (1.0 - p) * option_values[i as usize])
                    * f64::exp(-self.risk_free_rate * dt);
            }
        }

        option_values[0]
    }
}