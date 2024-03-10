[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/tokio-rs/tokio/blob/master/LICENSE
[crates-badge]: https://img.shields.io/crates/v/indicato_rs.svg
[crates-url]: https://crates.io/crates/indicato_rs
[codecov-badge]: https://codecov.io/github/Aiden-Ziegelaar/indicato-rs/graph/badge.svg?token=F62M3Z8IA4
[codecov-url]: https://codecov.io/github/Aiden-Ziegelaar/indicato-rs

# DISCLAIMER
This is a personal project I am doing for fun, it is not recommended that you
use these in any project where reliable statistical analysis is required. 
For limitations and license see LICENSE.md in the top level  of this repo.

# Indicato-rs
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![codecov][codecov-badge]][codecov-url]

This is a library built to calculate common statistical signals used in 
Technical Analysis (TA) of markets.

## Signals Currently Implemented:

|Signal                                         | Code  | Tests | Bench |
|-----------------------------------------------|-------|-------|-------|
|Exponential Moving Average (EMA)               |✅|✅|❌|
|Maximum in Period                              |✅|✅|❌|
|Minimum in Period                              |✅|✅|❌|
|Moving Average Convergence Divergence (MACD)   |✅|✅|❌|
|Relative Strength Index (RSI)                  |✅|✅|❌|
|Simple Moving Average (SMA)                    |✅|✅|❌|
|Stochastic Momentum Oscillator                 |✅|✅|❌|
|Wilders Smoothing                              |✅|✅|✅|

## Design Philosophy
The library is designed to make it easy to use a combination of signal primitives to 
get the desired outcome. You'll notice that typically only the signal itself is 
implemented, any additional calculations must be added. A good example of this is 
MACD, typically MACD will have the MACD line, a EMA signal line, and the divergence. 
This library will only return the MACD line, for the signal line you will need an 
additional EMA primitive and it is the responsibility of the consumer to make comparisons.

## Existing Work
Theres a great crate called [ta](https://docs.rs/ta/latest/ta/), this crate was 
partially inspired by it. I mainly started this because I wanted lower level primitives
to work with, I also wanted an easy way to peek at the result of a prospective input
without cloning the whole struct by using an `evaluate` and `apply` mode. 

Where possible I have used [Investopedia](https://www.investopedia.comInvestopedia)'s 
definition of formulas.