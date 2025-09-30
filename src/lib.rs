use pyo3::prelude::*;

#[cfg(feature = "uuid")]
use rusthonian_uuid;

/// Rusthonian - High-performance Python bindings for Rust crates
/// 
/// This module provides access to various Rust crate functionality through
/// efficient PyO3 bindings.
/// 
/// Available submodules (when compiled with corresponding features):
/// - uuid: Complete bindings for the Rust uuid crate
#[pymodule]
fn rusthonian(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "High-performance Python bindings for Rust crates")?;
    m.add("__author__", "Rusthonian Team")?;

    #[cfg(feature = "uuid")]
    {
        let uuid_module = PyModule::new_bound(py, "uuid")?;
        rusthonian_uuid::setup_uuid_module(py, &uuid_module)?;
        m.add_submodule(&uuid_module)?;
        
        // Make uuid available as Rusthonian.uuid
        py.import_bound("sys")?
            .getattr("modules")?
            .set_item("Rusthonian.uuid", &uuid_module)?;
    }

    Ok(())
}