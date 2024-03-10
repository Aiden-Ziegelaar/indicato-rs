use std::collections::VecDeque;

use indicato_rs_proc::{Apply, Evaluate};

use crate::{
    deque_math::DequeMathExtF64, fin_error::{FinError, FinErrorType}, traits::{Apply, Current, Evaluate, Executable, ExecutionContext, IoState}
};

/// # Maximum Period
///
/// The maximum period signal is a signal that calculates the maximum value of a given period.
///
/// The aggregation will begin producing values immediately, the first value will be the input, after which the following formula is applied:
/// <br>
/// <br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
///     <mrow>
///     <mtable><mtr><mtd>
///         <msub>
///             <mi>o</mi>
///             <mn>n</mn>
///         </msub>
///         <mo>=</mo>
///         <msub>
///             <mi>i</mi>
///             <mn>max</mn>
///         </msub>
///     </mtd>
///     <mtd>
///         <mn>where</mn>
///     </mtd>
///     <mtd>
///         <msub>
///             <mi>i</mi>
///             <mn>max</mn>
///         </msub>
///         <mo>≥</mo>
///         <mo>∀</mo>
///         <msub>
///             <mi>i</mi>
///             <mn>k</mn>
///         </msub>
///     </mtd>
///     <mtd>
///         <mn>where</mn>
///     </mtd>
///     <mtd>
///         <mi>k</mi>
///         <mo>∈</mo>
///         <mo>{</mo>
///             <mrow><mn>H(n-p)</mn><mo>⋅</mo><mo>(</mo><mn>n-p</mn><mo>)</mo></mrow>
///             <mo>..</mo>
///             <mn>n</mn>
///         <mo>}</mo>
///     </mtd></mtr></mtable>
///     </mrow>
/// </semantics>
/// </math>
/// <br>
/// Where `o` is the output, `n` is the current step, `p` is the period, `H` is the Heaviside function, and `i` is the input.
/// # Example Usage
/// ```
/// use indicato_rs::signals::MaximumPeriod;
/// use indicato_rs::traits::{Apply, Evaluate, Current};
///
/// // Create a new MaximumPeriod signal with a period of 3
/// let mut max = MaximumPeriod::new(3).unwrap();
///
/// // Apply some values and check their output
/// assert_eq!(max.apply(1.0), 1.0);
/// assert_eq!(max.apply(2.0), 2.0);
/// assert_eq!(max.apply(3.0), 3.0);
/// assert_eq!(max.apply(2.0), 3.0);
/// assert_eq!(max.apply(1.0), 3.0);
/// assert_eq!(max.apply(0.0), 2.0);
///
/// // Evaluate some values, these won't affect the internal state of the MaximumPeriod
/// assert_eq!(max.evaluate(5.0), 5.0);
/// assert_eq!(max.evaluate(4.0), 4.0);
///
/// // Fetch the current value of the MaximumPeriod
/// assert_eq!(max.current(), 2.0);
/// ```
#[derive(Apply, Evaluate)]
pub struct MaximumPeriod {
    period: usize,
    values: VecDeque<f64>,
}

impl MaximumPeriod {
    pub fn new(period: usize) -> Result<Self, FinError> {
        match period {
            0 => Err(FinError::new(
                FinErrorType::InvalidInput,
                "Period must be greater than 0",
            )),
            _ => Ok(Self {
                period,
                values: VecDeque::with_capacity(period),
            }),
        }
    }
}

impl IoState for MaximumPeriod {
    type Input = f64;
    type Output = f64;
}

impl Executable for MaximumPeriod {
    fn execute(
        &mut self,
        input: Self::Input,
        execution_context: &ExecutionContext,
    ) -> Self::Output {
        match execution_context {
            ExecutionContext::Apply => {
                self.values.push_back(input);
                if self.values.len() > self.period {
                    self.values.pop_front();
                }
                self.values.max()
            }
            ExecutionContext::Evaluate => self
                .values
                .iter()
                .skip(1)
                .fold(f64::MIN, |acc, &x| acc.max(x))
                .max(input),
        }
    }
}

impl Current for MaximumPeriod {
    fn current(&self) -> Self::Output {
        self.values.max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_period_apply() {
        let mut max = MaximumPeriod::new(3).unwrap();
        assert_eq!(max.apply(1.0), 1.0);
        assert_eq!(max.apply(2.0), 2.0);
        assert_eq!(max.apply(3.0), 3.0);
        assert_eq!(max.apply(2.0), 3.0);
        assert_eq!(max.apply(1.0), 3.0);
        assert_eq!(max.apply(0.0), 2.0);
    }

    #[test]
    fn test_maximum_period_evaluate() {
        let mut max = MaximumPeriod::new(3).unwrap();
        assert_eq!(max.apply(1.0), 1.0);
        assert_eq!(max.apply(2.0), 2.0);
        assert_eq!(max.apply(3.0), 3.0);
        assert_eq!(max.evaluate(5.0), 5.0);
        assert_eq!(max.apply(2.0), 3.0);
        assert_eq!(max.apply(1.0), 3.0);
        assert_eq!(max.apply(0.0), 2.0);
        assert_eq!(max.evaluate(0.5), 1.0);
    }

    #[test]
    fn test_maximum_period_current() {
        let mut max = MaximumPeriod::new(3).unwrap();
        assert_eq!(max.apply(1.0), 1.0);
        assert_eq!(max.apply(2.0), 2.0);
        assert_eq!(max.apply(3.0), 3.0);
        assert_eq!(max.apply(2.0), 3.0);
        assert_eq!(max.apply(1.0), 3.0);
        assert_eq!(max.apply(0.0), 2.0);
        assert_eq!(max.current(), 2.0);
    }

    #[test]
    fn test_invalid_period() {
        let max = MaximumPeriod::new(0);
        assert!(max.is_err());
    }
}
