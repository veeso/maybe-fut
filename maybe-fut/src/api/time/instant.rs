use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

use crate::{maybe_fut_constructor_sync, maybe_fut_method_sync};

/// A measurement of a monotonically nondecreasing clock. Opaque and useful only with [`std::time::Duration`].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Unwrap)]
#[unwrap_types(
    std(std::time::Instant),
    tokio(tokio::time::Instant),
    tokio_gated("tokio-time")
)]
pub struct Instant(InstantInner);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
enum InstantInner {
    /// Std instant
    Std(std::time::Instant),
    /// Tokio instant
    #[cfg(tokio_time)]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio-time")))]
    Tokio(tokio::time::Instant),
}

impl From<std::time::Instant> for Instant {
    fn from(instant: std::time::Instant) -> Self {
        Instant(InstantInner::Std(instant))
    }
}

#[cfg(tokio_time)]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-time")))]
impl From<tokio::time::Instant> for Instant {
    fn from(instant: tokio::time::Instant) -> Self {
        Instant(InstantInner::Tokio(instant))
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, other: Duration) -> Self::Output {
        // convert the inner types to std
        #[cfg(tokio_time)]
        {
            let is_async = matches!(self.0, InstantInner::Tokio(_));
            let a = match self.0 {
                InstantInner::Std(a) => a,
                #[cfg(tokio_time)]
                InstantInner::Tokio(a) => a.into_std(),
            };
            // perform the addition
            if is_async {
                Instant(InstantInner::Tokio((a + other).into()))
            } else {
                Instant(InstantInner::Std(a + other))
            }
        }
        #[cfg(not(tokio_time))]
        {
            use crate::unwrap::Unwrap as _;
            Instant(InstantInner::Std(self.unwrap_std() + other))
        }
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        #[cfg(tokio_time)]
        {
            // convert the inner types to std
            let is_async = matches!(self.0, InstantInner::Tokio(_));
            let a = match self.0 {
                InstantInner::Std(a) => a,
                #[cfg(tokio_time)]
                InstantInner::Tokio(a) => a.into_std(),
            };
            // perform the addition
            if is_async {
                self.0 = InstantInner::Tokio((a + other).into());
            } else {
                self.0 = InstantInner::Std(a + other);
            }
        }
        #[cfg(not(tokio_time))]
        {
            // perform the addition
            use crate::unwrap::Unwrap as _;
            *self = (self.unwrap_std() + other).into();
        }
    }
}

impl Sub for Instant {
    type Output = std::time::Duration;

    fn sub(self, other: Instant) -> Self::Output {
        // convert the inner types to std
        let a = match self.0 {
            InstantInner::Std(a) => a,
            #[cfg(tokio_time)]
            InstantInner::Tokio(a) => a.into_std(),
        };
        let b = match other.0 {
            InstantInner::Std(b) => b,
            #[cfg(tokio_time)]
            InstantInner::Tokio(b) => b.into_std(),
        };
        // perform the subtraction
        a - b
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        #[cfg(tokio_time)]
        {
            let is_async = matches!(self.0, InstantInner::Tokio(_));

            // convert the inner types to std
            let a = match self.0 {
                InstantInner::Std(a) => a,
                #[cfg(tokio_time)]
                InstantInner::Tokio(a) => a.into_std(),
            };

            // perform the subtraction
            if is_async {
                self.0 = InstantInner::Tokio((a - other).into());
            } else {
                self.0 = InstantInner::Std(a - other);
            }
        }
        #[cfg(not(tokio_time))]
        {
            use crate::unwrap::Unwrap as _;
            // perform the subtraction
            *self = (self.unwrap_std() - other).into();
        }
    }
}

impl Instant {
    maybe_fut_constructor_sync!(
        /// Returns an instant corresponding to the current time.
        now() -> Self,
        std::time::Instant::now,
        tokio::time::Instant::now,
        tokio_time
    );

    maybe_fut_method_sync!(
        /// Returns the amount of time elapsed since this instant was created, or zero duration if this instant is in the future.
        elapsed() -> Duration,
        InstantInner::Std,
        InstantInner::Tokio,
        tokio_time
    );

