use ndarray::{ArrayView2,Ix2};

mod heap;
mod errors;
mod pathfinder;
mod neighbors;

pub fn find_path<'a>(obstacles: ArrayView2<'a, bool>, start: &'a Ix2, end: &'a Ix2) -> Result<Vec<Ix2>, Box<dyn std::error::Error>> {
    pathfinder::find_path(
        obstacles.view(),
        &start,
        &end,
    )
}
