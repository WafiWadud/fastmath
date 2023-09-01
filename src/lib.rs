use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn add(numbers: Vec<f64>) -> f64 {
    numbers.into_iter().sum()
}

#[pyfunction]
pub fn multiply(numbers: Vec<f64>) -> f64 {
    numbers.into_iter().product()
}

#[pyfunction]
pub fn subtract(mut numbers: Vec<f64>) -> Option<f64> {
    let first: f64 = numbers.remove(0);
    Some(numbers.into_iter().fold(first, |acc: f64, v: f64| acc - v))
}

#[pyfunction]
pub fn divide(mut numbers: Vec<f64>) -> Option<f64> {
    let first: f64 = numbers.remove(0);
    numbers.into_iter().try_fold(
        first,
        |acc: f64, v: f64| if v != 0.0 { Some(acc / v) } else { None },
    )
}
#[pyfunction]
pub fn pow(base: f64, exponent: f64) -> f64 {
    let e = exponent.floor();
    let frac = exponent - e;
    let mut result = 1.0;
    let mut b = base;
    let mut i = e as i64;

    while i > 0 {
        if i % 2 == 1 {
            result *= b;
        }
        b *= b;
        i /= 2;
    }

    if frac > 0.0 {
        let root = 1.0 / frac;
        let mut x = base;
        let mut y = result;
        let epsilon = 0.00001;

        while (y - x).abs() > epsilon {
            y = x;
            x = (1.0 / root) * ((root - 1.0) * y + base / y.powf(root - 1.0));
        }

        result *= x;
    }

    result
}
#[pymodule]
fn fastmathpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(subtract, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_function(wrap_pyfunction!(pow, m)?)?;
    Ok(())
}
