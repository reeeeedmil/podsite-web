mod calculations;
mod menu;
mod tests;

use crate::calculations::*;
use crate::menu::*;

use pyo3::prelude::*;

#[pymodule]
fn rsnet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Address>()?;
    m.add_class::<Net>()?;
    m.add_wrapped(wrap_pyfunction!(init_menu)).unwrap();
    m.add_wrapped(wrap_pyfunction!(scaffold_prefixes)).unwrap();
    m.add_wrapped(wrap_pyfunction!(scaffold_hosts)).unwrap();
    Ok(())
}
