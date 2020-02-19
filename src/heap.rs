use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

use ndarray::Ix2;

pub struct HeapElement {
    pub distance: f64,
    pub position: Ix2,
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.partial_cmp(&other.distance)?.reverse())
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance)
            .unwrap_or_else(|| { Equal })
            .reverse()
    }
}

impl Eq for HeapElement {}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

