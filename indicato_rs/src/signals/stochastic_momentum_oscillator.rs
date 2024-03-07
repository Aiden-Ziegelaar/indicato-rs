use indicato_rs_proc::{Apply, Evaluate};

use crate::{
    error::FinError,
    traits::{Apply, Current, Evaluate, Executable, ExecutionContext, IoState},
};

use super::{MaximumPeriod, MinimumPeriod};

/// # Stochastic Momentum Oscillator
///
/// The Stochastic Momentum Oscillator (SMO) is a signal that calculates the momentum of a given period.
///
/// The aggregation will begin producing values immediately, the following formula is applied:
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
///         <mfrac>
///             <mrow>
///                 <msub>
///                     <mi>c</mi>
///                     <mn>n</mn>
///                 </msub>
///                 <mo>-</mo>
///                 <msub>
///                     <mi>l</mi>
///                     <mn>min</mn>
///                 </msub>
///             </mrow>
///             <mrow>
///                 <msub>
///                     <mi>h</mi>
///                     <mn>max</mn>
///                 </msub>
///                 <mo>-</mo>
///                 <msub>
///                     <mi>l</mi>
///                     <mn>min</mn>
///                 </msub>
///             </mrow>
///         </mfrac>
///     </mtd>
///     <mtd>
///         <mn>where</mn>
///     </mtd>
///     <mtd>
///         <msub>
///             <mi>h</mi>
///             <mn>max</mn>
///         </msub>
///         <mo>≥</mo>
///         <mo>∀</mo>
///         <msub>
///             <mi>h</mi>
///             <mn>k</mn>
///         </msub>
///     </mtd>
///     <mtd>
///         <mn>and</mn>
///     </mtd>
///     <mtd>
///         <msub>
///             <mi>l</mi>
///             <mn>min</mn>
///         </msub>
///         <mo>≤</mo>
///         <mo>∀</mo>
///         <msub>
///             <mi>l</mi>
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
/// Where `o` is the output, `n` is the current step, `c` is the close value, `p` is the period, `H` is the Heaviside function, `h` is the high value, and `l` is the low value.
#[derive(Apply, Evaluate)]
pub struct StochasticMomentumOscillator {
    high: MaximumPeriod,
    low: MinimumPeriod,
    current: f64,
}

impl StochasticMomentumOscillator {
    pub fn new(period: usize) -> Result<Self, FinError> {
        Ok(Self {
            high: MaximumPeriod::new(period)?,
            low: MinimumPeriod::new(period)?,
            current: 50.0,
        })
    }
}

impl IoState for StochasticMomentumOscillator {
    type Input = (f64, f64, f64);
    type Output = f64;
}

impl Executable for StochasticMomentumOscillator {
    fn execute(
        &mut self,
        input: Self::Input,
        execution_context: &ExecutionContext,
    ) -> Self::Output {
        let (high, low, close) = input;
        match execution_context {
            ExecutionContext::Apply => {
                let high = self.high.execute(high, execution_context);
                let low = self.low.execute(low, execution_context);
                if high == low {
                    self.current = 50.0
                } else {
                    self.current = 100.0 * (close - low) / (high - low)
                }
                self.current
            }
            ExecutionContext::Evaluate => {
                let high = self.high.execute(high, execution_context);
                let low = self.low.execute(low, execution_context);
                if high == low {
                    50.0
                } else {
                    100.0 * (close - low) / (high - low)
                }
            }
        }
    }
}

impl Current for StochasticMomentumOscillator {
    fn current(&self) -> Self::Output {
        self.current
    }
}
