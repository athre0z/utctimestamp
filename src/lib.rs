//! Simple & fast UTC time types.
//!
//! While [chrono](https://crates.io/crates/chrono) is great for dealing with time
//! in most cases, its 96-bit integer design can be costly when processing and storing
//! large amounts of timestamp data.
//!
//! This lib solves this problem by providing very simple UTC timestamps that can be
//! converted from and into their corresponding chrono counterpart using Rust's
//! `From` and `Into` traits. chrono is then used for all things that aren't expected
//! to occur in big batches, such as formatting and displaying the timestamps.

use core::{fmt, ops};

#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

// ============================================================================================== //
// [UTC timestamp]                                                                                //
// ============================================================================================== //

/// Represents a dumb but fast UTC timestamp.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct UtcTimeStamp(i64);

/// Display timestamp using chrono.
impl fmt::Display for UtcTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        chrono::DateTime::<chrono::Utc>::from(*self).fmt(f)
    }
}

impl fmt::Debug for UtcTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UtcTimeStamp({})", self.0)
    }
}

/// Create a dumb timestamp from a chrono date time object.
impl From<chrono::DateTime<chrono::Utc>> for UtcTimeStamp {
    fn from(other: chrono::DateTime<chrono::Utc>) -> Self {
        Self(other.timestamp_millis())
    }
}

/// Create a chrono date time object from a dumb timestamp.
impl From<UtcTimeStamp> for chrono::DateTime<chrono::Utc> {
    fn from(other: UtcTimeStamp) -> Self {
        let sec = other.0 / 1000;
        let ns = (other.0 % 1000 * 1_000_000) as u32;
        let naive = chrono::NaiveDateTime::from_timestamp(sec, ns);
        chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc)
    }
}

impl UtcTimeStamp {
    /// Initialize a timestamp with 0, `1970-01-01 00:00:00 UTC`.
    #[inline]
    pub const fn zero() -> Self {
        UtcTimeStamp(0)
    }

    /// Initialize a timestamp using the current local time converted to UTC.
    pub fn now() -> Self {
        chrono::Utc::now().into()
    }

    /// Explicit conversion from `i64`.
    #[inline]
    pub const fn from_milliseconds(int: i64) -> Self {
        UtcTimeStamp(int)
    }

    /// Explicit conversion from `i64` seconds.
    #[inline]
    pub const fn from_seconds(int: i64) -> Self {
        UtcTimeStamp(int * 1000)
    }

    /// Explicit conversion to `i64`.
    #[inline]
    pub const fn as_milliseconds(self) -> i64 {
        self.0
    }

    /// Align a timestamp to a given frequency.
    pub const fn align_to(self, freq: TimeDelta) -> UtcTimeStamp {
        self.align_to_anchored(UtcTimeStamp::zero(), freq)
    }

    /// Align a timestamp to a given frequency, with a time anchor.
    pub const fn align_to_anchored(self, anchor: UtcTimeStamp, freq: TimeDelta) -> UtcTimeStamp {
        UtcTimeStamp((self.0 - anchor.0) / freq.0 * freq.0 + anchor.0)
    }

    /// Check whether the timestamp is 0 (`1970-01-01 00:00:00 UTC`).
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

/// Calculate the timestamp advanced by a timedelta.
impl ops::Add<TimeDelta> for UtcTimeStamp {
    type Output = UtcTimeStamp;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        UtcTimeStamp(self.0 + rhs.0)
    }
}

impl ops::AddAssign<TimeDelta> for UtcTimeStamp {
    fn add_assign(&mut self, rhs: TimeDelta) {
        *self = *self + rhs;
    }
}

/// Calculate the timestamp lessened by a timedelta.
impl ops::Sub<TimeDelta> for UtcTimeStamp {
    type Output = UtcTimeStamp;

    fn sub(self, rhs: TimeDelta) -> Self::Output {
        UtcTimeStamp(self.0 - rhs.0)
    }
}

impl ops::SubAssign<TimeDelta> for UtcTimeStamp {
    fn sub_assign(&mut self, rhs: TimeDelta) {
        *self = *self - rhs;
    }
}

/// Calculate signed timedelta between two timestamps.
impl ops::Sub<UtcTimeStamp> for UtcTimeStamp {
    type Output = TimeDelta;

    fn sub(self, rhs: UtcTimeStamp) -> Self::Output {
        TimeDelta(self.0 - rhs.0)
    }
}

