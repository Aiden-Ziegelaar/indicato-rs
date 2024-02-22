use local_macros::{Apply, Evaluate};

use crate::{
    error::{FinError, FinErrorType},
    traits::{Apply, Evaluate, Current, Executable, ExecutionContext, IoState},
};

fn up_down(input: f64, previous: f64) -> (f64, f64) {
    match input > previous {
        true => (input - previous, 0.0),
        false => (0.0, previous - input),
    }
}

/// # Relative Strength Index
/// Container for Relative Strength Index (RSI) aggregation
/// The relative strength index (RSI) is a momentum indicator used in technical analysis that measures the magnitude 
/// of recent price changes to evaluate overbought or oversold conditions in the price of a stock or other asset.
/// 
/// The RSI is displayed as an oscillator (a line graph that moves between two extremes) and can have a reading from 0 to 100.
/// 
/// The RSI is calculated on trends, in order to smooth these trends the RSI is calculated using the Wilders Smoothing method.
/// Two Wilders Smoothing aggregations are used to calculate the average of the upward price change and the average of the downward price change.
/// <br>
/// <br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
///     <mrow>
///         <msub>
///             <mi>U</mi>
///             <mi>n</mi>
///         </msub>
///         <mi>,</mi>
///         <msub>
///             <mi>D</mi>
///             <mi>n</mi>
///         </msub>
///         <mo>=</mo>
///         <mo>{</mo>
///         <mtable>
///             <mtr>
///                 <mtd><mrow>
///                     <msub>
///                         <mi>WS</mi>
///                         <mi>U</mi>
///                     </msub>
///                     <mo>(</mo>
///                     <mrow>
///                         <mo>|</mo>
///                         <mi>Δ</mi>
///                         <msub>
///                             <mi>i</mi>
///                             <mi>n</mi>
///                         </msub>
///                         <mo>|</mo>
///                     </mrow>
///                     <mo>)</mo>
///                     <mo>,</mo>
///                     <msub>
///                         <mi>WS</mi>
///                         <mi>D</mi>
///                     </msub>
///                     <mo>(</mo>
///                     <mn>0</mn>
///                     <mo>)</mo>
///                 </mrow></mtd>
///                 <mtd>if</mtd>
///                 <mtd>
///                     <mi>Δ</mi>
///                     <msub>
///                         <mi>i</mi>
///                         <mi>n</mi>
///                     </msub> ≥ 0
///                 </mtd>
///             </mtr>
///             <mtr>
///                 <mtd><mrow>
///                     <msub>
///                         <mi>WS</mi>
///                         <mi>U</mi>
///                     </msub>
///                     <mo>(</mo>
///                     <mn>0</mn>
///                     <mo>)</mo>
///                     <mo>,</mo>
///                     <msub>
///                         <mi>WS</mi>
///                         <mi>D</mi>
///                     </msub>
///                     <mo>(</mo>
///                     <mrow>
///                         <mo>|</mo>
///                         <mi>Δ</mi>
///                         <msub>
///                             <mi>i</mi>
///                             <mi>n</mi>
///                         </msub>
///                         <mo>|</mo>
///                     </mrow>
///                     <mo>)</mo>
///                 </mrow></mtd>
///                 <mtd>if</mtd>
///                 <mtd>
///                     <mi>Δ</mi>
///                     <msub>
///                         <mi>i</mi>
///                         <mi>n</mi>
///                     </msub> < 0
///                 </mtd>
///             </mtr>
///         </mtable>
///     </mrow>
/// </semantics>
/// </math>
/// 
/// Where `U` is the average of the upward price change, `D` is the average of the downward price change, 
/// `WS` is the Wilders Smoothing aggregation, `Δi` is the difference between the current and previous step input and `n` is the step.
/// 
/// Given:
/// <br>
/// <br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
///     <mrow>
///         <mi>Δ</mi>
///         <msub>
///             <mi>i</mi>
///             <mi>n</mi>
///         </msub>
///         <mo>=</mo>
///         <msub>
///             <mi>i</mi>
///             <mi>n</mi>
///         </msub>
///         <mo>−</mo>
///         <msub>
///             <mi>i</mi>
///             <mrow>
///                 <mi>n</mi>
///                 <mo>−</mo>
///                 <mn>1</mn>
///             </mrow>
///         </msub>
///     </mrow>
/// </semantics>
/// </math>
/// <br>
/// 
/// 
/// The RSI is calculated using the following formula:
/// <br>
/// <br>
/// <math display="block" style="font-size: 20px;">
/// <semantics>
///     <mrow>
///     <msub>
///         <mi>o</mi>
///         <mi>n</mi>
///     </msub>
///     <mo>=</mo>
///     <mn>100</mn>
///     <mo>−</mo>
///     <mfrac>
///         <mn>100</mn>
///         <mrow>
///             <mn>1</mn>
///             <mo>+</mo>
///             <mfrac>
///                 <msub>
///                     <mi>U</mi>
///                     <mi>n</mi>
///                 </msub>
///                 <msub>
///                     <mi>D</mi>
///                     <mi>n</mi>
///                 </msub>
///            </mfrac>
///        </mrow>
///     </mfrac>
///     </mrow>
/// </semantics>
/// </math>
/// <br>
/// Where `o` is the RSI output, `n` is the current step, `U` is the average of the upward price change, `D` is the average of the downward price change.
/// 
/// # Example Usage
/// ```
/// use indicato_rs::signals::RelativeStrengthIndex;
/// use indicato_rs::traits::{Apply, Evaluate, Current};
/// 
/// let mut rsi = RelativeStrengthIndex::new(3, 0).unwrap();
/// 
/// // apply some values and check their output
/// assert_eq!(rsi.apply(0.0), None);
/// assert_eq!(rsi.apply(1.0), None);
/// assert_eq!(rsi.apply(2.0), None);
/// assert_eq!(rsi.apply(3.0), Some(100.0));
/// assert_eq!(rsi.apply(4.0), Some(100.0));
/// 
/// // evaluate the RSI
/// assert_eq!(rsi.evaluate(5.0), Some(100.0));
/// assert_eq!(rsi.evaluate(5.0), Some(100.0));
/// 
/// // check the current RSI
/// assert_eq!(rsi.current(), Some(100.0));
/// ```

