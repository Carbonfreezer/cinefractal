//! Time-indexed key frame tracks: a mutable builder ([`KeyFrameList`]) that
//! collects key frames and an immutable, time-sorted query structure
//! ([`ClosedKeyFrameList`]) that interpolates values at arbitrary times.

use crate::animation::interpol_value::Interpolatable;
use crate::animation::key_time::KeyTime;
use std::cmp::Ordering;

#[derive(Clone)]
struct KeyFrame<T>
where
    T: Interpolatable,
{
    time: KeyTime,
    value: T,
}

impl<T: Interpolatable> PartialEq for KeyFrame<T> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<T: Interpolatable> Eq for KeyFrame<T> {}

impl<T: Interpolatable> PartialOrd for KeyFrame<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Interpolatable> Ord for KeyFrame<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl<T> KeyFrame<T>
where
    T: Interpolatable,
{
    pub fn interpolate_value_to(&self, other: &KeyFrame<T>, probing_time: KeyTime) -> T {
        let alpha = self
            .time
            .get_interpolation_from_partner(other.time, probing_time);
        self.value.interpolate(&other.value, alpha)
    }

    pub fn new(time: f32, value: T) -> KeyFrame<T> {
        Self {
            time: time.into(),
            value,
        }
    }
}

/// Mutable builder for a list of key frames.
///
/// Key frames can only be added while the list is in this open state. Calling
/// [`KeyFrameList::into_closed`] sorts the frames and hands out an immutable
/// [`ClosedKeyFrameList`] that can be queried (by shared reference, hence in
/// parallel) but no longer modified.
#[derive(Clone)]
pub struct KeyFrameList<T>
where
    T: Interpolatable + Clone,
{
    key_frame_list: Vec<KeyFrame<T>>,
}

impl<T> KeyFrameList<T>
where
    T: Interpolatable + Clone,
{
    /// Generates a list seeded with a single key frame at time 0.
    pub fn new(value: T) -> KeyFrameList<T> {
        let key_frame_list = vec![KeyFrame::new(0.0, value)];
        Self { key_frame_list }
    }

    /// Appends a key frame at the given time. Order does not matter; the frames
    /// are sorted (and duplicate times collapsed) in [`KeyFrameList::into_closed`].
    pub fn add_key_frame(&mut self, time: f32, value: T) {
        self.key_frame_list.push(KeyFrame::new(time, value));
    }

    /// Consumes the builder, sorts the key frames by time and returns the
    /// immutable, queryable list.
    ///
    /// Key frames sharing the same time are collapsed to a single frame keeping
    /// the most recently added value. This makes "re-setting" a key frame at an
    /// already used time behave as an override (last write wins) instead of
    /// leaving an ambiguous duplicate.
    pub fn into_closed(mut self) -> ClosedKeyFrameList<T> {
        // `sort` is stable, so among frames with equal time the insertion order
        // is preserved: the later (overriding) frame comes last.
        self.key_frame_list.sort();
        // `dedup_by` keeps the earlier element `b` and drops the later `a`; we
        // copy `a`'s value into `b` first so the most recent value survives.
        self.key_frame_list.dedup_by(|a, b| {
            if a.time == b.time {
                b.value = a.value.clone();
                true
            } else {
                false
            }
        });
        ClosedKeyFrameList {
            key_frame_list: self.key_frame_list,
        }
    }
}

/// Immutable, time-sorted list of key frames.
///
/// Obtained via [`KeyFrameList::into_closed`]. All queries take `&self`, so a closed
/// list is `Sync` for `T: Sync` and can be shared across threads.
pub struct ClosedKeyFrameList<T>
where
    T: Interpolatable + Clone,
{
    key_frame_list: Vec<KeyFrame<T>>,
}

impl<T> ClosedKeyFrameList<T>
where
    T: Interpolatable + Clone,
{
    /// The time of the last key frame. Since the list is always seeded with a
    /// frame at time 0, this is well defined and `>= 0`.
    pub fn max_time(&self) -> f32 {
        self.key_frame_list
            .last()
            .expect("closed list always holds at least the seed key frame")
            .time
            .into()
    }

    /// Returns the value at `time`. Before the first key frame the first value
    /// is returned, after the last key frame the last value; in between the two
    /// surrounding frames are interpolated.
    pub fn get_interpolated_value(&self, time: f32) -> T {
        let search_time: KeyTime = time.into();

        let idx = self
            .key_frame_list
            .partition_point(|k| k.time < search_time);

        let length = self.key_frame_list.len();
        match idx {
            // If we are before the first element we return the first one.
            0 => self.key_frame_list[0].value.clone(),
            // Let us see, if we are behind the last one.
            i if i >= length => self.key_frame_list[length - 1].value.clone(),
            // In this case we are at an interval.
            i => {
                let first = &self.key_frame_list[i - 1];
                let second = &self.key_frame_list[i];
                first.interpolate_value_to(second, search_time)
            }
        }
    }
}
