use core::num::{
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
pub trait Gcd {
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using [`gcd_binary`].
    ///
    /// [`gcd_binary`]: #method.gcd_binary
    ///
    /// # Examples
    ///
    /// ```
    /// use gcd::Gcd;
    ///
    /// assert_eq!(0, 0u8.gcd(0));
    /// assert_eq!(10, 10u8.gcd(0));
    /// assert_eq!(10, 0u8.gcd(10));
    /// assert_eq!(10, 10u8.gcd(20));
    /// assert_eq!(44, 2024u32.gcd(748));
    /// ```
    fn gcd(self, other: Self) -> Self;
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Binary GCD algorithm](https://en.wikipedia.org/wiki/Binary_GCD_algorithm).
    fn gcd_binary(self, other: Self) -> Self;
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
    fn gcd_euclid(self, other: Self) -> Self;
}

///Const binary GCD implementation for `u8`.
pub fn binary_u8(mut u: u8, mut v: u8) -> u8 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();
    while u != v {
        if u > v {
            let temp = u;
            u = v;
            v = temp;
        }
        v -= u;
        v >>= v.trailing_zeros();
    }
    u << shift
}

///Const euclid GCD implementation for `u8`.
pub fn euclid_u8(a: u8, b: u8) -> u8 {
    let a0 = a;
    let b0 = b;
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
    #[allow(clippy::manual_swap)] while b != 0 {
        let temp = a;
        a = b;
        b = temp;
        b %= a;
    }
    if !(a == 0 || a0 % a == 0) {
        panic!("assertion failed: a == 0 || a0 % a == 0")
    }
    if !(a == 0 || b0 % a == 0) {
        panic!("assertion failed: a == 0 || b0 % a == 0")
    }
    a
}

impl Gcd for u8 {
    #[inline]
    fn gcd(self, other: u8) -> u8 {
        self.gcd_binary(other)
    }
    #[inline]
    fn gcd_binary(self, v: u8) -> u8 {
        binary_u8(self, v)
    }
    #[inline]
    fn gcd_euclid(self, other: u8) -> u8 {
        euclid_u8(self, other)
    }
}
