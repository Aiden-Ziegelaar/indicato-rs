/// Context enum to decided whether to apply or evaluate the signal.
pub enum ExecutionContext {
    /// Next value will be calculated and the currrent input will be applied to the aggregation.
    Apply,
    /// Next value will be calculated but the current input will not be applied to the aggregation.
    Evaluate,
}

/// A trait that specifies the input and output types of all signals, this generalises the
/// application of of the trait definitions allowing for a more flexible and generic approach.
pub trait IoState {
    /// The input type of the signal.
    type Input;
    /// The output type of the signal.
    type Output;
}

/// Evaluates the input and returns the result without applying the value to the aggregation.
pub trait Evaluate: Executable {
    /// Evaluates the input and returns the result without applying the value to the aggregation.
    fn evaluate(&mut self, input: Self::Input) -> Self::Output;
}

/// Applies the input to the aggregation and returns the result.
pub trait Apply: Executable {
    /// Applies the input to the aggregation and returns the result.
    fn apply(&mut self, input: Self::Input) -> Self::Output;
}

/// Returns the current value of the aggregation.
pub trait Current: IoState {
    /// Returns the current value of the aggregation.
    fn current(&self) -> Self::Output;
}

/// A trait for objects that can be executed, either peeking at the prospective result or
/// applying the value to the aggregation and returning the result.
pub trait Executable: IoState {
    /// Executes the signal and returns the result applying the input to the aggregation as described by the `ExecutionContext`.
    fn execute(&mut self, input: Self::Input, execution_context: &ExecutionContext)
        -> Self::Output;
}