#[derive(Apply, Evaluate)]
pub struct RelativeStrengthIndex {
    /// The period of the RSI, used for the Wilders Smoothing aggregations.
    pub period: usize,
    /// Even though the RSI is available from the first value after the period parameter, additional values 
    /// can be used to seed the RSI. This is added to the period to prevent values from being produced until
    /// `period` + `seed_period` values have been applied.
    pub seed_period: usize,
    /// The Wilders Smoothing aggregation for the upward price change.
    up_ws: super::WildersSmoothing,
    // The Wilders Smoothing aggregation for the downward price change.
    down_ws: super::WildersSmoothing,
    /// Whether the RSI has been seeded.
    is_seeded: bool,
    /// The number of values that have been applied to the RSI.
    seed_values: usize,
    /// The previous input value.
    previous: Option<f64>,
}

impl IoState for RelativeStrengthIndex {
    type Input = f64;
    type Output = Option<f64>;
}

impl RelativeStrengthIndex {
    /// Creates a new RelativeStrengthIndex aggregation.
    /// 
    /// # Arguments
    /// * `period` - The period of the RSI, used for the Wilders Smoothing aggregations.
    /// * `seed_period` - The number of values that must be applied beyond the period to the RSI before it produces values.
    /// 
    pub fn new(period: usize, seed_period: usize) -> Result<Self, FinError> {
        match period {
            0 => Err(FinError::new(
                FinErrorType::InvalidInput,
                "Period must be greater than 0",
            )),
            _ => Ok(Self {
                period,
                seed_period: period + seed_period,
                up_ws: super::WildersSmoothing::new(period)?,
                down_ws: super::WildersSmoothing::new(period)?,
                is_seeded: false,
                seed_values: 0,
                previous: None,
            }),
        }
    }
}

