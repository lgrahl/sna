use std::fmt;
use std::ops::{Add, AddAssign};
use std::cmp::Ordering;
use std::num::Wrapping;

/// Provides an implementation of a Serial Number as defined by
/// [RFC 1982](https://tools.ietf.org/html/rfc1982).
///
/// There are only two operations:
///
/// * `+` will apply addition of a positive integer modulo the largest
///   representable number of this type (e.g. it will *wrap* when
///   overflowing).
/// * Partial equality operators are defined but may lead to surprising
/// results, so make sure you've read
/// [chapter 3.2 of RFC 1982](https://tools.ietf.org/html/rfc1982#section-3.2).
///
/// # Examples
///
/// ```
/// use sna::SerialNumber;
///
/// let zero = SerialNumber(0u8);
/// let one = SerialNumber(1u8);
///
/// assert_eq!(0u8, one + 255u8);
/// assert!(zero > 255u8);
/// ```
#[derive(PartialEq, Clone, Copy, Hash)]
pub struct SerialNumber<T>(pub T); // TODO: Can we limit this to the types defined below?

impl<T: fmt::Debug> fmt::Debug for SerialNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for SerialNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Binary> fmt::Binary for SerialNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Octal> fmt::Octal for SerialNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for SerialNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for SerialNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! uint_half {
    ($x:expr) => (1 << ($x-1));
}

