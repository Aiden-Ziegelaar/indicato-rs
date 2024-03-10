use std::collections::VecDeque;

use crate::{
    fin_error::{FinError, FinErrorType},
    traits::{Apply, Current, Evaluate},
};
use indicato_rs_proc::{Apply, Evaluate};

use crate::traits::{Executable, ExecutionContext, IoState};

fn calculate_sma(input: f64, period: usize, values: &mut VecDeque<f64>) -> f64 {
    values.push_back(input);
    if values.len() > period {
        values.pop_front();
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// # Simple Moving Average
/// Container for Simple Moving Average (SMA) aggregation
///
/// Formula applied:
/// <br><br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
/// <mrow>
/// <msub>
///     <mi>o</mi>
///     <mn>n</mn>
/// </msub>
/// <mo>=</mo>
/// <mo>{</mo>
/// <mtable>
///     <mtr>
///     <mtd><mrow>
///     <mfrac>
///         <mrow>
///         <munderover>
///             <mo>∑</mo>
///             <mi>n=0</mi>
///             <mi>n</mi>
///         </munderover>
///         <msub>
///             <mi>i</mi>
///             <mn>n</mn>
///         </msub>
///         </mrow>
///         <mi>n</mi>
///     </mfrac>
///     </mrow></mtd>
///     <mtd>if</mtd>
///     <mtd><mrow><mi>n</mi><mo><</mo><mi>p</mi></mrow></mtd>
///     </mtr>
///     <mtr>
///     <mtd><mrow>
///         <mfrac>
///             <mrow>
///                 <munderover>
///                     <mo>∑</mo>
///                     <mi>n=n-p</mi>
///                     <mi>n</mi>
///                 </munderover>
///                 <msub>
///                     <mi>i</mi>
///                     <mn>n</mn>
///                 </msub>
///             </mrow>
///             <mi>p</mi>
///         </mfrac>
///     </mrow></mtd>
///     <mtd>if</mtd>
///     <mtd><mrow><mi>n</mi><mo>≥</mo><mi>p</mi></mrow></mtd>
///     </mtr>
/// </mtable>
/// </mrow>
/// </semantics>
/// </math>
/// <br><br>
/// Where `o` is the output, `n` is the current step, `n-1` is the previous step, `p` is the period of the simple moving average and `i` is the input.
///
/// # Example Usage
/// ```
/// use indicato_rs::signals::SimpleMovingAverage;
/// use indicato_rs::traits::{Apply, Evaluate, Current};
///
/// // create a new Simple Moving Average with a period of 3
/// let mut sma = SimpleMovingAverage::new(3).unwrap();
///
/// // apply some values and check their output
/// assert_eq!(sma.apply(1.0), 1.0);
/// assert_eq!(sma.apply(2.0), 1.5);
/// assert_eq!(sma.apply(3.0), 2.0);
/// assert_eq!(sma.apply(4.0), 3.0);
///
/// // evaluate some values, these won't affect the internal state of the SMA
/// assert_eq!(sma.evaluate(5.0), 4.0);
/// assert_eq!(sma.evaluate(8.0), 5.0);
///
/// // fetch the current value of the EMA
/// assert_eq!(sma.current(), 3.0);
/// ````
///
#[derive(Apply, Evaluate)]
pub struct SimpleMovingAverage {
    period: usize,
    values: VecDeque<f64>,
}

impl IoState for SimpleMovingAverage {
    type Input = f64;
    type Output = f64;
}

impl SimpleMovingAverage {
    /// Create a new Simple Moving Average instance
    /// # Arguments
    /// * `period` - The period of the Simple Moving Average aggregation, must be greater than 0
    ///
    /// # Example
    /// ```
    /// use indicato_rs::signals::SimpleMovingAverage;
    ///
    /// let sma = SimpleMovingAverage::new(3);
    ///
    /// assert!(sma.is_ok());
    /// ```
    /// # Errors
    /// Will return an error if the period is 0
    /// ```
    /// use indicato_rs::signals::SimpleMovingAverage;
    ///
    /// let sma = SimpleMovingAverage::new(0);
    ///
    /// assert!(sma.is_err());
    /// ```
    pub fn new(period: usize) -> Result<Self, FinError> {
        match period {
            0 => Err(FinError::new(
                FinErrorType::InvalidInput,
                "Period must be greater than 0",
            )),
            _ => Ok(Self {
                period,
                values: VecDeque::with_capacity(period + 1),
            }),
        }
    }
}

impl Executable for SimpleMovingAverage {
    fn execute(
        &mut self,
        input: Self::Input,
        execution_context: &ExecutionContext,
    ) -> Self::Output {
        match execution_context {
            ExecutionContext::Apply => calculate_sma(input, self.period, &mut self.values),
            ExecutionContext::Evaluate => {
                let mut values = self.values.clone();
                calculate_sma(input, self.period, &mut values)
            }
        }
    }
}

impl Current for SimpleMovingAverage {
    fn current(&self) -> Self::Output {
        if self.values.is_empty() {
            0.0
        } else {
            self.values.iter().sum::<f64>() / self.values.len() as f64
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let mut sma = SimpleMovingAverage::new(3).unwrap();
        assert_eq!(sma.apply(1.0), 1.0);
        assert_eq!(sma.apply(2.0), 1.5);
        assert_eq!(sma.apply(3.0), 2.0);
        assert_eq!(sma.apply(4.0), 3.0);
        assert_eq!(sma.apply(5.0), 4.0);
    }

    #[test]
    fn test_evaluate() {
        let mut sma = SimpleMovingAverage::new(3).unwrap();
        assert_eq!(sma.apply(1.0), 1.0);
        assert_eq!(sma.apply(2.0), 1.5);
        assert_eq!(sma.apply(3.0), 2.0);
        assert_eq!(sma.apply(4.0), 3.0);
        assert_eq!(sma.evaluate(5.0), 4.0);
        assert_eq!(sma.apply(5.0), 4.0);
    }

    #[test]
    fn test_current() {
        let mut sma = SimpleMovingAverage::new(3).unwrap();
        assert_eq!(sma.apply(1.0), 1.0);
        assert_eq!(sma.apply(2.0), 1.5);
        assert_eq!(sma.apply(3.0), 2.0);
        assert_eq!(sma.apply(4.0), 3.0);
        assert_eq!(sma.current(), 3.0);
    }

    #[test]
    fn test_invalid_period() {
        let sma = SimpleMovingAverage::new(0);
        assert!(sma.is_err());
    }
}
