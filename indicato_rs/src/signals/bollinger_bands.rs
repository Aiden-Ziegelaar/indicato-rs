use std::collections::VecDeque;

use crate::traits::{Current, Executable, ExecutionContext, IoState};
use crate::fin_error::{FinError, FinErrorType};
use crate::deque_math::DequeMathExtF64;

pub struct BollingerBands {
    typical_price: VecDeque<f64>,
    std_dev_count: f64,
    period: usize,   
}

impl BollingerBands {
    pub fn new(period: usize, std_dev_count: f64) -> Result<Self, FinError> {
        match period {
            0 => Err(FinError::new(
                FinErrorType::InvalidInput,
                "Period must be greater than 0",
            )),
            _ => Ok(Self {
                typical_price: VecDeque::with_capacity(period),
                std_dev_count,
                period,
            }),
        }
    }
}

impl IoState for BollingerBands {
    /// Input is a tuple of (high, low, close)
    type Input = (f64, f64, f64);
    /// Output is a tuple of (upper_band, typical_price_sma, lower_band)
    type Output = (f64, f64, f64);
}

impl Executable for BollingerBands {
    fn execute(&mut self, input: Self::Input, execution_context: &ExecutionContext) -> Self::Output {
        let typical_price = (input.0 + input.1 + input.2) / 3.0;
        let mean: f64;
        let std_dev: f64;
        match execution_context {
            ExecutionContext::Apply => {
                self.typical_price.push_back(typical_price);
                if self.typical_price.len() > self.period {
                    self.typical_price.pop_front();
                }
                mean = self.typical_price.mean();
                std_dev = self.typical_price.standard_deviation();
            }
            ExecutionContext::Evaluate => {
                let mut typical_price_clone = self.typical_price.clone();
                typical_price_clone.push_back(typical_price);
                if typical_price_clone.len() > self.period {
                    typical_price_clone.pop_front();
                }
                mean = typical_price_clone.mean();
                std_dev = typical_price_clone.standard_deviation();
            }
        }
        let upper_band = mean + (std_dev * self.std_dev_count);
        let lower_band = mean - (std_dev * self.std_dev_count);
        (upper_band, mean, lower_band)
    }
}

impl Current for BollingerBands{
    fn current(&self) -> (f64, f64, f64) {
        let mean = self.typical_price.mean();
        let std_dev = self.typical_price.standard_deviation();
        let upper_band = mean + (std_dev * self.std_dev_count);
        let lower_band = mean - (std_dev * self.std_dev_count);
        (upper_band, mean, lower_band)
    }
}
