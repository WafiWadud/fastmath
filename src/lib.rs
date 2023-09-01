// use PyO3 to interface Rust with Python
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Define a Python function to add numbers.
// Input: a vector of f64 numbers.
// Output: the sum of all numbers.
#[pyfunction]
pub fn add(numbers: Vec<f64>) -> f64 {
    numbers.into_iter().sum()
}

// Define a Python function to multiply numbers.
// Input: a vector of f64 numbers.
// Output: the product of all numbers.
#[pyfunction]
pub fn multiply(numbers: Vec<f64>) -> f64 {
    numbers.into_iter().product()
}

// Define a Python function to subtract numbers.
// Input: a vector of f64 numbers.
// Output: the value resulting from accumulating subtraction operations to the numbers (first number - all the others).
#[pyfunction]
pub fn subtract(mut numbers: Vec<f64>) -> Option<f64> {
    let first: f64 = numbers.remove(0);
    Some(numbers.into_iter().fold(first, |acc: f64, v: f64| acc - v))
}

// Define a Python function to divide numbers.
// Input: a vector of f64 numbers.
// Output: the value resulting from accumulating division operations to the numbers (first number divided by all the others).
// Note: if divide by zero is encountered, the function will not panic but return None. 
#[pyfunction]
pub fn divide(mut numbers: Vec<f64>) -> Option<f64> {
    let first: f64 = numbers.remove(0);
    numbers.into_iter().try_fold(
        first,
        |acc: f64, v: f64| if v != 0.0 { Some(acc / v) } else { None },
    )
}

// Define a Python function that computes the power.
// Input: base number (f64) and the exponent (f64).
// Output: the value representing the base raised to the exponent.
#[pyfunction]
pub fn pow(base: f64, exponent: f64) -> f64 {
    let e: f64 = exponent.floor();
    let frac: f64 = exponent - e;
    let mut result: f64 = 1.0;
    let mut b: f64 = base;
    let mut i: i64 = e as i64;

    // Real number exponentiation (integer part)
    while i > 0 {
        if i % 2 == 1 {
            result *= b;
        }
        b *= b;
        i /= 2;
    }

    // Real number exponentiation (fractional part) using Newton's method.
    if frac > 0.0 {
        let root: f64 = 1.0 / frac;
        let mut x: f64 = base;
        let mut y: f64 = result;
        let epsilon: f64 = 0.00001;

        while (y - x).abs() > epsilon {
            y = x;
            x = (1.0 / root) * ((root - 1.0) * y + base / y.powf(root - 1.0));
        }

        result *= x;
    }

    result
}

// Define a Python module that includes all the above functions.
#[pymodule]
fn fastmathpy(_py: Python, m: &PyModule) -> PyResult<()> {
    // add the above functions into module
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(subtract, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_function(wrap_pyfunction!(pow, m)?)?;

    Ok(())
}
