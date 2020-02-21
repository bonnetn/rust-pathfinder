use ndarray::Ix2;
use numpy::PyArray2;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod heap;
mod errors;
mod neighbors;
mod line_of_sight;
mod pathfinder;

fn is_in_bounds((x, y): (usize, usize), (max_x, max_y): (usize, usize)) -> bool {
    x < max_x && y < max_y
}

#[pyfunction]
fn find_path(obstacles: &PyArray2<bool>, start: (usize, usize), end: (usize, usize)) -> PyResult<Vec<(usize, usize)>> {
    let obstacles = obstacles.as_array();

    if !is_in_bounds(start, obstacles.dim()) {
        return Err(exceptions::ValueError::py_err("start position not in bounds".to_string()));
    }

    if !is_in_bounds(end, obstacles.dim()) {
        return Err(exceptions::ValueError::py_err("end position not in bounds".to_string()));
    }

    let result = match pathfinder::find_path(
        obstacles.view(),
        &Ix2(start.0, start.1),
        &Ix2(end.0, end.1),
    ) {
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
