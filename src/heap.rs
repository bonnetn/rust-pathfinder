use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

pub struct HeapElement<T> {
    pub f_score: f64,
    pub position: T,
}

impl<T> PartialOrd for HeapElement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.f_score.partial_cmp(&other.f_score)?.reverse())
    }
}

impl<T> Ord for HeapElement<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.partial_cmp(&other.f_score)
            .unwrap_or_else(|| { Equal })
            .reverse()
    }
}

impl<T> Eq for HeapElement<T> {}

impl<T> PartialEq for HeapElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.f_score.eq(&other.f_score)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::{Greater, Less};

    use super::*;

    fn heap_element() -> (HeapElement<i32>, HeapElement<i32>) {
        (HeapElement { f_score: 0.0, position: 42 },
         HeapElement { f_score: 1.0, position: 64 })
    }

    #[test]
    fn test_heap_element_partial_ordering() {
        let (elem_a, elem_b) = heap_element();
        assert_eq!(elem_a.partial_cmp(&elem_b), Some(Greater));
        assert_eq!(elem_b.partial_cmp(&elem_a), Some(Less));
        assert_eq!(elem_a.partial_cmp(&elem_a), Some(Equal));
    }

    #[test]
    fn test_heap_element_ordering() {
        let (elem_a, elem_b) = heap_element();
        assert_eq!(elem_a.partial_cmp(&elem_b), Some(Greater));
        assert_eq!(elem_b.partial_cmp(&elem_a), Some(Less));
        assert_eq!(elem_a.partial_cmp(&elem_a), Some(Equal));
    }
}
