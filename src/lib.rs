use pyo3::prelude::*;

/// Main Rusthonian module - super project for various Rust crate bindings
/// This module links to submodules like UUID, etc.
#[pymodule]
fn Rusthonian(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add UUID submodule if available
    #[cfg(feature = "uuid")]
    {
        let uuid_module = PyModule::new(_py, "UUID")?;
        rusthonian_uuid::setup_uuid_module(_py, uuid_module)?;
        m.add_submodule(uuid_module)?;
    }
    
    // Add metadata
    m.add("__version__", "0.1.0")?;
    m.add("__description__", "Python bindings for Rust crates via Rusthonian")?;
    
    Ok(())
}