    /// Returns `Some(T)` where `t is the time `self + duration` if `t` can be represented as [`Instant`], otherwise `None`.
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        match self.0 {
            InstantInner::Std(a) => Some(InstantInner::Std(a.checked_add(duration)?)),
            #[cfg(tokio_time)]
            InstantInner::Tokio(a) => Some(InstantInner::Tokio(a.checked_add(duration)?)),
        }
        .map(Instant)
    }

    /// Returns `Some(T)` where `t is the time `self - duration` if `t` can be represented as [`Instant`], otherwise `None`.
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        #[cfg(tokio_time)]
        {
            let is_async = matches!(self.0, InstantInner::Tokio(_));

            // convert the inner types to std
            let a = match self.0 {
                InstantInner::Std(a) => a,
                #[cfg(tokio_time)]
                InstantInner::Tokio(a) => a.into_std(),
            };

            // perform the checked subtraction
            if is_async {
                Some(InstantInner::Tokio(a.checked_sub(duration)?.into()))
            } else {
                Some(InstantInner::Std(a.checked_sub(duration)?))
            }
            .map(Instant)
        }
        #[cfg(not(tokio_time))]
        {
            // convert the inner types to std
            use crate::unwrap::Unwrap as _;
            let a = self.unwrap_std();

            // perform the checked subtraction
            Some(InstantInner::Std(a.checked_sub(duration)?)).map(Instant)
        }
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        // convert the inner types to std
        let a = match self.0 {
            InstantInner::Std(a) => a,
            #[cfg(tokio_time)]
            InstantInner::Tokio(a) => a.into_std(),
        };
        let b = match earlier.0 {
            InstantInner::Std(b) => b,
            #[cfg(tokio_time)]
            InstantInner::Tokio(b) => b.into_std(),
        };

        // perform the duration since
        a.duration_since(b)
    }

    /// Returns the duration since `earlier` if `earlier` is before `self`, otherwise returns `None`.
    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        // convert the inner types to std
        let a = match self.0 {
            InstantInner::Std(a) => a,
            #[cfg(tokio_time)]
            InstantInner::Tokio(a) => a.into_std(),
        };
        let b = match earlier.0 {
            InstantInner::Std(b) => b,
            #[cfg(tokio_time)]
            InstantInner::Tokio(b) => b.into_std(),
        };

        // perform the checked duration since
        a.checked_duration_since(b)
    }

    /// Returns the amount of time elapsed from another instant to this one, or zero duration if that instant is later than this one.
    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        // convert the inner types to std
        let a = match self.0 {
            InstantInner::Std(a) => a,
            #[cfg(tokio_time)]
            InstantInner::Tokio(a) => a.into_std(),
        };
        let b = match earlier.0 {
            InstantInner::Std(b) => b,
            #[cfg(tokio_time)]
            InstantInner::Tokio(b) => b.into_std(),
        };

        // perform the saturation duration since
        a.saturating_duration_since(b)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_instant_add() {
        let instant = Instant::now();
        let duration = Duration::new(1, 0);
        let new_instant = instant + duration;
        assert!(new_instant > instant);
    }

    #[test]
    fn test_instant_sub() {
        let instant1 = Instant::now();
        let instant2 = Instant::now();
        let duration = instant1 - instant2;
        assert!(duration >= Duration::new(0, 0));
    }

    #[test]
    fn test_instant_checked_add() {
        let instant = Instant::now();
        let duration = Duration::new(1, 0);
        let new_instant = instant.checked_add(duration).unwrap();
        assert!(new_instant > instant);
    }

    #[test]
    fn test_instant_checked_sub() {
        let instant1 = Instant::now();
        let duration = Duration::new(1, 0);
        let new_instant = instant1.checked_sub(duration).unwrap();
        assert!(new_instant < instant1);

        // check if it's still std
        assert!(matches!(new_instant.0, InstantInner::Std(_)));
    }

    #[cfg(tokio_time)]
    #[tokio::test]
    async fn test_instant_checked_sub_async() {
        let instant1 = Instant::now();
        let duration = Duration::new(1, 0);
        let new_instant = instant1.checked_sub(duration).unwrap();
        assert!(new_instant < instant1);

        // check if it's still tokio
        assert!(matches!(new_instant.0, InstantInner::Tokio(_)));
    }

    #[test]
    fn test_instant_duration_since() {
        let instant1 = Instant::now();
        let instant2 = Instant::now();
        let duration = instant1.duration_since(instant2);
        assert!(duration >= Duration::new(0, 0));
    }

    #[test]
    fn test_instant_checked_duration_since() {
        let instant2 = Instant::now();
        std::thread::sleep(Duration::from_millis(100));
        let instant1 = Instant::now();
        let duration = instant1.checked_duration_since(instant2);
        assert!(duration.is_some());
        assert!(duration.unwrap() >= Duration::new(0, 0));
    }

    #[test]
    fn test_instant_saturating_duration_since() {
        let instant1 = Instant::now();
        let instant2 = Instant::now();
        let duration = instant1.saturating_duration_since(instant2);
        assert!(duration >= Duration::new(0, 0));
    }

    #[test]
    fn test_instant_elapsed() {
        let instant = Instant::now();
        std::thread::sleep(Duration::from_millis(100));
        let elapsed = instant.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[cfg(tokio_time)]
    #[tokio::test]
    async fn test_instant_elapsed_async() {
        let instant = Instant::now();
        tokio::time::sleep(Duration::from_millis(100)).await;
        let elapsed = instant.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[test]
    fn test_instant_now() {
        let instant = Instant::now();
        assert!(instant.elapsed() >= Duration::new(0, 0));

        assert!(matches!(instant.0, InstantInner::Std(_)));
    }

    #[cfg(tokio_time)]
    #[tokio::test]
    async fn test_instant_now_async() {
        let instant = Instant::now();
        assert!(instant.elapsed() >= Duration::new(0, 0));

        assert!(matches!(instant.0, InstantInner::Tokio(_)));
    }

    #[test]
    fn test_instant_checked_add_none() {
        let instant = Instant::now();
        let duration = Duration::new(u64::MAX, 0);
        let new_instant = instant.checked_add(duration);
        assert!(new_instant.is_none());
    }

    #[test]
    fn test_instant_checked_sub_none() {
        let instant = Instant::now();
        let duration = Duration::new(u64::MAX, 0);
        let new_instant = instant.checked_sub(duration);
        assert!(new_instant.is_none());
    }

    #[cfg(tokio_time)]
    #[tokio::test]
    async fn test_instant_checked_sub_async_none() {
        let instant = Instant::now();
        let duration = Duration::new(u64::MAX, 0);
        let new_instant = instant.checked_sub(duration);
        assert!(new_instant.is_none());
    }

    #[test]
    fn test_instant_saturating_duration_since_zero() {
        let instant = Instant::now();
        let duration = instant.saturating_duration_since(instant);
        assert_eq!(duration, Duration::new(0, 0));
    }

    #[test]
    fn test_instant_saturating_duration_since_future() {
        let instant1 = Instant::now();
        let instant2 = Instant::now() + Duration::new(1, 0);
        let duration = instant1.saturating_duration_since(instant2);
        assert_eq!(duration, Duration::new(0, 0));
    }
}
