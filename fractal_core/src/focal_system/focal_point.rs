//! The definition of a focal point and a container to keep track of the most interesting points.

use crate::iteration_and_color::complex_number::ComplexNumber;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A focal point with an evaluation
#[derive(Debug, Clone, Copy, Default)]
pub struct FocalPoint {
    /// The point we represent.
    pub point: ComplexNumber,
    /// The evaluation we have (the bigger the better.)
    pub evaluation: f64,
}

impl Eq for FocalPoint {}

impl PartialEq<Self> for FocalPoint {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for FocalPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FocalPoint {
    /// We reverse the ordering because of the Binary Heap, which gives us the maximum, but we want the minimum.
    fn cmp(&self, other: &Self) -> Ordering {
        other.evaluation.total_cmp(&self.evaluation)
    }
}

/// A collection with focal points. Used for sorting and collection.
pub struct FocalPointCollection {
    inner_heap: BinaryHeap<FocalPoint>,
    capacity: usize,
}

impl FocalPointCollection {
    /// Creates a collection of focal points with a set capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner_heap: BinaryHeap::with_capacity(capacity),
            capacity,
        }
    }

    /// Processes a slice of focal points, keeping only the best ones up to the
    /// collection's capacity. Could also be done with `sort().take(n)`, but this
    /// approach should be faster when `n` is significantly smaller than the
    /// number of elements handed over.
    pub fn append(&mut self, point_slice: &[FocalPoint]) {
        for point in point_slice {
            if self.inner_heap.len() < self.capacity {
                self.inner_heap.push(*point);
                continue;
            }
            debug_assert!(
                self.inner_heap.len() == self.capacity,
                "We should never run over capacity"
            );
            let minimal_element = self.inner_heap.peek().unwrap();
            if point.evaluation <= minimal_element.evaluation {
                continue;
            }
            self.inner_heap.pop();
            self.inner_heap.push(*point);
        }
    }

    /// Gets the iterator of the focal points.
    pub fn get_iterator(&self) -> impl Iterator<Item = &FocalPoint> {
        self.inner_heap.iter()
    }

    /// Gets a reasonably short path (greedy approximation of TSP),
    /// starting with the element that is closest to the point handed over.
    pub fn get_short_path(&self, reference_point: ComplexNumber) -> Vec<FocalPoint> {
        let mut todo_list = self.get_iterator().collect::<Vec<_>>();
        let mut result = Vec::with_capacity(todo_list.len());
        let mut scan = reference_point;
        while !todo_list.is_empty() {
            let (ind, element) = todo_list
                .iter()
                .enumerate()
                .min_by(
                    |&(_, &FocalPoint { point: x, .. }), &(_, &FocalPoint { point: y, .. })| {
                        (*x - scan)
                            .sq_magnitude()
                            .total_cmp(&(*y - scan).sq_magnitude())
                    },
                )
                .expect("Emptiness of list has been caught in while loop");
            scan = element.point;
            result.push(**element);
            todo_list.swap_remove(ind);
        }
        result
    }

    /// Returns the list with the focus point in descending order.
    /// Pay attention: We have inverted the order relation on focal point.
    pub fn get_descending_path(&self) -> Vec<FocalPoint> {
        let mut list = self.get_iterator().copied().collect::<Vec<_>>();
        list.sort();
        list
    }
}