// /// How far away is the timestamp from being aligned to the given timedelta?
// impl ops::Rem<TimeDelta> for UtcTimeStamp {
//     type Output = TimeDelta;
//
//     fn rem(self, rhs: TimeDelta) -> Self::Output {
//         TimeDelta(self.0 % rhs.0)
//     }
// }

// ============================================================================================== //
// [TimeDelta]                                                                                    //
// ============================================================================================== //

/// Millisecond precision time delta.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct TimeDelta(i64);

/// Display timedelta using chrono.
impl fmt::Display for TimeDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        chrono::Duration::from(*self).fmt(f)
    }
}

/// Create a simple timedelta from a chrono duration.
impl From<chrono::Duration> for TimeDelta {
    fn from(other: chrono::Duration) -> Self {
        Self(other.num_milliseconds())
    }
}

/// Create a chrono duration from a simple timedelta.
impl From<TimeDelta> for chrono::Duration {
    fn from(other: TimeDelta) -> Self {
        chrono::Duration::milliseconds(other.0)
    }
}

impl ops::Add<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        TimeDelta(self.0 + rhs.0)
    }
}

impl ops::Sub<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn sub(self, rhs: TimeDelta) -> Self::Output {
        TimeDelta(self.0 - rhs.0)
    }
}

/// Multiply the delta to be n times as long.
impl ops::Mul<i64> for TimeDelta {
    type Output = TimeDelta;

    fn mul(self, rhs: i64) -> Self::Output {
        TimeDelta(self.0 * rhs)
    }
}

/// Shorten the delta by a given factor. Integer div.
impl ops::Div<i64> for TimeDelta {
    type Output = TimeDelta;

    fn div(self, rhs: i64) -> Self::Output {
        TimeDelta(self.0 / rhs)
    }
}

/// How many times does the timestamp fit into another?
impl ops::Div<TimeDelta> for TimeDelta {
    type Output = i64;

    fn div(self, rhs: TimeDelta) -> Self::Output {
        self.0 / rhs.0
    }
}

/// How far away is the delta from being aligned to another delta?
impl ops::Rem<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn rem(self, rhs: TimeDelta) -> Self::Output {
        TimeDelta(self.0 % rhs.0)
    }
}

/// Explicit conversion from and to `i64`.
impl TimeDelta {
    #[inline]
    pub const fn zero() -> Self {
        TimeDelta(0)
    }

    #[inline]
    pub const fn from_hours(int: i64) -> Self {
        TimeDelta::from_minutes(int * 60)
    }

    #[inline]
    pub const fn from_minutes(int: i64) -> Self {
        TimeDelta::from_seconds(int * 60)
    }

    #[inline]
    pub const fn from_seconds(int: i64) -> Self {
        TimeDelta(int * 1000)
    }

    #[inline]
    pub const fn from_milliseconds(int: i64) -> Self {
        TimeDelta(int)
    }

    #[inline]
    pub const fn as_milliseconds(self) -> i64 {
        self.0
    }

    /// Check whether the timedelta is 0.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if the timedelta is positive and
    /// `false` if it is zero or negative.
    #[inline]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    /// Returns `true` if the timedelta is negative and
    /// `false` if it is zero or positive.
    #[inline]
    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }
}

// ============================================================================================== //
// [TimeRange]                                                                                    //
// ============================================================================================== //

/// An iterator looping over dates given a time delta as step.
///
/// The range is either right open or right closed depending on the
/// constructor chosen, but always left closed.
///
/// Examples:
///
/// ```
/// use utctimestamp::TimeRange;
/// use chrono::{offset::TimeZone, Duration, Utc};
///
/// let start = Utc.ymd(2019, 4, 14).and_hms(0, 0, 0);
/// let end = Utc.ymd(2019, 4, 16).and_hms(0, 0, 0);
/// let step = Duration::hours(12);
/// let tr: Vec<_> = TimeRange::right_closed(start, end, step).collect();
///
/// assert_eq!(tr, vec![
///     Utc.ymd(2019, 4, 14).and_hms(0, 0, 0).into(),
///     Utc.ymd(2019, 4, 14).and_hms(12, 0, 0).into(),
///     Utc.ymd(2019, 4, 15).and_hms(0, 0, 0).into(),
///     Utc.ymd(2019, 4, 15).and_hms(12, 0, 0).into(),
///     Utc.ymd(2019, 4, 16).and_hms(0, 0, 0).into(),
/// ]);
/// ```
#[derive(Debug)]
pub struct TimeRange {
    cur: UtcTimeStamp,
    end: UtcTimeStamp,
    step: TimeDelta,
    right_closed: bool,
}

