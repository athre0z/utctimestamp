use std::fmt::{Display, Formatter};
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

// ============================================================================================== //
// [UTC timestamp]                                                                                //
// ============================================================================================== //

/// Represents a dumb but fast UTC timestamp.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct UtcTimeStamp(i64);

/// Display timestamp using chrono.
impl Display for UtcTimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        chrono::DateTime::<chrono::Utc>::from(*self).fmt(f)
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

/// Explicit conversion from and to `i64`.
impl UtcTimeStamp {
    pub fn from_milliseconds(int: i64) -> Self {
        UtcTimeStamp(int)
    }

    pub fn as_milliseconds(self) -> i64 {
        self.0
    }
}

/// Calculate the timestamp advanced by a timedelta.
impl std::ops::Add<TimeDelta> for UtcTimeStamp {
    type Output = UtcTimeStamp;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        UtcTimeStamp(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<TimeDelta> for UtcTimeStamp {
    fn add_assign(&mut self, rhs: TimeDelta) {
        *self = *self + rhs;
    }
}

/// Calculate the timestamp lessened by a timedelta.
impl std::ops::Sub<TimeDelta> for UtcTimeStamp {
    type Output = UtcTimeStamp;

    fn sub(self, rhs: TimeDelta) -> Self::Output {
        UtcTimeStamp(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign<TimeDelta> for UtcTimeStamp {
    fn sub_assign(&mut self, rhs: TimeDelta) {
        *self = *self - rhs;
    }
}

/// Calculate signed timedelta between two timestamps.
impl std::ops::Sub<UtcTimeStamp> for UtcTimeStamp {
    type Output = TimeDelta;

    fn sub(self, rhs: UtcTimeStamp) -> Self::Output {
        TimeDelta(self.0 - rhs.0)
    }
}

// ============================================================================================== //
// [TimeDelta]                                                                                    //
// ============================================================================================== //

/// Millisecond precision time delta.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct TimeDelta(i64);

/// Display timedelta using chrono.
impl Display for TimeDelta {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
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

impl std::ops::Add<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        TimeDelta(self.0 + rhs.0)
    }
}

impl std::ops::Sub<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn sub(self, rhs: TimeDelta) -> Self::Output {
        TimeDelta(self.0 - rhs.0)
    }
}

/// Multiply the timestamp to be n times as long.
/// `i32` because that's what chrono does.
impl std::ops::Mul<i32> for TimeDelta {
    type Output = TimeDelta;

    fn mul(self, rhs: i32) -> Self::Output {
        TimeDelta(self.0 * i64::from(rhs))
    }
}

/// Explicit conversion from and to `i64`.
impl TimeDelta {
    pub fn from_milliseconds(int: i64) -> Self {
        TimeDelta(int)
    }

    pub fn as_milliseconds(self) -> i64 {
        self.0
    }
}

// ============================================================================================== //
// [TimeRange]                                                                                    //
// ============================================================================================== //

/// An iterator looping over dates given a time delta as step.
/// The range is closed both left and right.
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
}

// ============================================================================================== //
