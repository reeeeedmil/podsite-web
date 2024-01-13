mod calculations;
mod menu;
mod tests;

pub use crate::calculations::*;
pub use crate::menu::*;

use pyo3::prelude::*;

#[pymodule]
fn rsnet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Address>()?;
    m.add_class::<Net>()?;
    m.add_wrapped(wrap_pyfunction!(init_menu)).unwrap();
    Ok(())
}
