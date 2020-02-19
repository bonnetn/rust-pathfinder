use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

use ndarray::Ix2;

pub struct HeapElement {
    pub f_score: f64,
    pub position: Ix2,
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.f_score.partial_cmp(&other.f_score)?.reverse())
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.partial_cmp(&other.f_score)
            .unwrap_or_else(|| { Equal })
            .reverse()
    }
}

impl Eq for HeapElement {}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.f_score.eq(&other.f_score)
    }
}