impl Executable for RelativeStrengthIndex {
    fn execute(
        &mut self,
        input: Self::Input,
        execution_context: &ExecutionContext,
    ) -> Self::Output {
        let previous = match self.previous {
            None => {
                self.previous = Some(input);
                self.seed_values += 1;
                return None;
            }
            Some(previous) => previous,
        };
        let (up, down) = up_down(input, previous);
        let up_ws = self.up_ws.execute(up, execution_context);
        let down_ws = self.down_ws.execute(down, execution_context);
        if !self.is_seeded {
            match execution_context {
                ExecutionContext::Apply => {
                    self.seed_values += 1;
                    if self.seed_values == self.seed_period {
                        self.is_seeded = true;
                    }
                    self.previous = Some(input);
                }
                ExecutionContext::Evaluate => {}
            }
            return None;
        }
        if down_ws == Some(0.0) {
            return Some(100.0);
        }

        match (up_ws, down_ws) {
            (Some(up_ws), Some(down_ws)) => {
                let rs = up_ws / down_ws;
                let rsi = 100.0 - (100.0 / (1.0 + rs));
                match execution_context {
                    ExecutionContext::Apply => {
                        self.previous = Some(input);
                    }
                    ExecutionContext::Evaluate => {}
                }
                Some(rsi)
            }
            _ => None,
        }
    }
}


impl Current for RelativeStrengthIndex {
    fn current(&self) -> Self::Output {
        if self.is_seeded {
            match (self.up_ws.current(), self.down_ws.current()) {
                (Some(up_ws), Some(down_ws)) => {
                    let rs = up_ws / down_ws;
                    let rsi = 100.0 - (100.0 / (1.0 + rs));
                    Some(rsi)
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let mut rsi = RelativeStrengthIndex::new(3, 0).unwrap();
        assert_eq!(rsi.apply(0.0), None);
        assert_eq!(rsi.apply(1.0), None);
        assert_eq!(rsi.apply(2.0), None);
        assert_eq!(rsi.apply(3.0), Some(100.0));
        assert_eq!(rsi.apply(4.0), Some(100.0));
        assert_eq!(rsi.apply(5.0), Some(100.0));
    }

    #[test]
    fn test_evaluate() {
        let mut rsi = RelativeStrengthIndex::new(3, 0).unwrap();
        assert_eq!(rsi.apply(0.0), None);
        assert_eq!(rsi.apply(1.0), None);
        assert_eq!(rsi.apply(2.0), None);
        assert_eq!(rsi.apply(3.0), Some(100.0));
        assert_eq!(rsi.apply(4.0), Some(100.0));
        assert_eq!(rsi.evaluate(5.0), Some(100.0));
        assert_eq!(rsi.apply(5.0), Some(100.0));
    }

    #[test]
    fn test_invalid_period() {
        let rsi = RelativeStrengthIndex::new(0, 3);
        assert!(rsi.is_err());
    }

    #[test]
    fn test_rsi_data() {
        let mut rsi = RelativeStrengthIndex::new(14, 0).unwrap();
        rsi.apply(10.92521440760443900);
        rsi.apply(10.19859579534958400);
        rsi.apply(10.67283651362573500);
        rsi.apply(10.59985028709600800);
        rsi.apply(10.92213316907769000);
        rsi.apply(10.21930613382197500);
        rsi.apply(10.97345881837492400);
        rsi.apply(10.52359275836161700);
        rsi.apply(10.84870000849940300);
        rsi.apply(10.47114753347496000);
        rsi.apply(10.50194664759466100);
        rsi.apply(10.56933881368713200);
        rsi.apply(10.30682386665992900);
        rsi.apply(10.93831484940749100);
        assert_eq!(rsi.apply(10.11768126451183100), Some(43.29120317120137));
        assert_eq!(rsi.apply(10.11768126451183100), Some(43.29120317120137));
        assert_eq!(rsi.apply(10.11768126451183100), Some(43.291203171201374));
        assert_eq!(rsi.apply(10.11768126451183100), Some(43.291203171201374));
        assert_eq!(rsi.apply(10.11768126451183100), Some(43.291203171201374));
        assert_eq!(rsi.apply(10.93831484940749100), Some(52.644368580828655));
    }
}
