use crate::RosMessageType;
use std::ops::{Add, Sub};
use std::cmp::Ordering::{Equal, Greater, Less};

/// Matches the integral ros1 type time, with extensions for ease of use
/// NOTE: in ROS1 "Time" is not a message in and of itself and std_msgs/Time should be used.
/// However, in ROS2 "Time" is a message and part of builtin_interfaces/Time.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Time {
    // Note: rosbridge appears to accept secs and nsecs in for time without issue?
    // Not sure we should actually rely on this behavior, but ok for now...

    // This alias is required for ros2 where field has been renamed
    #[serde(alias = "sec")]
    pub secs: u32,
    // This alias is required for ros2 where field has been renamed
    #[serde(alias = "nanosec")]
    pub nsecs: u32,
}

impl Time {
    fn seconds(&self) -> f64 {
        f64::from(self.secs) + f64::from(self.nsecs) / 1e9
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.seconds() == other.seconds() {
            return Some(Equal);
        } else if self.seconds() > other.seconds() {
            return Some(Greater);
        }
        return Some(Less);
    }

    fn lt(&self, other: &Self) -> bool {
        self.seconds().lt(&other.seconds())
    }

    fn le(&self, other: &Self) -> bool {
        self.seconds().le(&other.seconds())
    }

    fn gt(&self, other: &Self) -> bool {
        self.seconds().gt(&other.seconds())
    }

    fn ge(&self, other: &Self) -> bool {
        self.seconds().ge(&other.seconds())
    }
}

impl From<std::time::SystemTime> for Time {
    fn from(val: std::time::SystemTime) -> Self {
        let delta = val
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to convert system time into unix epoch");
        let downcast_secs = u32::try_from(delta.as_secs()).expect("Failed to convert system time to ROS representation, seconds term overflows u32 likely");
        Time {
            secs: downcast_secs,
            nsecs: delta.subsec_nanos(),
        }
    }
}

impl RosMessageType for Time {
    const ROS_TYPE_NAME: &'static str = "builtin_interfaces/Time";
    // TODO: ROS2 support
    const MD5SUM: &'static str = "";
    const DEFINITION: &'static str = "";
}

// TODO provide chrono conversions here behind a cfg flag

/// Matches the integral ros1 duration type, with extensions for ease of use
/// NOTE: Is not a message in and of itself use std_msgs/Duration for that
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Debug, Default, Clone, PartialEq)]
pub struct Duration {
    pub sec: i32,
    pub nsec: i32,
}

/// Note this provides both tokio::time::Duration and std::time::Duration
impl From<tokio::time::Duration> for Duration {
    fn from(val: tokio::time::Duration) -> Self {
        let downcast_sec = i32::try_from(val.as_secs())
            .expect("Failed to cast tokio duration to ROS duration, secs could not fit in i32");
        let downcast_nsec = i32::try_from(val.subsec_nanos())
            .expect("Failed to cast tokio duration ROS duration, nsecs could not fit in i32");
        Duration {
            sec: downcast_sec,
            nsec: downcast_nsec,
        }
    }
}

impl Add<Duration> for Time {
    type Output = Time;
    fn add(self, rhs: Duration) -> Self {
        let nsec_sum = self.nsecs as i64 + rhs.nsec as i64;
        let secs = self.secs as i64 + rhs.sec as i64 + nsec_sum / 1_000_000_000;
        let nsecs = nsec_sum.rem_euclid(1_000_000_000);
        if secs < 0 {
            // TODO(lucasw) return an error
            return Self {secs: 0, nsecs: 0};
        }
        Self {
            secs: secs as u32,
            nsecs: nsecs as u32,
        }
    }
}

impl Sub<Time> for Time {
    type Output = Duration;
    fn sub(self, rhs: Time) -> Duration {
        let nsec_diff = self.nsecs as i64 - rhs.nsecs as i64;
        let secs = self.secs as i64 - rhs.secs as i64 + nsec_diff / 1_000_000_000;
        let nsecs = nsec_diff.rem_euclid(1_000_000_000);

        Duration {
            sec: secs as i32,
            nsec: nsecs as i32,
        }
    }
}

// TODO: provide chrono conversions here behind a cfg flag
