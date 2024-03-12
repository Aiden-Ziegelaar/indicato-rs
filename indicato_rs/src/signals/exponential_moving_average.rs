use indicato_rs_proc::{Apply, Evaluate};

use crate::{
    fin_error::{FinError, FinErrorType},
    traits::{Apply, Current, Evaluate, Executable, ExecutionContext, IoState},
};

fn calculate_emas(input: f64, k: f64, current: f64, is_new: bool) -> f64 {
    match is_new {
        true => input,
        false => (input - current) * k + current,
    }
}

/// # Exponential Moving Average
/// Container for Exponential Moving Average (EMA) aggregation
///
/// The aggregation will begin producing values immediately, the first value will be the input, after which the following formula is applied:
/// <br>
/// <br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
///     <mrow>
///         <msub>
///             <mi>o</mi>
///             <mn>n</mn>
///         </msub>
///         <mo>=</mo>
///         <mrow><mo stretchy="true" form="prefix">(</mo>
///             <msub>
///                 <mi>i</mi>
///                 <mn>n</mn>
///             </msub>
///             <mo>−</mo>
///             <msub>
///                 <mi>o</mi>
///                 <mn>n-1</mn>
///             </msub>
///         <mo stretchy="true" form="postfix">)</mo></mrow>
///         <mo>⋅</mo>
///         <mfrac>
///             <mn>2</mn>
///             <mrow><mi>p</mi><mo>+</mo><mn>1</mn></mrow>
///         </mfrac>
///         <mo>+</mo>
///         <msub>
///             <mi>o</mi>
///             <mn>n-1</mn>
///         </msub>   
///     </mrow>
/// </semantics>
/// </math>
/// <br>
/// Where `o` is the output, `n` is the current step, `n-1` is the previous step, `p` is the period of the exponential moving average and `i` is the input.
///
/// # Example Usage
/// ```
/// use indicato_rs::signals::ExponentialMovingAverage;
/// use indicato_rs::traits::{Apply, Evaluate, Current};
///
/// // create a new Exponential Moving Average with a period of 3
/// let mut ema = ExponentialMovingAverage::new(3).unwrap();
///
/// // apply some values and check their output
/// assert_eq!(ema.apply(2.0), 2.0);
/// assert_eq!(ema.apply(5.0), 3.5);
/// assert_eq!(ema.apply(1.0), 2.25);
/// assert_eq!(ema.apply(6.25), 4.25);
///
/// // evaluate some values, these won't affect the internal state of the EMA
/// assert_eq!(ema.evaluate(5.0), 4.625);
/// assert_eq!(ema.evaluate(4.0), 4.125);
///
/// // fetch the current value of the EMA
/// assert_eq!(ema.current(), 4.25);
/// ````
///
#[derive(Apply, Evaluate)]
pub struct ExponentialMovingAverage {
    current: f64,
    k: f64,
    is_new: bool,
}

impl ExponentialMovingAverage {
    /// Create a new Exponential Moving Average instance
    /// # Arguments
    /// * `period` - The period of the Exponential Moving Average aggregation, must be greater than 0
    ///
    /// # Example
    /// ```
    /// use indicato_rs::signals::ExponentialMovingAverage;
    /// use indicato_rs::traits::{Apply, Evaluate, Current};
    ///
    /// let ema = ExponentialMovingAverage::new(3);
    /// assert!(ema.is_ok());
    /// ```
    /// # Errors
    /// Will return an error if the period is 0
    /// ```
    /// use indicato_rs::signals::ExponentialMovingAverage;
    ///
    /// let ema = ExponentialMovingAverage::new(0);
    ///
    /// assert!(ema.is_err());
    /// ```
    pub fn new(period: usize) -> Result<Self, FinError> {
        match period {
            0 => Err(FinError::new(
                FinErrorType::InvalidInput,
                "Period must be greater than 0",
            )),
            _ => Ok(Self {
                k: 2.0 / (period + 1) as f64,
                current: 0.0,
                is_new: true,
            }),
        }
    }
}

impl IoState for ExponentialMovingAverage {
    type Input = f64;
    type Output = f64;
}

impl Executable for ExponentialMovingAverage {
    fn execute(&mut self, input: f64, execution_context: &ExecutionContext) -> Self::Output {
        let result = calculate_emas(input, self.k, self.current, self.is_new);
        match execution_context {
            ExecutionContext::Apply => {
                self.current = result;
                self.is_new = false;
            }
            ExecutionContext::Evaluate => {}
        }
        result
    }
}

impl Current for ExponentialMovingAverage {
    fn current(&self) -> f64 {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();
        assert_eq!(ema.apply(2.0), 2.0);
        assert_eq!(ema.apply(5.0), 3.5);
        assert_eq!(ema.apply(1.0), 2.25);
        assert_eq!(ema.apply(6.25), 4.25);
    }

    #[test]
    fn test_evaluate() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();
        assert_eq!(ema.apply(1.0), 1.0);
        assert_eq!(ema.apply(2.0), 1.5);
        assert_eq!(ema.apply(3.0), 2.25);
        assert_eq!(ema.apply(4.0), 3.125);
        assert_eq!(ema.evaluate(5.0), 4.0625);
        assert_eq!(ema.apply(5.0), 4.0625);
    }

    #[test]
    fn test_current() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();
        assert_eq!(ema.apply(1.0), 1.0);
        assert_eq!(ema.apply(2.0), 1.5);
        assert_eq!(ema.apply(3.0), 2.25);
        assert_eq!(ema.apply(4.0), 3.125);
        assert_eq!(ema.current(), 3.125);
    }

    #[test]
    fn test_invalid_period() {
        let ema = ExponentialMovingAverage::new(0);
        assert!(ema.is_err());
    }

    #[test]
    fn zero_ema_input() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();
        assert_eq!(ema.apply(0.0), 0.0);
        assert_eq!(ema.apply(0.0), 0.0);
        assert_eq!(ema.apply(0.0), 0.0);
    }
}
