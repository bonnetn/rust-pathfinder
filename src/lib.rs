use ndarray::{ArrayView2, Ix2};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use numpy::PyArray2;
use pyo3::exceptions;

mod heap;
mod errors;
mod pathfinder;
mod neighbors;

pub fn _find_path<'a>(obstacles: ArrayView2<'a, bool>, start: &'a Ix2, end: &'a Ix2) -> Result<Vec<Ix2>, Box<dyn std::error::Error>> {
    pathfinder::find_path(
        obstacles.view(),
        &start,
        &end,
    )
}

#[pyfunction]
fn find_path(obstacles: &PyArray2<bool>, start: (usize, usize), end: (usize, usize)) -> PyResult<Vec<(usize, usize)>> {
    let obstacles = obstacles.as_array();
    let result = match _find_path(obstacles, &Ix2(start.0, start.1), &Ix2(end.0, end.1)) {
        Ok(r) => r,
        Err(e) => return Err(exceptions::RuntimeError::py_err(e.to_string())),
    };
    let result = result
        .iter()
        .map(|p| (p[0], p[1]))
        .collect();

    Ok(result)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn grid_pathfinding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_path))?;

    Ok(())
}
