use grex::{Feature, RegExpBuilder};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn build_regex(strings: Vec<String>, character_classes: bool) -> String {
    if character_classes {
        grex::RegExpBuilder::from(&strings).build()
    } else {
        RegExpBuilder::from(&strings)
        .with_conversion_of(&[Feature::Digit, Feature::Word])
        .build()
    }
}

#[pymodule]
fn pygrex(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(build_regex))?;

    Ok(())
}