impl TimeRange {
    /// Create a time range that includes the end date.
    pub fn right_closed(
        start: impl Into<UtcTimeStamp>,
        end: impl Into<UtcTimeStamp>,
        step: impl Into<TimeDelta>,
    ) -> Self {
        TimeRange {
            cur: start.into(),
            end: end.into(),
            step: step.into(),
            right_closed: true,
        }
    }

    /// Create a time range that excludes the end date.
    pub fn right_open(
        start: impl Into<UtcTimeStamp>,
        end: impl Into<UtcTimeStamp>,
        step: impl Into<TimeDelta>,
    ) -> Self {
        TimeRange {
            cur: start.into(),
            end: end.into(),
            step: step.into(),
            right_closed: false,
        }
    }
}

impl Iterator for TimeRange {
    type Item = UtcTimeStamp;

    fn next(&mut self) -> Option<Self::Item> {
        let exhausted = if self.right_closed {
            self.cur > self.end
        } else {
            self.cur >= self.end
        };

        if exhausted {
            None
        } else {
            let cur = self.cur;
            self.cur += self.step;
            Some(cur)
        }
    }
}

// ============================================================================================== //
// [Tests]                                                                                        //
// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{offset::TimeZone, Duration, Utc};

    #[test]
    fn open_time_range() {
        let start = Utc.ymd(2019, 4, 14).and_hms(0, 0, 0);
        let end = Utc.ymd(2019, 4, 16).and_hms(0, 0, 0);
        let step = Duration::hours(12);
        let tr: Vec<_> = Iterator::collect(TimeRange::right_closed(start, end, step));
        assert_eq!(tr, vec![
            Utc.ymd(2019, 4, 14).and_hms(0, 0, 0).into(),
            Utc.ymd(2019, 4, 14).and_hms(12, 0, 0).into(),
            Utc.ymd(2019, 4, 15).and_hms(0, 0, 0).into(),
            Utc.ymd(2019, 4, 15).and_hms(12, 0, 0).into(),
            Utc.ymd(2019, 4, 16).and_hms(0, 0, 0).into(),
        ]);
    }

    #[test]
    fn timestamp_and_delta_vs_chrono() {
        let c_dt = Utc.ymd(2019, 3, 13).and_hms(16, 14, 9);
        let c_td = Duration::milliseconds(123456);

        let my_dt = UtcTimeStamp::from(c_dt.clone());
        let my_td = TimeDelta::from_milliseconds(123456);
        assert_eq!(TimeDelta::from(c_td.clone()), my_td);

        let c_result = c_dt + c_td * 555;
        let my_result = my_dt + my_td * 555;
        assert_eq!(UtcTimeStamp::from(c_result.clone()), my_result);
    }

    #[test]
    fn timestamp_ord_eq() {
        let ts1: UtcTimeStamp = UtcTimeStamp::from_milliseconds(111);
        let ts2: UtcTimeStamp = UtcTimeStamp::from_milliseconds(222);
        let ts3: UtcTimeStamp = UtcTimeStamp::from_milliseconds(222);

        assert!(ts1 < ts2);
        assert!(ts2 > ts1);
        assert!(ts1 <= ts2);
        assert!(ts2 >= ts3);
        assert!(ts2 <= ts3);
        assert!(ts2 >= ts3);
        assert_eq!(ts2, ts3);
        assert_ne!(ts1, ts3);
    }

    #[test]
    fn align_to_anchored() {
        let day = Utc.ymd(2020, 9, 28);
        let ts: UtcTimeStamp = day.and_hms(19, 32, 51).into();

        assert_eq!(
            ts.align_to_anchored(day.and_hms(0, 0, 0).into(), TimeDelta::from_seconds(60 * 5)),
            day.and_hms(19, 30, 0).into(),
        );

        assert_eq!(
            ts.align_to_anchored(
                day.and_hms(9 /* irrelevant */, 1, 3).into(),
                TimeDelta::from_seconds(60 * 5)
            ),
            day.and_hms(19, 31, 3).into(),
        );
    }

    #[test]
    fn align_to_anchored_eq() {
        let day = Utc.ymd(2020, 1, 1);
        let anchor: UtcTimeStamp = day.and_hms(0, 0, 0).into();
        let freq = TimeDelta::from_seconds(5 * 60);

        let ts1: UtcTimeStamp = day.and_hms(12, 1, 11).into();
        let ts2: UtcTimeStamp = day.and_hms(12, 4, 11).into();
        assert_eq!(
            ts1.align_to_anchored(anchor, freq),
            ts2.align_to_anchored(anchor, freq),
        );
    }
}

// ============================================================================================== //
