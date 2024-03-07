use indicato_rs_proc::{Apply, Evaluate};

use crate::{
    error::{FinError, FinErrorType},
    traits::{Apply, Current, Evaluate, Executable, ExecutionContext, IoState},
};

fn calculate_wilders(input: f64, previous: f64, period: usize) -> f64 {
    (previous * (period as f64 - 1.0) + input) / period as f64
}

/// # Wilders Smoothing
/// Container for Wilders Smoothing aggregation
///
/// Formula applied:
/// <br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
///     <mrow>
///         <msub>
///             <mi>o</mi>
///             <mn>n</mn>
///         </msub>
///         <mo>=</mo>
///         <mfrac>
///             <mrow>
///                 <msub>
///                     <mi>o</mi>
///                     <mn>n-1</mn>
///                 </msub><mo>⋅</mo>
///                 <mrow><mo stretchy="true" form="prefix">(</mo>
///                     <mi>p</mi><mo>-</mo><mn>1</mn>
///                 <mo stretchy="true" form="postfix">)</mo></mrow>
///                 <mo>+</mo>
///                 <msub>
///                     <mi>i</mi>
///                     <mi>n</mi>
///                 </msub>
///             </mrow>
///             <mrow>
///                 <mi>p</mi>
///             </mrow>
///         </mfrac>
///     </mrow>
/// </semantics>
/// </math>
/// <br>
///
/// Where `o` is the output, `n` is the current step, `n-1` is the previous step, `p` is the period of the Wilders Smoothing and `i` is the input.
///
/// The first entries up until the period will produce `None` as the output, as the aggregation is being seeded.
/// Once the aggregation is seeded the first output will be the average of the first `period` entries.
/// The first value is calculated using the formula:
/// <br><br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
///     <mrow>
///         <msub>
///             <mi>o</mi>
///             <mi>p</mi>
///         </msub>
///         <mo>=</mo>
///         <mfrac>
///             <mrow>
///                 <munderover>
///                     <mo>∑</mo>
///                     <mi>n=0</mi>
///                     <mi>p</mi>
///                 </munderover>
///                 <msub>
///                     <mi>i</mi>
///                     <mi>n</mi>
///                 </msub>
///             </mrow>
///             <mrow>
///                 <mi>p</mi>
///             </mrow>
///         </mfrac>
///     </mrow>
/// </semantics>
/// </math>
/// <br>
///
/// # Example Usage
/// ```
/// use indicato_rs::signals::WildersSmoothing;
/// use indicato_rs::traits::{Apply, Evaluate, Current};
///
/// // create a new Wilders Smoothing with a period of 3
/// let mut ws = WildersSmoothing::new(3).unwrap();
///
/// // apply some values and check their output
/// assert_eq!(ws.apply(2.0), None);
/// assert_eq!(ws.apply(4.0), None);
/// assert_eq!(ws.apply(3.0), Some(3.0));
/// assert_eq!(ws.apply(9.0), Some(5.0));
///
/// // evaluate some values, these won't affect the internal state of the Wilders Smoothing
/// assert_eq!(ws.evaluate(8.0), Some(6.0));
/// assert_eq!(ws.evaluate(2.0), Some(4.0));
///
/// // check the current value of the Wilders Smoothing
/// assert_eq!(ws.current(), Some(5.0));
/// ```
#[derive(Apply, Evaluate)]
pub struct WildersSmoothing {
    /// The period of the Wilders Smoothing aggregation
    period: usize,
    current: f64,
    previous: f64,
    seed_count: usize,
}

impl IoState for WildersSmoothing {
    type Input = f64;
    type Output = Option<f64>;
}

impl WildersSmoothing {
    /// Create a new WildersSmoothing instance
    /// # Arguments
    /// * `period` - The period of the Wilders Smoothing aggregation, must be greater than 0
    ///
    /// # Example
    /// ```
    /// use indicato_rs::signals::WildersSmoothing;
    ///
    /// let ws = WildersSmoothing::new(3);
    ///
    /// assert!(ws.is_ok());
    /// ```
    /// # Errors
    /// Will return an error if the period is 0
    /// ```
    /// use indicato_rs::signals::WildersSmoothing;
    ///
    /// let ws = WildersSmoothing::new(0);
    ///
    /// assert!(ws.is_err());
    /// ```
    pub fn new(period: usize) -> Result<Self, FinError> {
        match period {
            0 => Err(FinError::new(
                FinErrorType::InvalidInput,
                "Period must be greater than 0",
            )),
            _ => Ok(Self {
                period,
                previous: 0.0,
                current: 0.0,
                seed_count: 1,
            }),
        }
    }
}

impl Executable for WildersSmoothing {
    fn execute(
        &mut self,
        input: Self::Input,
        execution_context: &ExecutionContext,
    ) -> Self::Output {
        match execution_context {
            ExecutionContext::Apply => {
                if self.seed_count < self.period {
                    self.current += input;
                    self.previous = self.current / self.seed_count as f64;
                    self.seed_count += 1;
                    None
                } else {
                    self.current = calculate_wilders(input, self.previous, self.period);
                    self.previous = self.current;
                    Some(self.current)
                }
            }
            ExecutionContext::Evaluate => {
                if self.seed_count < self.period {
                    None
                } else {
                    let current = calculate_wilders(input, self.previous, self.period);
                    Some(current)
                }
            }
        }
    }
}

impl Current for WildersSmoothing {
    fn current(&self) -> Self::Output {
        if self.seed_count < self.period {
            None
        } else {
            Some(self.current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let mut ws = WildersSmoothing::new(3).unwrap();
        assert_eq!(ws.apply(1.0), None);
        assert_eq!(ws.apply(2.0), None);
        assert_eq!(ws.apply(3.0), Some(2.0));
        assert_eq!(ws.apply(2.0), Some(2.0));
        assert_eq!(ws.apply(5.0), Some(3.0));
    }

    #[test]
    fn test_evaluate() {
        let mut ws = WildersSmoothing::new(3).unwrap();
        assert_eq!(ws.apply(1.0), None);
        assert_eq!(ws.apply(2.0), None);
        assert_eq!(ws.apply(3.0), Some(2.0));
        assert_eq!(ws.apply(2.0), Some(2.0));
        assert_eq!(ws.evaluate(5.0), Some(3.0));
        assert_eq!(ws.apply(5.0), Some(3.0));
    }
}
