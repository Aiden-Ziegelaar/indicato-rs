use indicato_rs_proc::{Apply, Evaluate};

use crate::{
    error::{FinError, FinErrorType},
    traits::{Apply, Current, Evaluate, Executable, ExecutionContext, IoState},
};

use super::ExponentialMovingAverage;

/// # Moving Average Convergence Divergence
/// Container for Moving Average Convergence Divergence (MACD) aggregation
///
/// The aggregation will begin producing values immediately, the first value
/// will be zero as both EMAs will use the input as the first value, after
/// which the following formula is applied:
/// <br>
/// <br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
///    <mrow>
///         <msub>
///             <mi>o</mi>
///             <mn>n</mn>
///         </msub>
///         <mo>=</mo>
///         <mrow>
///             <msub>
///                 <mi>EMA</mi>
///                 <mn>S</mn>
///             </msub>
///             <mo>(</mo>
///             <msub>
///                 <mi>i</mi>
///                 <mn>n</mn>
///             </msub>
///             <mo>)</mo>
///             <mo>-</mo>
///             <msub>
///                 <mi>EMA</mi>
///                 <mn>L</mn>
///             </msub>
///             <mo>(</mo>
///             <msub>
///                 <mi>i</mi>
///                 <mn>n</mn>
///             </msub>
///             <mo>)</mo>
///         </mrow>
///    </mrow>
/// </semantics>
/// </math>
/// <br>
/// Where `o` is the output, `n` is the current step, `EMA` is the Exponential Moving Average, `S` is the short period, `L` is the long period and `i` is the input.
///
/// _NB._ This will not produce a signal line, you will need to produce your own signal line from the MACD output.
///
/// # Example Usage
/// ```
/// use indicato_rs::signals::MovingAverageConvergenceDivergence;
/// use indicato_rs::traits::{Apply, Evaluate, Current};
///
/// #[macro_use]
/// use approx::assert_abs_diff_eq;
///
/// let mut macd = MovingAverageConvergenceDivergence::new(2, 4).unwrap();
///
/// // apply some values and check their output
/// assert_eq!(macd.apply(3.0), 0.0);
/// assert_abs_diff_eq!(macd.apply(4.8), 0.48, epsilon = 10e-7);
/// assert_abs_diff_eq!(macd.apply(6.3), 0.848, epsilon =  10e-7);
/// assert_abs_diff_eq!(macd.apply(5.0), 0.3488, epsilon = 10e-7);
///
/// // evaluate some values, these won't affect the internal state of the MACD
/// assert_abs_diff_eq!(macd.evaluate(10.0), 1.48928, epsilon = 10e-7);
///
/// // fetch the current value of the MACD
/// assert_abs_diff_eq!(macd.current(),  0.3488, epsilon = 10e-7);
/// ```
#[derive(Apply, Evaluate)]
pub struct MovingAverageConvergenceDivergence {
    short_ema: ExponentialMovingAverage,
    long_ema: ExponentialMovingAverage,
}

impl IoState for MovingAverageConvergenceDivergence {
    type Input = f64;
    type Output = f64;
}

impl MovingAverageConvergenceDivergence {
    /// Create a new Moving Average Convergence Divergence (MACD) aggregation
    ///
    /// # Arguments
    ///
    /// * `short_period` - The period for the short Exponential Moving Average
    /// * `long_period` - The period for the long Exponential Moving Average
    ///
    /// _NB._ Both periods must be greater than 0, there is no requirement for the short period to be less than the long period.
    ///
    pub fn new(short_period: usize, long_period: usize) -> Result<Self, FinError> {
        match (short_period, long_period) {
            (0, _) | (_, 0) => Err(FinError::new(
                FinErrorType::InvalidInput,
                "Periods must be greater than 0",
            )),
            _ => Ok(Self {
                short_ema: ExponentialMovingAverage::new(short_period)?,
                long_ema: ExponentialMovingAverage::new(long_period)?,
            }),
        }
    }
}

impl Current for MovingAverageConvergenceDivergence {
    fn current(&self) -> Self::Output {
        self.short_ema.current() - self.long_ema.current()
    }
}

impl Executable for MovingAverageConvergenceDivergence {
    fn execute(&mut self, input: f64, execution_context: &ExecutionContext) -> Self::Output {
        let short_ema = self.short_ema.execute(input, execution_context);
        let long_ema = self.long_ema.execute(input, execution_context);
        short_ema - long_ema
    }
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_macd() {
        let mut macd = MovingAverageConvergenceDivergence::new(2, 4).unwrap();

        assert_eq!(macd.apply(3.0), 0.0);
        assert_abs_diff_eq!(macd.apply(4.8), 0.48, epsilon = 10e-7);
        assert_abs_diff_eq!(macd.apply(6.3), 0.848, epsilon = 10e-7);
        assert_abs_diff_eq!(macd.apply(5.0), 0.3488, epsilon = 10e-7);

        assert_abs_diff_eq!(macd.evaluate(10.0), 1.48928, epsilon = 10e-7);

        assert_abs_diff_eq!(macd.current(), 0.3488, epsilon = 10e-7);
    }

    #[test]
    fn test_macd_new_invalid() {
        assert!(MovingAverageConvergenceDivergence::new(0, 0).is_err());
        assert!(MovingAverageConvergenceDivergence::new(0, 1).is_err());
        assert!(MovingAverageConvergenceDivergence::new(1, 0).is_err());
    }
}