use std::collections::VecDeque;

pub trait DequeMathExtF64 {
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
    fn standard_deviation(&self) -> f64;
    fn max(&self) -> f64;
    fn min(&self) -> f64;
}

impl DequeMathExtF64 for VecDeque<f64> {
    fn mean(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        self.iter().sum::<f64>() / self.len() as f64
    }

    fn variance(&self) -> f64 {
        let mean = self.iter().sum::<f64>() / self.len() as f64;
        self.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / self.len() as f64
    }

    fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    fn max(&self) -> f64 {
        self.iter().fold(f64::MIN, |acc, &x| acc.max(x))
    }

    fn min(&self) -> f64 {
        self.iter().fold(f64::MAX, |acc, &x| acc.min(x))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let mut values = VecDeque::new();
        values.push_back(1.0);
        values.push_back(2.0);
        values.push_back(3.0);
        assert_eq!(values.mean(), 2.0);
    }

    #[test]
    fn test_mean_empty() {
        let values = VecDeque::new();
        assert_eq!(values.mean(), 0.0);
    }

    #[test]
    fn test_variance() {
        let mut values = VecDeque::new();
        values.push_back(1.0);
        values.push_back(2.0);
        values.push_back(3.0);
        assert_eq!(values.variance(), 2.0/3.0);
    }

    #[test]
    fn test_standard_deviation() {
        let mut values = VecDeque::new();
        values.push_back(1.0);
        values.push_back(2.0);
        values.push_back(3.0);
        assert_eq!(values.standard_deviation(), (2.0/3.0 as f64).sqrt());
    }

    #[test]
    fn test_max() {
        let mut values = VecDeque::new();
        values.push_back(1.0);
        values.push_back(2.0);
        values.push_back(3.0);
        assert_eq!(values.max(), 3.0);
    }

    #[test]
    fn test_min() {
        let mut values = VecDeque::new();
        values.push_back(2.0);
        values.push_back(1.0);
        values.push_back(3.0);
        assert_eq!(values.min(), 1.0);
    }
}