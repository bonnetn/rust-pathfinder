use numpy::PyArray2;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::exit_red_zone::exit_red_zone_impl;
pub use crate::find_path::find_path_impl;
pub use crate::grid::Grid;
pub use crate::map::{Bounded, GridMap};
use crate::point::{is_in_bounds, Point2D};
use crate::map::GridMap2;

mod errors;
mod exit_red_zone;
mod find_path;
mod grid;
mod heap;
mod line_of_sight;
mod map;
mod neighbors;
mod point;


#[pyfunction]
pub fn find_path(obstacles: &PyArray2<bool>, start: Point2D, end: Point2D) -> PyResult<Vec<Point2D>> {
    let obstacles = obstacles.to_owned_array();
    let obstacles = Grid::from(obstacles);
    let map = GridMap::new(obstacles);

    if !is_in_bounds(start, map.boundaries()) {
        return Err(exceptions::ValueError::py_err("start position not in bounds".to_string()));
    }

    if !is_in_bounds(end, map.boundaries()) {
        return Err(exceptions::ValueError::py_err("end position not in bounds".to_string()));
    }

    let result = match find_path_impl(&map, start, end) {
        Ok(r) => r,
        Err(e) => return Err(exceptions::RuntimeError::py_err(e.to_string())),
    };
    let result = result
        .iter()
        .map(|p| *p)
        .collect();

    Ok(result)
}

#[pyfunction]
pub fn find_path2(obstacles: &PyArray2<bool>, circles: Vec<(Point2D, isize)>, start: Point2D, end: Point2D) -> PyResult<Vec<Point2D>> {
    let obstacles = obstacles.to_owned_array();
    let obstacles = Grid::from(obstacles);
    let map = GridMap2::new(obstacles, circles);

    if !is_in_bounds(start, map.boundaries()) {
        return Err(exceptions::ValueError::py_err("start position not in bounds".to_string()));
    }

    if !is_in_bounds(end, map.boundaries()) {
        return Err(exceptions::ValueError::py_err("end position not in bounds".to_string()));
    }

    let result = match find_path_impl(&map, start, end) {
        Ok(r) => r,
        Err(e) => return Err(exceptions::RuntimeError::py_err(e.to_string())),
    };
    let result = result
        .iter()
        .map(|p| *p)
        .collect();

    Ok(result)
}

#[pyfunction]
pub fn exit_red_zone(obstacles: &PyArray2<bool>, start: Point2D) -> PyResult<Point2D> {
    let obstacles = obstacles.to_owned_array();
    let obstacles = Grid::from(obstacles);
    let map = GridMap::new(obstacles);
    if !is_in_bounds(start, map.boundaries()) {
        return Err(exceptions::ValueError::py_err("start position not in bounds".to_string()));
    }

    let result = match exit_red_zone_impl(&map, &start) {
        Ok(r) => r,
        Err(e) => return Err(exceptions::RuntimeError::py_err(e.to_string())),
    };
    Ok(result)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn grid_pathfinding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(exit_red_zone))?;
    m.add_wrapped(wrap_pyfunction!(find_path))?;
    m.add_wrapped(wrap_pyfunction!(find_path2))?;

    Ok(())
}