macro_rules! uint_impl {
    ($m:ident, $T:ty, $BITS:expr) => {
        impl From<$T> for SerialNumber<$T> {
            /// Convert from this integer type into a `SerialNumber`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// assert_eq!(SerialNumber::from(255u8), SerialNumber(255u8));
            /// ```
            #[inline]
            fn from(number: $T) -> Self {
                SerialNumber(number)
            }
        }

        impl From<SerialNumber<$T>> for $T {
            /// Convert from a `SerialNumber` into this integer type.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// assert_eq!(<u8>::from(SerialNumber(42u8)), 42u8);
            /// ```
            #[inline]
            fn from(number: SerialNumber<$T>) -> Self {
                number.0
            }
        }

        impl Add for SerialNumber<$T> {
            type Output = SerialNumber<$T>;

            /// Apply addition of a positive integer modulo the largest
            /// possible representable number of this type (e.g. it
            /// will *wrap* when overflowing).
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// assert_eq!(SerialNumber(254u8) + SerialNumber(4), 2u8);
            /// ```
            #[inline]
            fn add(self, other: SerialNumber<$T>) -> SerialNumber<$T> {
                SerialNumber((Wrapping(self.0) + Wrapping(other.0)).0)
            }
        }

        impl Add<$T> for SerialNumber<$T> {
            type Output = SerialNumber<$T>;

            /// Apply addition of a positive integer modulo the largest
            /// possible representable number of this type (e.g. it
            /// will *wrap* when overflowing).
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// assert_eq!(SerialNumber(254u8) + 4, 2u8);
            /// ```
            #[inline]
            fn add(self, other: $T) -> SerialNumber<$T> {
                SerialNumber((Wrapping(self.0) + Wrapping(other)).0)
            }
        }

        impl Add<SerialNumber<$T>> for $T {
            type Output = SerialNumber<$T>;

            /// Apply addition of a positive integer modulo the largest
            /// possible representable number of this type (e.g. it
            /// will *wrap* when overflowing).
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// assert_eq!(254u8 + SerialNumber(4), 2u8);
            /// ```
            #[inline]
            fn add(self, other: SerialNumber<$T>) -> SerialNumber<$T> {
                other + self
            }
        }

        impl AddAssign for SerialNumber<$T> {
            /// Performs the `+=` operation modulo the largest possible
            /// representable number of this type (e.g. it will *wrap*
            /// when overflowing).
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// let mut a = SerialNumber(255u8);
            /// a += SerialNumber(3u8);
            /// assert_eq!(SerialNumber(2), a);
            /// ```
            #[inline]
            fn add_assign(&mut self, other: SerialNumber<$T>) {
                *self = SerialNumber((Wrapping(self.0) + Wrapping(other.0)).0);
            }
        }

        impl AddAssign<$T> for SerialNumber<$T> {
            /// Performs the `+=` operation modulo the largest possible
            /// representable number of this type (e.g. it will *wrap*
            /// when overflowing).
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// let mut a = SerialNumber(255u8);
            /// a += 3u8;
            /// assert_eq!(SerialNumber(2), a);
            /// ```
            #[inline]
            fn add_assign(&mut self, other: $T) {
                *self = SerialNumber((Wrapping(self.0) + Wrapping(other)).0);
            }
        }

        impl PartialEq<$T> for SerialNumber<$T> {
            /// Test if `self` and `other` of this integer type are
            /// equal.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// assert_eq!(SerialNumber(42u8), 42u8);
            /// ```
            #[inline]
            fn eq(&self, other: &$T) -> bool {
                self.eq(&SerialNumber(*other))
            }
        }

        impl PartialEq<SerialNumber<$T>> for $T {
            /// Test if `self` and `other` of type `SerialNumber` are
            /// equal.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// assert_eq!(42u8, SerialNumber(42u8));
            /// ```
            #[inline]
            fn eq(&self, other: &SerialNumber<$T>) -> bool {
                other.eq(self)
            }
        }

        impl PartialOrd for SerialNumber<$T> {
            /// Return an ordering between `self` and `other`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// use std::cmp::Ordering;
            ///
            /// let zero = SerialNumber(0u8);
            /// let u8_max = SerialNumber(255u8);
            ///
            /// assert_eq!(zero.partial_cmp(&zero), Some(Ordering::Equal));
            /// assert_eq!(zero.partial_cmp(&u8_max), Some(Ordering::Greater));
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &SerialNumber<$T>) -> Option<Ordering> {
                if self.0 == other.0 {
                    Some(Ordering::Equal)
                } else if
                    (self.0 < other.0 && (other.0 - self.0) < uint_half!($BITS)) ||
                    (self.0 > other.0 && (self.0 - other.0) > uint_half!($BITS))
                {
                    Some(Ordering::Less)
                } else if
                    (self.0 < other.0 && (other.0 - self.0) > uint_half!($BITS)) ||
                    (self.0 > other.0 && (self.0 - other.0) < uint_half!($BITS))
                {
                    Some(Ordering::Greater)
                } else {
                    None
                }
            }
        }

        impl PartialOrd<$T> for SerialNumber<$T> {
            /// Return an ordering between `self` and `other` of this
            /// integer type.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// use std::cmp::Ordering;
            ///
            /// let zero = SerialNumber(0u8);
            ///
            /// assert_eq!(zero.partial_cmp(&0u8), Some(Ordering::Equal));
            /// assert_eq!(zero.partial_cmp(&255u8), Some(Ordering::Greater));
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &$T) -> Option<Ordering> {
                self.partial_cmp(&SerialNumber(*other))
            }
        }

        impl PartialOrd<SerialNumber<$T>> for $T {
            /// Return an ordering between `self` and `other` of type
            /// `SerialNumber`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sna::SerialNumber;
            /// use std::cmp::Ordering;
            ///
            /// let zero = SerialNumber(0u8);
            /// let u8_max = SerialNumber(255u8);
            ///
            /// assert_eq!(0u8.partial_cmp(&zero), Some(Ordering::Equal));
            /// assert_eq!(0u8.partial_cmp(&u8_max), Some(Ordering::Greater));
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &SerialNumber<$T>) -> Option<Ordering> {
                match other.partial_cmp(self) {
                    Some(ordering) => Some(ordering.reverse()),
                    None => None,
                }
            }
        }

        #[cfg(test)]
        mod $m {
            use super::*;

            #[test]
            fn from() {
                assert_eq!(SerialNumber::from(<$T>::max_value()), SerialNumber(<$T>::max_value()));
                assert_eq!(<$T>::from(SerialNumber(<$T>::max_value())), <$T>::max_value());
            }

            #[test]
            fn into() {
                let value: SerialNumber<$T> = <$T>::max_value().into();
                assert_eq!(SerialNumber(<$T>::max_value()), value);
                let value: $T = SerialNumber(<$T>::max_value()).into();
                assert_eq!(<$T>::max_value(), value);
            }

            #[test]
            fn add() {
                assert_eq!(0, SerialNumber(1) + SerialNumber(<$T>::max_value()));
                assert_eq!(0, <$T>::max_value() + SerialNumber(1));
                assert_eq!(0, SerialNumber(1) + <$T>::max_value());
            }

            #[test]
            fn add_assign() {
                let mut a = SerialNumber(<$T>::max_value());
                a += SerialNumber(<$T>::max_value());
                assert_eq!(SerialNumber(<$T>::max_value() - 1), a);

                let mut a = SerialNumber(<$T>::max_value());
                a += <$T>::max_value();
                assert_eq!(SerialNumber(<$T>::max_value() - 1), a);
            }

            #[test]
            fn eq() {
                let max = SerialNumber(<$T>::max_value());
                assert_eq!(max, max);
                assert_eq!(max, <$T>::max_value());
                assert_eq!(<$T>::max_value(), max);
            }

            #[test]
            fn ne() {
                let zero: SerialNumber<$T> = 0.into();
                let max = SerialNumber(<$T>::max_value());
                assert_ne!(zero, max);
                assert_ne!(zero, <$T>::max_value());
                assert_ne!(<$T>::max_value(), zero);
            }

            #[test]
            fn partial_cmp() {
                let zero: SerialNumber<$T> = 0.into();
                let half_minus_one = uint_half!($BITS) - 1;
                let half: SerialNumber<$T> = uint_half!($BITS).into();
                let max = SerialNumber(<$T>::max_value());

                // Equal
                assert_eq!(max.partial_cmp(&max), Some(Ordering::Equal));
                assert_eq!(max.partial_cmp(&<$T>::max_value()), Some(Ordering::Equal));
                assert_eq!(<$T>::max_value().partial_cmp(&max), Some(Ordering::Equal));

                // Less
                assert_eq!(max.partial_cmp(&zero), Some(Ordering::Less));
                assert_eq!(max.partial_cmp(&0), Some(Ordering::Less));
                assert_eq!(<$T>::max_value().partial_cmp(&zero), Some(Ordering::Less));

                // Greater
                assert_eq!(zero.partial_cmp(&max), Some(Ordering::Greater));
                assert_eq!(zero.partial_cmp(&<$T>::max_value()), Some(Ordering::Greater));
                assert_eq!(0.partial_cmp(&max), Some(Ordering::Greater));

                // None
                assert_eq!(zero.partial_cmp(&half), None);
                assert_eq!(1.partial_cmp(&(half + 1)), None);
                assert_eq!(half_minus_one.partial_cmp(&max), None);
            }
        }
    };
}

// Add implementations for u8, u16, u32 and u64
uint_impl!(u8, u8, 8);
uint_impl!(u16, u16, 16);
uint_impl!(u32, u32, 32);
uint_impl!(u64, u64, 64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        assert!(SerialNumber(1u8) > 0);
        assert!(SerialNumber(44u8) > 0);
        assert!(SerialNumber(100u8) > 0);
        assert!(SerialNumber(100u8) > 44);
        assert!(SerialNumber(200u8) > 100);
        assert!(SerialNumber(255u8) > 200);
        assert!(SerialNumber(0u8) > 255);
        assert!(SerialNumber(100u8) > 255);
        assert!(SerialNumber(0u8) > 200);
        assert!(SerialNumber(44u8) > 200);
    }

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", SerialNumber(33u8)), "33");
        assert_eq!(format!("{:?}", SerialNumber(33u8)), "33");
    }
}
