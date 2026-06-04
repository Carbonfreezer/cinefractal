//! A total-ordered wrapper around the `f32` timestamp of a key frame.

use std::cmp::Ordering;

/// The time of a key frame. Wraps an `f32` and provides a total ordering (via
/// `total_cmp`) so key frames can be sorted and compared deterministically,
/// even in the presence of NaN.
#[derive(Clone, Copy)]
pub struct KeyTime(f32);

// All comparisons are derived from `Ord`/`total_cmp` so that `PartialOrd`,
// `PartialEq` and `Eq` stay consistent with each other (and handle NaN
// deterministically instead of yielding `None`).
impl Ord for KeyTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for KeyTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for KeyTime {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for KeyTime {}

impl From<f32> for KeyTime {
    fn from(value: f32) -> KeyTime {
        KeyTime(value)
    }
}

impl From<KeyTime> for f32 {
    fn from(value: KeyTime) -> f32 {
        value.0
    }
}

impl KeyTime {
    /// Returns where `probing` lies between `self` and `partner` as a fraction
    /// in `0..=1` (`self` -> 0, `partner` -> 1). Requires `self <= partner`. If
    /// both times coincide the span is zero and we snap to the partner (1.0).
    pub fn get_interpolation_from_partner(&self, partner: KeyTime, probing: KeyTime) -> f32 {
        debug_assert!(
            *self <= partner,
            "We should be smaller than the probing partner handed over."
        );
        let span = partner.0 - self.0;
        // Two key frames sharing the same time would otherwise yield a
        // division by zero (NaN). In that case we snap to the partner value.
        if span == 0.0 {
            return 1.0;
        }
        (probing.0 - self.0) / span
    }
}
