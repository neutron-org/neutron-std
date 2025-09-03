use core::cmp::Ordering;
use core::fmt::{self, Write};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use core::str::FromStr;
use serde::{de, ser, Deserialize, Deserializer, Serialize};

use cosmwasm_std::{
    CheckedFromRatioError, CheckedMultiplyRatioError, DivideByZeroError, OverflowError,
    OverflowOperation, RoundUpOverflowError, StdError,
};
use crate::types::neutron::util::forward_ref::{forward_ref_binop, forward_ref_op_assign, forward_ref_partial_eq};
use cosmwasm_std::{Decimal256, SignedDecimal, SignedDecimal256};

use cosmwasm_std::{Fraction, Isqrt, Uint256, Uint512};




/// A fixed-point decimal value with 18 fractional digits, i.e. Decimal(1_000_000_000_000_000_000) == 1.0
///
/// The greatest possible value that can be represented is 340282366920938463463.374607431768211455 (which is (2^128 - 1) / 10^18)
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, schemars::JsonSchema)]
pub struct PrecDec(#[schemars(with = "String")] Uint256);

forward_ref_partial_eq!(PrecDec, PrecDec);

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Decimal range exceeded")]
pub struct DecimalRangeExceeded;

impl PrecDec {
    const DECIMAL_FRACTIONAL: Uint256 = Uint256::from_uint128(1_000_000_000_000_000_000_000_000_000u128); // 1*10**27
    // const DECIMAL_FRACTIONAL_SQUARED: Uint256 = Uint256::from_uint128(1_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000u128); // (1*10**18)**2 = 1*10**54



    /// The number of decimal places. Since decimal types are fixed-point rather than
    /// floating-point, this is a constant.
    pub const DECIMAL_PLACES: u32 = 27; // This needs to be an even number. // TODO: is it ok ?
    /// The largest value that can be represented by this decimal type.
    pub const MAX: Self = Self(Uint256::MAX);
    /// The smallest value that can be represented by this decimal type.
    pub const MIN: Self = Self(Uint256::MIN);

    /// Creates a Decimal(value)
    /// This is equivalent to `Decimal::from_atomics(value, 18)` but usable in a const context.
    pub const fn new(value: Uint256) -> Self {
        Self(value)
    }

    /// Creates a Decimal(Uint128(value))
    /// This is equivalent to `Decimal::from_atomics(value, 18)` but usable in a const context.
    pub const fn raw(value: u128) -> Self {
        Self(Uint256::from_u128(value))
    }

    /// Create a 1.0 Decimal
    #[inline]
    pub const fn one() -> Self {
        Self(Self::DECIMAL_FRACTIONAL)
    }

    /// Create a 0.0 Decimal
    #[inline]
    pub const fn zero() -> Self {
        Self(Uint256::zero())
    }

    /// Convert x% into Decimal
    ///
    /// ## Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use cosmwasm_std::Decimal;
    /// const HALF: Decimal = Decimal::percent(50);
    ///
    /// assert_eq!(HALF, Decimal::from_str("0.5").unwrap());
    /// ```
    // TODO: fix me
    // pub const fn percent(x: u64) -> Self {
    //     // multiplication does not overflow since `u64::MAX` * 10**16 is well in u128 range
    //     let atomics = (x as u128) * 10_000_000_000_000_000;
    //     Self(Uint128::new(atomics))
    // }

    /// Convert permille (x/1000) into Decimal
    ///
    /// ## Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use cosmwasm_std::Decimal;
    /// const HALF: Decimal = Decimal::permille(500);
    ///
    /// assert_eq!(HALF, Decimal::from_str("0.5").unwrap());
    /// ```

    // TODO: fix me
    // pub const fn permille(x: u64) -> Self {
    //     // multiplication does not overflow since `u64::MAX` * 10**15 is well in u128 range
    //     let atomics = (x as u128) * 1_000_000_000_000_000;
    //     Self(Uint128::new(atomics))
    // }

    /// Convert basis points (x/10000) into Decimal
    ///
    /// ## Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use cosmwasm_std::Decimal;
    /// const TWO_BPS: Decimal = Decimal::bps(2);
    /// const HALF: Decimal = Decimal::bps(5000);
    ///
    /// assert_eq!(TWO_BPS, Decimal::from_str("0.0002").unwrap());
    /// assert_eq!(HALF, Decimal::from_str("0.5").unwrap());
    /// ```

    // TODO: fix me
    // pub const fn bps(x: u64) -> Self {
    //     // multiplication does not overflow since `u64::MAX` * 10**14 is well in u128 range
    //     let atomics = (x as u128) * 100_000_000_000_000;
    //     Self(Uint128::new(atomics))
    // }

    /// Creates a decimal from a number of atomic units and the number
    /// of decimal places. The inputs will be converted internally to form
    /// a decimal with 18 decimal places. So the input 123 and 2 will create
    /// the decimal 1.23.
    ///
    /// Using 18 decimal places is slightly more efficient than other values
    /// as no internal conversion is necessary.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use cosmwasm_std::{Decimal, Uint128};
    /// let a = Decimal::from_atomics(Uint128::new(1234), 3).unwrap();
    /// assert_eq!(a.to_string(), "1.234");
    ///
    /// let a = Decimal::from_atomics(1234u128, 0).unwrap();
    /// assert_eq!(a.to_string(), "1234");
    ///
    /// let a = Decimal::from_atomics(1u64, 18).unwrap();
    /// assert_eq!(a.to_string(), "0.000000000000000001");
    /// ```
    pub fn from_atomics(
        atomics: impl Into<Uint256>,
        decimal_places: u32,
    ) -> Result<Self, DecimalRangeExceeded> {
        let atomics = atomics.into();
        const TEN: Uint256 = Uint256::from_u128(10);
        Ok(match decimal_places.cmp(&Self::DECIMAL_PLACES) {
            Ordering::Less => {
                let digits = (Self::DECIMAL_PLACES) - decimal_places; // No overflow because decimal_places < DECIMAL_PLACES
                let factor = TEN.checked_pow(digits).unwrap(); // Safe because digits <= 17 //TODO: Is still ok?
                Self(
                    atomics
                        .checked_mul(factor)
                        .map_err(|_| DecimalRangeExceeded)?,
                )
            }
            Ordering::Equal => Self(atomics),
            Ordering::Greater => {
                let digits = decimal_places - (Self::DECIMAL_PLACES); // No overflow because decimal_places > DECIMAL_PLACES
                if let Ok(factor) = TEN.checked_pow(digits) {
                    Self(atomics.checked_div(factor).unwrap()) // Safe because factor cannot be zero
                } else {
                    // In this case `factor` exceeds the Uint128 range.
                    // Any Uint128 `x` divided by `factor` with `factor > Uint128::MAX` is 0.
                    // Try e.g. Python3: `(2**128-1) // 2**128`
                    Self(Uint256::zero())
                }
            }
        })
    }

    /// Returns the ratio (numerator / denominator) as a Decimal
    pub fn from_ratio(numerator: impl Into<Uint256>, denominator: impl Into<Uint256>) -> Self {
        match PrecDec::checked_from_ratio(numerator, denominator) {
            Ok(value) => value,
            Err(CheckedFromRatioError::DivideByZero) => {
                panic!("Denominator must not be zero")
            }
            Err(CheckedFromRatioError::Overflow) => panic!("Multiplication overflow"),
        }
    }

    /// Returns the ratio (numerator / denominator) as a Decimal
    pub fn checked_from_ratio(
        numerator: impl Into<Uint256>,
        denominator: impl Into<Uint256>,
    ) -> Result<Self, CheckedFromRatioError> {
        let numerator: Uint256 = numerator.into();
        let denominator: Uint256 = denominator.into();
        match numerator.checked_multiply_ratio(Self::DECIMAL_FRACTIONAL, denominator) {
            Ok(ratio) => {
                // numerator * DECIMAL_FRACTIONAL / denominator
                Ok(PrecDec(ratio))
            }
            Err(CheckedMultiplyRatioError::Overflow) => Err(CheckedFromRatioError::Overflow),
            Err(CheckedMultiplyRatioError::DivideByZero) => {
                Err(CheckedFromRatioError::DivideByZero)
            }
        }
    }

    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// A decimal is an integer of atomic units plus a number that specifies the
    /// position of the decimal dot. So any decimal can be expressed as two numbers.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use cosmwasm_std::{Decimal, Uint128};
    /// # use core::str::FromStr;
    /// // Value with whole and fractional part
    /// let a = Decimal::from_str("1.234").unwrap();
    /// assert_eq!(a.decimal_places(), 18);
    /// assert_eq!(a.atomics(), Uint128::new(1234000000000000000));
    ///
    /// // Smallest possible value
    /// let b = Decimal::from_str("0.000000000000000001").unwrap();
    /// assert_eq!(b.decimal_places(), 18);
    /// assert_eq!(b.atomics(), Uint128::new(1));
    /// ```
    #[must_use]
    #[inline]
    pub const fn atomics(&self) -> Uint256 {
        self.0
    }

    /// The number of decimal places. This is a constant value for now
    /// but this could potentially change as the type evolves.
    ///
    /// See also [`Decimal::atomics()`].
    #[must_use]
    #[inline]
    pub const fn decimal_places(&self) -> u32 {
        Self::DECIMAL_PLACES
    }

    /// Rounds value down after decimal places.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn floor(&self) -> Self {
        Self((self.0 / Self::DECIMAL_FRACTIONAL) * Self::DECIMAL_FRACTIONAL)
    }

    /// Rounds value up after decimal places. Panics on overflow.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn ceil(&self) -> Self {
        match self.checked_ceil() {
            Ok(value) => value,
            Err(_) => panic!("attempt to ceil with overflow"),
        }
    }

    /// Rounds value up after decimal places. Returns OverflowError on overflow.
    pub fn checked_ceil(&self) -> Result<Self, RoundUpOverflowError> {
        let floor = self.floor();
        if floor == self {
            Ok(floor)
        } else {
            floor
                .checked_add(PrecDec::one())
                .map_err(|_| RoundUpOverflowError)
        }
    }

    pub fn checked_add(self, other: Self) -> Result<Self, OverflowError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .map_err(|_| OverflowError::new(OverflowOperation::Add))
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, OverflowError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .map_err(|_| OverflowError::new(OverflowOperation::Sub))
    }

    //TODO: fix me, overflow check doesn't work
    /// Multiplies one `Decimal` by another, returning an `OverflowError` if an overflow occurred.
    pub fn checked_mul(self, other: Self) -> Result<Self, OverflowError> {
        let result_as_uint256 = self.numerator().full_mul(other.numerator())
            / Uint512::from_uint256(Self::DECIMAL_FRACTIONAL) ; // from_uint128 is a const method and should be "free"
        result_as_uint256
            .try_into()
            .map(Self)
            .map_err(|_| OverflowError::new(OverflowOperation::Mul))
    }

    /// Raises a value to the power of `exp`, panics if an overflow occurred.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn pow(self, exp: u32) -> Self {
        match self.checked_pow(exp) {
            Ok(value) => value,
            Err(_) => panic!("Multiplication overflow"),
        }
    }

    /// Raises a value to the power of `exp`, returning an `OverflowError` if an overflow occurred.
    pub fn checked_pow(self, exp: u32) -> Result<Self, OverflowError> {
        // This uses the exponentiation by squaring algorithm:
        // https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Basic_method

        fn inner(mut x: PrecDec, mut n: u32) -> Result<PrecDec, OverflowError> {
            if n == 0 {
                return Ok(PrecDec::one());
            }

            let mut y = PrecDec::one();

            while n > 1 {
                if n % 2 == 0 {
                    x = x.checked_mul(x)?;
                    n /= 2;
                } else {
                    y = x.checked_mul(y)?;
                    x = x.checked_mul(x)?;
                    n = (n - 1) / 2;
                }
            }

            Ok(x * y)
        }

        inner(self, exp).map_err(|_| OverflowError::new(OverflowOperation::Pow))
    }

    pub fn checked_div(self, other: Self) -> Result<Self, CheckedFromRatioError> {
        PrecDec::checked_from_ratio(self.numerator(), other.numerator())
    }

    pub fn checked_rem(self, other: Self) -> Result<Self, DivideByZeroError> {
        self.0
            .checked_rem(other.0)
            .map(Self)
            .map_err(|_| DivideByZeroError)
    }

    /// Returns the approximate square root as a Decimal.
    ///
    /// This should not overflow or panic.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn sqrt(&self) -> Self {
        // Algorithm described in https://hackmd.io/@webmaster128/SJThlukj_
        // We start with the highest precision possible and lower it until
        // there's no overflow.
        //
        // TODO: This could be made more efficient once log10 is in:
        // https://github.com/rust-lang/rust/issues/70887
        // The max precision is something like `9 - log10(self.0) / 2`.
        (0..=Self::DECIMAL_PLACES / 2)
            .rev()
            .find_map(|i| self.sqrt_with_precision(i))
            // The last step (i = 0) is guaranteed to succeed because `isqrt(u128::MAX) * 10^9` does not overflow
            .unwrap()
    }

    /// Lower precision means more aggressive rounding, but less risk of overflow.
    /// Precision *must* be a number between 0 and 9 (inclusive).
    ///
    /// Returns `None` if the internal multiplication overflows.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sqrt_with_precision(&self, precision: u32) -> Option<Self> {
        let inner_mul = 100u128.pow(precision);
        self.0.checked_mul(inner_mul.into()).ok().map(|inner| {
            let outer_mul = 10u128.pow(Self::DECIMAL_PLACES / 2 - precision);
            PrecDec(inner.isqrt().checked_mul(Uint256::from(outer_mul)).unwrap())
        })
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn abs_diff(self, other: Self) -> Self {
        Self(self.0.abs_diff(other.0))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn saturating_add(self, other: Self) -> Self {
        match self.checked_add(other) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn saturating_sub(self, other: Self) -> Self {
        match self.checked_sub(other) {
            Ok(value) => value,
            Err(_) => Self::zero(),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn saturating_mul(self, other: Self) -> Self {
        match self.checked_mul(other) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn saturating_pow(self, exp: u32) -> Self {
        match self.checked_pow(exp) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Converts this decimal to an unsigned integer by truncating
    /// the fractional part, e.g. 22.5 becomes 22.
    ///
    /// ## Examples
    ///
    /// ```
    /// use core::str::FromStr;
    /// use cosmwasm_std::{Decimal, Uint128};
    ///
    /// let d = Decimal::from_str("12.345").unwrap();
    /// assert_eq!(d.to_uint_floor(), Uint128::new(12));
    ///
    /// let d = Decimal::from_str("12.999").unwrap();
    /// assert_eq!(d.to_uint_floor(), Uint128::new(12));
    ///
    /// let d = Decimal::from_str("75.0").unwrap();
    /// assert_eq!(d.to_uint_floor(), Uint128::new(75));
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn to_uint_floor(self) -> Uint256 {
        self.0 / Self::DECIMAL_FRACTIONAL
    }

    /// Converts this decimal to an unsigned integer by rounting up
    /// to the next integer, e.g. 22.3 becomes 23.
    ///
    /// ## Examples
    ///
    /// ```
    /// use core::str::FromStr;
    /// use cosmwasm_std::{Decimal, Uint128};
    ///
    /// let d = Decimal::from_str("12.345").unwrap();
    /// assert_eq!(d.to_uint_ceil(), Uint128::new(13));
    ///
    /// let d = Decimal::from_str("12.999").unwrap();
    /// assert_eq!(d.to_uint_ceil(), Uint128::new(13));
    ///
    /// let d = Decimal::from_str("75.0").unwrap();
    /// assert_eq!(d.to_uint_ceil(), Uint128::new(75));
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn to_uint_ceil(self) -> Uint256 {
        // Using `q = 1 + ((x - 1) / y); // if x != 0` with unsigned integers x, y, q
        // from https://stackoverflow.com/a/2745086/2013738. We know `x + y` CAN overflow.
        let x = self.0;
        let y = Self::DECIMAL_FRACTIONAL;
        if x.is_zero() {
            Uint256::zero()
        } else {
            Uint256::one() + ((x - Uint256::one()) / y)
        }
    }
}

impl Fraction<Uint256> for PrecDec {
    #[inline]
    fn numerator(&self) -> Uint256 {
        self.0
    }

    #[inline]
    fn denominator(&self) -> Uint256 {
        Self::DECIMAL_FRACTIONAL
    }

    /// Returns the multiplicative inverse `1/d` for decimal `d`.
    ///
    /// If `d` is zero, none is returned.
    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            let decimal_fractional_squared = Self::DECIMAL_FRACTIONAL.Mul(Self::DECIMAL_FRACTIONAL);
            // Let self be p/q with p = self.0 and q = DECIMAL_FRACTIONAL.
            // Now we calculate the inverse a/b = q/p such that b = DECIMAL_FRACTIONAL. Then
            // `a = DECIMAL_FRACTIONAL*DECIMAL_FRACTIONAL / self.0`.
            Some(PrecDec(decimal_fractional_squared / self.0))
        }
    }
}

impl TryFrom<Decimal256> for PrecDec {
    type Error = DecimalRangeExceeded;

    fn try_from(value: Decimal256) -> Result<Self, Self::Error> {
        value
            .atomics()
            .try_into()
            .map(PrecDec)
            .map_err(|_| DecimalRangeExceeded)
    }
}

impl TryFrom<SignedDecimal> for PrecDec {
    type Error = DecimalRangeExceeded;

    fn try_from(value: SignedDecimal) -> Result<Self, Self::Error> {
        value
            .atomics()
            .try_into()
            .map(PrecDec)
            .map_err(|_| DecimalRangeExceeded)
    }
}

impl TryFrom<SignedDecimal256> for PrecDec {
    type Error = DecimalRangeExceeded;

    fn try_from(value: SignedDecimal256) -> Result<Self, Self::Error> {
        value
            .atomics()
            .try_into()
            .map(PrecDec)
            .map_err(|_| DecimalRangeExceeded)
    }
}

impl TryFrom<Uint256> for PrecDec {
    type Error = DecimalRangeExceeded;

    #[inline]
    fn try_from(value: Uint256) -> Result<Self, Self::Error> {
        Self::from_atomics(value, 0)
    }
}

impl FromStr for PrecDec {
    type Err = StdError;

    /// Converts the decimal string to a Decimal
    /// Possible inputs: "1.23", "1", "000012", "1.123000000"
    /// Disallowed: "", ".23"
    ///
    /// This never performs any kind of rounding.
    /// More than DECIMAL_PLACES fractional digits, even zeros, result in an error.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts_iter = input.split('.');

        let whole_part = parts_iter.next().unwrap(); // split always returns at least one element
        let whole = whole_part
            .parse::<Uint256>()
            .map_err(|_| StdError::generic_err("Error parsing whole"))?;
        let mut atomics = whole
            .checked_mul(Self::DECIMAL_FRACTIONAL)
            .map_err(|_| StdError::generic_err("Value too big"))?;

        if let Some(fractional_part) = parts_iter.next() {
            let fractional = fractional_part
                .parse::<Uint256>()
                .map_err(|_| StdError::generic_err("Error parsing fractional"))?;
            let exp = (Self::DECIMAL_PLACES.checked_sub(fractional_part.len() as u32)).ok_or_else(
                || {
                    StdError::generic_err(format!(
                        "Cannot parse more than {} fractional digits",
                        Self::DECIMAL_PLACES
                    ))
                },
            )?;
            debug_assert!(exp <= Self::DECIMAL_PLACES);
            let fractional_factor = Uint256::from(10u128.pow(exp));
            atomics = atomics
                .checked_add(
                    // The inner multiplication can't overflow because
                    // fractional < 10^DECIMAL_PLACES && fractional_factor <= 10^DECIMAL_PLACES
                    fractional.checked_mul(fractional_factor).unwrap(),
                )
                .map_err(|_| StdError::generic_err("Value too big"))?;
        }

        if parts_iter.next().is_some() {
            return Err(StdError::generic_err("Unexpected number of dots"));
        }

        Ok(PrecDec(atomics))
    }
}

impl fmt::Display for PrecDec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let whole = (self.0) / Self::DECIMAL_FRACTIONAL;
        let fractional = (self.0).checked_rem(Self::DECIMAL_FRACTIONAL).unwrap();

        if fractional.is_zero() {
            write!(f, "{whole}")
        } else {
            let fractional_string = format!(
                "{:0>padding$}",
                fractional,
                padding = Self::DECIMAL_PLACES as usize
            );
            f.write_str(&whole.to_string())?;
            f.write_char('.')?;
            f.write_str(fractional_string.trim_end_matches('0'))?;
            Ok(())
        }
    }
}

impl fmt::Debug for PrecDec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decimal({self})")
    }
}

impl Add for PrecDec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        PrecDec(self.0 + other.0)
    }
}
forward_ref_binop!(impl Add, add for PrecDec, PrecDec);

impl AddAssign for PrecDec {
    fn add_assign(&mut self, rhs: PrecDec) {
        *self = *self + rhs;
    }
}
forward_ref_op_assign!(impl AddAssign, add_assign for PrecDec, PrecDec);

impl Sub for PrecDec {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        PrecDec(self.0 - other.0)
    }
}
forward_ref_binop!(impl Sub, sub for PrecDec, PrecDec);

impl SubAssign for PrecDec {
    fn sub_assign(&mut self, rhs: PrecDec) {
        *self = *self - rhs;
    }
}
forward_ref_op_assign!(impl SubAssign, sub_assign for PrecDec, PrecDec);

impl Mul for PrecDec {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, other: Self) -> Self {
        // Decimals are fractions. We can multiply two decimals a and b
        // via
        //       (a.numerator() * b.numerator()) / (a.denominator() * b.denominator())
        //     = (a.numerator() * b.numerator()) / a.denominator() / b.denominator()

        let result_as_uint256 = self.numerator().full_mul(other.numerator())
            /  Uint512::from_uint256(Self::DECIMAL_FRACTIONAL);  // from_uint128 is a const method and should be "free" // TODO: fix me, this can still overflow
        match result_as_uint256.try_into() {
            Ok(result) => Self(result),
            Err(_) => panic!("attempt to multiply with overflow"),
        }
    }
}
forward_ref_binop!(impl Mul, mul for PrecDec, PrecDec);

impl MulAssign for PrecDec {
    fn mul_assign(&mut self, rhs: PrecDec) {
        *self = *self * rhs;
    }
}
forward_ref_op_assign!(impl MulAssign, mul_assign for PrecDec, PrecDec);

impl Div for PrecDec {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match PrecDec::checked_from_ratio(self.numerator(), other.numerator()) {
            Ok(ratio) => ratio,
            Err(CheckedFromRatioError::DivideByZero) => {
                panic!("Division failed - denominator must not be zero")
            }
            Err(CheckedFromRatioError::Overflow) => {
                panic!("Division failed - multiplication overflow")
            }
        }
    }
}
forward_ref_binop!(impl Div, div for PrecDec, PrecDec);

impl DivAssign for PrecDec {
    fn div_assign(&mut self, rhs: PrecDec) {
        *self = *self / rhs;
    }
}
forward_ref_op_assign!(impl DivAssign, div_assign for PrecDec, PrecDec);

impl Div<Uint256> for PrecDec {
    type Output = Self;

    fn div(self, rhs: Uint256) -> Self::Output {
        PrecDec(self.0 / rhs)
    }
}

impl DivAssign<Uint256> for PrecDec {
    fn div_assign(&mut self, rhs: Uint256) {
        self.0 /= rhs;
    }
}

impl Rem for PrecDec {
    type Output = Self;

    /// # Panics
    ///
    /// This operation will panic if `rhs` is zero
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self(self.0.rem(rhs.0))
    }
}
forward_ref_binop!(impl Rem, rem for PrecDec, PrecDec);

impl RemAssign<PrecDec> for PrecDec {
    fn rem_assign(&mut self, rhs: PrecDec) {
        *self = *self % rhs;
    }
}
forward_ref_op_assign!(impl RemAssign, rem_assign for PrecDec, PrecDec);

impl<A> core::iter::Sum<A> for PrecDec
where
    Self: Add<A, Output = Self>,
{
    fn sum<I: Iterator<Item = A>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

/// Serializes as a decimal string
impl Serialize for PrecDec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Deserializes as a base64 string
impl<'de> Deserialize<'de> for PrecDec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DecimalVisitor)
    }
}

struct DecimalVisitor;

impl<'de> de::Visitor<'de> for DecimalVisitor {
    type Value = PrecDec;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string-encoded decimal")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match PrecDec::from_str(v) {
            Ok(d) => Ok(d),
            Err(e) => Err(E::custom(format_args!("Error parsing decimal '{v}': {e}"))),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn dec(input: &str) -> PrecDec {
//         PrecDec::from_str(input).unwrap()
//     }

//     #[test]
//     fn decimal_new() {
//         let expected = Uint256::from(300u128);
//         assert_eq!(PrecDec::new(expected).0, expected);
//     }

//     #[test]
//     fn decimal_raw() {
//         let value = 300u128;
//         assert_eq!(PrecDec::raw(value).0.u128(), value);
//     }

//     #[test]
//     fn decimal_one() {
//         let value = PrecDec::one();
//         assert_eq!(value.0, PrecDec::DECIMAL_FRACTIONAL);
//     }

//     #[test]
//     fn decimal_zero() {
//         let value = PrecDec::zero();
//         assert!(value.0.is_zero());
//     }

//     #[test]
//     fn decimal_percent() {
//         let value = PrecDec::percent(50);
//         assert_eq!(value.0, PrecDec::DECIMAL_FRACTIONAL / Uint256::from(2u8));
//     }

//     #[test]
//     fn decimal_permille() {
//         let value = PrecDec::permille(125);
//         assert_eq!(value.0, PrecDec::DECIMAL_FRACTIONAL / Uint256::from(8u8));
//     }

//     #[test]
//     fn decimal_bps() {
//         let value = PrecDec::bps(125);
//         assert_eq!(value.0, PrecDec::DECIMAL_FRACTIONAL / Uint256::from(80u8));
//     }

//     #[test]
//     fn decimal_from_decimal256_works() {
//         let too_big = Decimal256::new(Uint256::from(Uint256::MAX) + Uint256::one());
//         assert_eq!(PrecDec::try_from(too_big), Err(DecimalRangeExceeded));

//         let just_right = Decimal256::new(Uint256::from(Uint256::MAX));
//         assert_eq!(PrecDec::try_from(just_right), Ok(PrecDec::MAX));

//         assert_eq!(PrecDec::try_from(Decimal256::zero()), Ok(PrecDec::zero()));
//         assert_eq!(PrecDec::try_from(Decimal256::one()), Ok(PrecDec::one()));
//         assert_eq!(
//             PrecDec::try_from(Decimal256::percent(50)),
//             Ok(PrecDec::percent(50))
//         );
//     }

//     #[test]
//     fn decimal_try_from_integer() {
//         let int = Uint256::new(0xDEADBEEF);
//         let decimal = PrecDec::try_from(int).unwrap();
//         assert_eq!(int.to_string(), decimal.to_string());
//     }

//     #[test]
//     fn decimal_try_from_signed_works() {
//         assert_eq!(
//             PrecDec::try_from(SignedDecimal::MAX).unwrap(),
//             PrecDec::raw(SignedDecimal::MAX.atomics().i128() as u128)
//         );
//         assert_eq!(
//             PrecDec::try_from(SignedDecimal::zero()).unwrap(),
//             PrecDec::zero()
//         );
//         assert_eq!(
//             PrecDec::try_from(SignedDecimal::one()).unwrap(),
//             PrecDec::one()
//         );
//         assert_eq!(
//             PrecDec::try_from(SignedDecimal::percent(50)).unwrap(),
//             PrecDec::percent(50)
//         );
//         assert_eq!(
//             PrecDec::try_from(SignedDecimal::negative_one()),
//             Err(DecimalRangeExceeded)
//         );
//         assert_eq!(
//             PrecDec::try_from(SignedDecimal::MIN),
//             Err(DecimalRangeExceeded)
//         );
//     }

//     #[test]
//     fn decimal_from_atomics_works() {
//         let one = PrecDec::one();
//         let two = one + one;

//         assert_eq!(PrecDec::from_atomics(1u128, 0).unwrap(), one);
//         assert_eq!(PrecDec::from_atomics(10u128, 1).unwrap(), one);
//         assert_eq!(PrecDec::from_atomics(100u128, 2).unwrap(), one);
//         assert_eq!(PrecDec::from_atomics(1000u128, 3).unwrap(), one);
//         assert_eq!(
//             PrecDec::from_atomics(1000000000000000000u128, 18).unwrap(),
//             one
//         );
//         assert_eq!(
//             PrecDec::from_atomics(10000000000000000000u128, 19).unwrap(),
//             one
//         );
//         assert_eq!(
//             PrecDec::from_atomics(100000000000000000000u128, 20).unwrap(),
//             one
//         );

//         assert_eq!(PrecDec::from_atomics(2u128, 0).unwrap(), two);
//         assert_eq!(PrecDec::from_atomics(20u128, 1).unwrap(), two);
//         assert_eq!(PrecDec::from_atomics(200u128, 2).unwrap(), two);
//         assert_eq!(PrecDec::from_atomics(2000u128, 3).unwrap(), two);
//         assert_eq!(
//             PrecDec::from_atomics(2000000000000000000u128, 18).unwrap(),
//             two
//         );
//         assert_eq!(
//             PrecDec::from_atomics(20000000000000000000u128, 19).unwrap(),
//             two
//         );
//         assert_eq!(
//             PrecDec::from_atomics(200000000000000000000u128, 20).unwrap(),
//             two
//         );

//         // Cuts decimal digits (20 provided but only 18 can be stored)
//         assert_eq!(
//             PrecDec::from_atomics(4321u128, 20).unwrap(),
//             PrecDec::from_str("0.000000000000000043").unwrap()
//         );
//         assert_eq!(
//             PrecDec::from_atomics(6789u128, 20).unwrap(),
//             PrecDec::from_str("0.000000000000000067").unwrap()
//         );
//         assert_eq!(
//             PrecDec::from_atomics(u128::MAX, 38).unwrap(),
//             PrecDec::from_str("3.402823669209384634").unwrap()
//         );
//         assert_eq!(
//             PrecDec::from_atomics(u128::MAX, 39).unwrap(),
//             PrecDec::from_str("0.340282366920938463").unwrap()
//         );
//         assert_eq!(
//             PrecDec::from_atomics(u128::MAX, 45).unwrap(),
//             PrecDec::from_str("0.000000340282366920").unwrap()
//         );
//         assert_eq!(
//             PrecDec::from_atomics(u128::MAX, 51).unwrap(),
//             PrecDec::from_str("0.000000000000340282").unwrap()
//         );
//         assert_eq!(
//             PrecDec::from_atomics(u128::MAX, 56).unwrap(),
//             PrecDec::from_str("0.000000000000000003").unwrap()
//         );
//         assert_eq!(
//             PrecDec::from_atomics(u128::MAX, 57).unwrap(),
//             PrecDec::from_str("0.000000000000000000").unwrap()
//         );
//         assert_eq!(
//             PrecDec::from_atomics(u128::MAX, u32::MAX).unwrap(),
//             PrecDec::from_str("0.000000000000000000").unwrap()
//         );

//         // Can be used with max value
//         let max = PrecDec::MAX;
//         assert_eq!(
//             PrecDec::from_atomics(max.atomics(), max.decimal_places()).unwrap(),
//             max
//         );

//         // Overflow is only possible with digits < 18
//         let result = PrecDec::from_atomics(u128::MAX, 17);
//         assert_eq!(result.unwrap_err(), DecimalRangeExceeded);
//     }

//     #[test]
//     fn decimal_from_ratio_works() {
//         // 1.0
//         assert_eq!(PrecDec::from_ratio(1u128, 1u128), PrecDec::one());
//         assert_eq!(PrecDec::from_ratio(53u128, 53u128), PrecDec::one());
//         assert_eq!(PrecDec::from_ratio(125u128, 125u128), PrecDec::one());

//         // 1.5
//         assert_eq!(PrecDec::from_ratio(3u128, 2u128), PrecDec::percent(150));
//         assert_eq!(PrecDec::from_ratio(150u128, 100u128), PrecDec::percent(150));
//         assert_eq!(PrecDec::from_ratio(333u128, 222u128), PrecDec::percent(150));

//         // 0.125
//         assert_eq!(PrecDec::from_ratio(1u64, 8u64), PrecDec::permille(125));
//         assert_eq!(PrecDec::from_ratio(125u64, 1000u64), PrecDec::permille(125));

//         // 1/3 (result floored)
//         assert_eq!(
//             PrecDec::from_ratio(1u64, 3u64),
//             PrecDec(Uint256::from(333_333_333_333_333_333u128))
//         );

//         // 2/3 (result floored)
//         assert_eq!(
//             PrecDec::from_ratio(2u64, 3u64),
//             PrecDec(Uint256::from(666_666_666_666_666_666u128))
//         );

//         // large inputs
//         assert_eq!(PrecDec::from_ratio(0u128, u128::MAX), PrecDec::zero());
//         assert_eq!(PrecDec::from_ratio(u128::MAX, u128::MAX), PrecDec::one());
//         // 340282366920938463463 is the largest integer <= Decimal::MAX
//         assert_eq!(
//             PrecDec::from_ratio(340282366920938463463u128, 1u128),
//             PrecDec::from_str("340282366920938463463").unwrap()
//         );
//     }

//     #[test]
//     #[should_panic(expected = "Denominator must not be zero")]
//     fn decimal_from_ratio_panics_for_zero_denominator() {
//         PrecDec::from_ratio(1u128, 0u128);
//     }

//     #[test]
//     #[should_panic(expected = "Multiplication overflow")]
//     fn decimal_from_ratio_panics_for_mul_overflow() {
//         PrecDec::from_ratio(u128::MAX, 1u128);
//     }

//     #[test]
//     fn decimal_checked_from_ratio_does_not_panic() {
//         assert_eq!(
//             PrecDec::checked_from_ratio(1u128, 0u128),
//             Err(CheckedFromRatioError::DivideByZero)
//         );

//         assert_eq!(
//             PrecDec::checked_from_ratio(u128::MAX, 1u128),
//             Err(CheckedFromRatioError::Overflow)
//         );
//     }

//     #[test]
//     fn decimal_implements_fraction() {
//         let fraction = PrecDec::from_str("1234.567").unwrap();
//         assert_eq!(
//             fraction.numerator(),
//             Uint256::from(1_234_567_000_000_000_000_000u128)
//         );
//         assert_eq!(
//             fraction.denominator(),
//             Uint256::from(1_000_000_000_000_000_000u128)
//         );
//     }

//     #[test]
//     fn decimal_from_str_works() {
//         // Integers
//         assert_eq!(PrecDec::from_str("0").unwrap(), PrecDec::percent(0));
//         assert_eq!(PrecDec::from_str("1").unwrap(), PrecDec::percent(100));
//         assert_eq!(PrecDec::from_str("5").unwrap(), PrecDec::percent(500));
//         assert_eq!(PrecDec::from_str("42").unwrap(), PrecDec::percent(4200));
//         assert_eq!(PrecDec::from_str("000").unwrap(), PrecDec::percent(0));
//         assert_eq!(PrecDec::from_str("001").unwrap(), PrecDec::percent(100));
//         assert_eq!(PrecDec::from_str("005").unwrap(), PrecDec::percent(500));
//         assert_eq!(PrecDec::from_str("0042").unwrap(), PrecDec::percent(4200));

//         // Decimals
//         assert_eq!(PrecDec::from_str("1.0").unwrap(), PrecDec::percent(100));
//         assert_eq!(PrecDec::from_str("1.5").unwrap(), PrecDec::percent(150));
//         assert_eq!(PrecDec::from_str("0.5").unwrap(), PrecDec::percent(50));
//         assert_eq!(PrecDec::from_str("0.123").unwrap(), PrecDec::permille(123));

//         assert_eq!(PrecDec::from_str("40.00").unwrap(), PrecDec::percent(4000));
//         assert_eq!(PrecDec::from_str("04.00").unwrap(), PrecDec::percent(400));
//         assert_eq!(PrecDec::from_str("00.40").unwrap(), PrecDec::percent(40));
//         assert_eq!(PrecDec::from_str("00.04").unwrap(), PrecDec::percent(4));

//         // Can handle DECIMAL_PLACES fractional digits
//         assert_eq!(
//             PrecDec::from_str("7.123456789012345678").unwrap(),
//             PrecDec(Uint256::from(7123456789012345678u128))
//         );
//         assert_eq!(
//             PrecDec::from_str("7.999999999999999999").unwrap(),
//             PrecDec(Uint256::from(7999999999999999999u128))
//         );

//         // Works for documented max value
//         assert_eq!(
//             PrecDec::from_str("340282366920938463463.374607431768211455").unwrap(),
//             PrecDec::MAX
//         );
//     }

//     #[test]
//     fn decimal_from_str_errors_for_broken_whole_part() {
//         match PrecDec::from_str("").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Error parsing whole"),
//             e => panic!("Unexpected error: {e:?}"),
//         }

//         match PrecDec::from_str(" ").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Error parsing whole"),
//             e => panic!("Unexpected error: {e:?}"),
//         }

//         match PrecDec::from_str("-1").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Error parsing whole"),
//             e => panic!("Unexpected error: {e:?}"),
//         }
//     }

//     #[test]
//     fn decimal_from_str_errors_for_broken_fractional_part() {
//         match PrecDec::from_str("1.").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Error parsing fractional"),
//             e => panic!("Unexpected error: {e:?}"),
//         }

//         match PrecDec::from_str("1. ").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Error parsing fractional"),
//             e => panic!("Unexpected error: {e:?}"),
//         }

//         match PrecDec::from_str("1.e").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Error parsing fractional"),
//             e => panic!("Unexpected error: {e:?}"),
//         }

//         match PrecDec::from_str("1.2e3").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Error parsing fractional"),
//             e => panic!("Unexpected error: {e:?}"),
//         }
//     }

//     #[test]
//     fn decimal_from_str_errors_for_more_than_18_fractional_digits() {
//         match PrecDec::from_str("7.1234567890123456789").unwrap_err() {
//             StdError::GenericErr { msg, .. } => {
//                 assert_eq!(msg, "Cannot parse more than 18 fractional digits",)
//             }
//             e => panic!("Unexpected error: {e:?}"),
//         }

//         // No special rules for trailing zeros. This could be changed but adds gas cost for the happy path.
//         match PrecDec::from_str("7.1230000000000000000").unwrap_err() {
//             StdError::GenericErr { msg, .. } => {
//                 assert_eq!(msg, "Cannot parse more than 18 fractional digits")
//             }
//             e => panic!("Unexpected error: {e:?}"),
//         }
//     }

//     #[test]
//     fn decimal_from_str_errors_for_invalid_number_of_dots() {
//         match PrecDec::from_str("1.2.3").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Unexpected number of dots"),
//             e => panic!("Unexpected error: {e:?}"),
//         }

//         match PrecDec::from_str("1.2.3.4").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Unexpected number of dots"),
//             e => panic!("Unexpected error: {e:?}"),
//         }
//     }

//     #[test]
//     fn decimal_from_str_errors_for_more_than_max_value() {
//         // Integer
//         match PrecDec::from_str("340282366920938463464").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Value too big"),
//             e => panic!("Unexpected error: {e:?}"),
//         }

//         // Decimal
//         match PrecDec::from_str("340282366920938463464.0").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Value too big"),
//             e => panic!("Unexpected error: {e:?}"),
//         }
//         match PrecDec::from_str("340282366920938463463.374607431768211456").unwrap_err() {
//             StdError::GenericErr { msg, .. } => assert_eq!(msg, "Value too big"),
//             e => panic!("Unexpected error: {e:?}"),
//         }
//     }

//     #[test]
//     fn decimal_atomics_works() {
//         let zero = PrecDec::zero();
//         let one = PrecDec::one();
//         let half = PrecDec::percent(50);
//         let two = PrecDec::percent(200);
//         let max = PrecDec::MAX;

//         assert_eq!(zero.atomics(), Uint256::new(0));
//         assert_eq!(one.atomics(), Uint256::new(1000000000000000000));
//         assert_eq!(half.atomics(), Uint256::new(500000000000000000));
//         assert_eq!(two.atomics(), Uint256::new(2000000000000000000));
//         assert_eq!(max.atomics(), Uint256::MAX);
//     }

//     #[test]
//     fn decimal_decimal_places_works() {
//         let zero = PrecDec::zero();
//         let one = PrecDec::one();
//         let half = PrecDec::percent(50);
//         let two = PrecDec::percent(200);
//         let max = PrecDec::MAX;

//         assert_eq!(zero.decimal_places(), 18);
//         assert_eq!(one.decimal_places(), 18);
//         assert_eq!(half.decimal_places(), 18);
//         assert_eq!(two.decimal_places(), 18);
//         assert_eq!(max.decimal_places(), 18);
//     }

//     #[test]
//     fn decimal_is_zero_works() {
//         assert!(PrecDec::zero().is_zero());
//         assert!(PrecDec::percent(0).is_zero());
//         assert!(PrecDec::permille(0).is_zero());

//         assert!(!PrecDec::one().is_zero());
//         assert!(!PrecDec::percent(123).is_zero());
//         assert!(!PrecDec::permille(1234).is_zero());
//     }

//     #[test]
//     fn decimal_inv_works() {
//         // d = 0
//         assert_eq!(PrecDec::zero().inv(), None);

//         // d == 1
//         assert_eq!(PrecDec::one().inv(), Some(PrecDec::one()));

//         // d > 1 exact
//         assert_eq!(
//             PrecDec::from_str("2").unwrap().inv(),
//             Some(PrecDec::from_str("0.5").unwrap())
//         );
//         assert_eq!(
//             PrecDec::from_str("20").unwrap().inv(),
//             Some(PrecDec::from_str("0.05").unwrap())
//         );
//         assert_eq!(
//             PrecDec::from_str("200").unwrap().inv(),
//             Some(PrecDec::from_str("0.005").unwrap())
//         );
//         assert_eq!(
//             PrecDec::from_str("2000").unwrap().inv(),
//             Some(PrecDec::from_str("0.0005").unwrap())
//         );

//         // d > 1 rounded
//         assert_eq!(
//             PrecDec::from_str("3").unwrap().inv(),
//             Some(PrecDec::from_str("0.333333333333333333").unwrap())
//         );
//         assert_eq!(
//             PrecDec::from_str("6").unwrap().inv(),
//             Some(PrecDec::from_str("0.166666666666666666").unwrap())
//         );

//         // d < 1 exact
//         assert_eq!(
//             PrecDec::from_str("0.5").unwrap().inv(),
//             Some(PrecDec::from_str("2").unwrap())
//         );
//         assert_eq!(
//             PrecDec::from_str("0.05").unwrap().inv(),
//             Some(PrecDec::from_str("20").unwrap())
//         );
//         assert_eq!(
//             PrecDec::from_str("0.005").unwrap().inv(),
//             Some(PrecDec::from_str("200").unwrap())
//         );
//         assert_eq!(
//             PrecDec::from_str("0.0005").unwrap().inv(),
//             Some(PrecDec::from_str("2000").unwrap())
//         );
//     }

//     #[test]
//     #[allow(clippy::op_ref)]
//     fn decimal_add_works() {
//         let value = PrecDec::one() + PrecDec::percent(50); // 1.5
//         assert_eq!(
//             value.0,
//             PrecDec::DECIMAL_FRACTIONAL * Uint256::from(3u8) / Uint256::from(2u8)
//         );

//         assert_eq!(
//             PrecDec::percent(5) + PrecDec::percent(4),
//             PrecDec::percent(9)
//         );
//         assert_eq!(PrecDec::percent(5) + PrecDec::zero(), PrecDec::percent(5));
//         assert_eq!(PrecDec::zero() + PrecDec::zero(), PrecDec::zero());

//         // works for refs
//         let a = PrecDec::percent(15);
//         let b = PrecDec::percent(25);
//         let expected = PrecDec::percent(40);
//         assert_eq!(a + b, expected);
//         assert_eq!(&a + b, expected);
//         assert_eq!(a + &b, expected);
//         assert_eq!(&a + &b, expected);
//     }

//     #[test]
//     #[should_panic(expected = "attempt to add with overflow")]
//     fn decimal_add_overflow_panics() {
//         let _value = PrecDec::MAX + PrecDec::percent(50);
//     }

//     #[test]
//     fn decimal_add_assign_works() {
//         let mut a = PrecDec::percent(30);
//         a += PrecDec::percent(20);
//         assert_eq!(a, PrecDec::percent(50));

//         // works for refs
//         let mut a = PrecDec::percent(15);
//         let b = PrecDec::percent(3);
//         let expected = PrecDec::percent(18);
//         a += &b;
//         assert_eq!(a, expected);
//     }

//     #[test]
//     #[allow(clippy::op_ref)]
//     fn decimal_sub_works() {
//         let value = PrecDec::one() - PrecDec::percent(50); // 0.5
//         assert_eq!(value.0, PrecDec::DECIMAL_FRACTIONAL / Uint256::from(2u8));

//         assert_eq!(
//             PrecDec::percent(9) - PrecDec::percent(4),
//             PrecDec::percent(5)
//         );
//         assert_eq!(PrecDec::percent(16) - PrecDec::zero(), PrecDec::percent(16));
//         assert_eq!(PrecDec::percent(16) - PrecDec::percent(16), PrecDec::zero());
//         assert_eq!(PrecDec::zero() - PrecDec::zero(), PrecDec::zero());

//         // works for refs
//         let a = PrecDec::percent(13);
//         let b = PrecDec::percent(6);
//         let expected = PrecDec::percent(7);
//         assert_eq!(a - b, expected);
//         assert_eq!(&a - b, expected);
//         assert_eq!(a - &b, expected);
//         assert_eq!(&a - &b, expected);
//     }

//     #[test]
//     #[should_panic(expected = "attempt to subtract with overflow")]
//     fn decimal_sub_overflow_panics() {
//         let _value = PrecDec::zero() - PrecDec::percent(50);
//     }

//     #[test]
//     fn decimal_sub_assign_works() {
//         let mut a = PrecDec::percent(20);
//         a -= PrecDec::percent(2);
//         assert_eq!(a, PrecDec::percent(18));

//         // works for refs
//         let mut a = PrecDec::percent(33);
//         let b = PrecDec::percent(13);
//         let expected = PrecDec::percent(20);
//         a -= &b;
//         assert_eq!(a, expected);
//     }

//     #[test]
//     #[allow(clippy::op_ref)]
//     fn decimal_implements_mul() {
//         let one = PrecDec::one();
//         let two = one + one;
//         let half = PrecDec::percent(50);

//         // 1*x and x*1
//         assert_eq!(one * PrecDec::percent(0), PrecDec::percent(0));
//         assert_eq!(one * PrecDec::percent(1), PrecDec::percent(1));
//         assert_eq!(one * PrecDec::percent(10), PrecDec::percent(10));
//         assert_eq!(one * PrecDec::percent(100), PrecDec::percent(100));
//         assert_eq!(one * PrecDec::percent(1000), PrecDec::percent(1000));
//         assert_eq!(one * PrecDec::MAX, PrecDec::MAX);
//         assert_eq!(PrecDec::percent(0) * one, PrecDec::percent(0));
//         assert_eq!(PrecDec::percent(1) * one, PrecDec::percent(1));
//         assert_eq!(PrecDec::percent(10) * one, PrecDec::percent(10));
//         assert_eq!(PrecDec::percent(100) * one, PrecDec::percent(100));
//         assert_eq!(PrecDec::percent(1000) * one, PrecDec::percent(1000));
//         assert_eq!(PrecDec::MAX * one, PrecDec::MAX);

//         // double
//         assert_eq!(two * PrecDec::percent(0), PrecDec::percent(0));
//         assert_eq!(two * PrecDec::percent(1), PrecDec::percent(2));
//         assert_eq!(two * PrecDec::percent(10), PrecDec::percent(20));
//         assert_eq!(two * PrecDec::percent(100), PrecDec::percent(200));
//         assert_eq!(two * PrecDec::percent(1000), PrecDec::percent(2000));
//         assert_eq!(PrecDec::percent(0) * two, PrecDec::percent(0));
//         assert_eq!(PrecDec::percent(1) * two, PrecDec::percent(2));
//         assert_eq!(PrecDec::percent(10) * two, PrecDec::percent(20));
//         assert_eq!(PrecDec::percent(100) * two, PrecDec::percent(200));
//         assert_eq!(PrecDec::percent(1000) * two, PrecDec::percent(2000));

//         // half
//         assert_eq!(half * PrecDec::percent(0), PrecDec::percent(0));
//         assert_eq!(half * PrecDec::percent(1), PrecDec::permille(5));
//         assert_eq!(half * PrecDec::percent(10), PrecDec::percent(5));
//         assert_eq!(half * PrecDec::percent(100), PrecDec::percent(50));
//         assert_eq!(half * PrecDec::percent(1000), PrecDec::percent(500));
//         assert_eq!(PrecDec::percent(0) * half, PrecDec::percent(0));
//         assert_eq!(PrecDec::percent(1) * half, PrecDec::permille(5));
//         assert_eq!(PrecDec::percent(10) * half, PrecDec::percent(5));
//         assert_eq!(PrecDec::percent(100) * half, PrecDec::percent(50));
//         assert_eq!(PrecDec::percent(1000) * half, PrecDec::percent(500));

//         // Move left
//         let a = dec("123.127726548762582");
//         assert_eq!(a * dec("1"), dec("123.127726548762582"));
//         assert_eq!(a * dec("10"), dec("1231.27726548762582"));
//         assert_eq!(a * dec("100"), dec("12312.7726548762582"));
//         assert_eq!(a * dec("1000"), dec("123127.726548762582"));
//         assert_eq!(a * dec("1000000"), dec("123127726.548762582"));
//         assert_eq!(a * dec("1000000000"), dec("123127726548.762582"));
//         assert_eq!(a * dec("1000000000000"), dec("123127726548762.582"));
//         assert_eq!(a * dec("1000000000000000"), dec("123127726548762582"));
//         assert_eq!(a * dec("1000000000000000000"), dec("123127726548762582000"));
//         assert_eq!(dec("1") * a, dec("123.127726548762582"));
//         assert_eq!(dec("10") * a, dec("1231.27726548762582"));
//         assert_eq!(dec("100") * a, dec("12312.7726548762582"));
//         assert_eq!(dec("1000") * a, dec("123127.726548762582"));
//         assert_eq!(dec("1000000") * a, dec("123127726.548762582"));
//         assert_eq!(dec("1000000000") * a, dec("123127726548.762582"));
//         assert_eq!(dec("1000000000000") * a, dec("123127726548762.582"));
//         assert_eq!(dec("1000000000000000") * a, dec("123127726548762582"));
//         assert_eq!(dec("1000000000000000000") * a, dec("123127726548762582000"));

//         // Move right
//         let max = PrecDec::MAX;
//         assert_eq!(
//             max * dec("1.0"),
//             dec("340282366920938463463.374607431768211455")
//         );
//         assert_eq!(
//             max * dec("0.1"),
//             dec("34028236692093846346.337460743176821145")
//         );
//         assert_eq!(
//             max * dec("0.01"),
//             dec("3402823669209384634.633746074317682114")
//         );
//         assert_eq!(
//             max * dec("0.001"),
//             dec("340282366920938463.463374607431768211")
//         );
//         assert_eq!(
//             max * dec("0.000001"),
//             dec("340282366920938.463463374607431768")
//         );
//         assert_eq!(
//             max * dec("0.000000001"),
//             dec("340282366920.938463463374607431")
//         );
//         assert_eq!(
//             max * dec("0.000000000001"),
//             dec("340282366.920938463463374607")
//         );
//         assert_eq!(
//             max * dec("0.000000000000001"),
//             dec("340282.366920938463463374")
//         );
//         assert_eq!(
//             max * dec("0.000000000000000001"),
//             dec("340.282366920938463463")
//         );

//         // works for refs
//         let a = PrecDec::percent(20);
//         let b = PrecDec::percent(30);
//         let expected = PrecDec::percent(6);
//         assert_eq!(a * b, expected);
//         assert_eq!(&a * b, expected);
//         assert_eq!(a * &b, expected);
//         assert_eq!(&a * &b, expected);
//     }

//     #[test]
//     fn decimal_mul_assign_works() {
//         let mut a = PrecDec::percent(15);
//         a *= PrecDec::percent(60);
//         assert_eq!(a, PrecDec::percent(9));

//         // works for refs
//         let mut a = PrecDec::percent(50);
//         let b = PrecDec::percent(20);
//         a *= &b;
//         assert_eq!(a, PrecDec::percent(10));
//     }

//     #[test]
//     #[should_panic(expected = "attempt to multiply with overflow")]
//     fn decimal_mul_overflow_panics() {
//         let _value = PrecDec::MAX * PrecDec::percent(101);
//     }

//     #[test]
//     fn decimal_checked_mul() {
//         let test_data = [
//             (PrecDec::zero(), PrecDec::zero()),
//             (PrecDec::zero(), PrecDec::one()),
//             (PrecDec::one(), PrecDec::zero()),
//             (PrecDec::percent(10), PrecDec::zero()),
//             (PrecDec::percent(10), PrecDec::percent(5)),
//             (PrecDec::MAX, PrecDec::one()),
//             (PrecDec::MAX / Uint256::new(2), PrecDec::percent(200)),
//             (PrecDec::permille(6), PrecDec::permille(13)),
//         ];

//         // The regular core::ops::Mul is our source of truth for these tests.
//         for (x, y) in test_data.into_iter() {
//             assert_eq!(x * y, x.checked_mul(y).unwrap());
//         }
//     }

//     #[test]
//     fn decimal_checked_mul_overflow() {
//         assert_eq!(
//             PrecDec::MAX.checked_mul(PrecDec::percent(200)),
//             Err(OverflowError::new(OverflowOperation::Mul))
//         );
//     }

//     #[test]
//     #[allow(clippy::op_ref)]
//     fn decimal_implements_div() {
//         let one = PrecDec::one();
//         let two = one + one;
//         let half = PrecDec::percent(50);

//         // 1/x and x/1
//         assert_eq!(one / PrecDec::percent(1), PrecDec::percent(10_000));
//         assert_eq!(one / PrecDec::percent(10), PrecDec::percent(1_000));
//         assert_eq!(one / PrecDec::percent(100), PrecDec::percent(100));
//         assert_eq!(one / PrecDec::percent(1000), PrecDec::percent(10));
//         assert_eq!(PrecDec::percent(0) / one, PrecDec::percent(0));
//         assert_eq!(PrecDec::percent(1) / one, PrecDec::percent(1));
//         assert_eq!(PrecDec::percent(10) / one, PrecDec::percent(10));
//         assert_eq!(PrecDec::percent(100) / one, PrecDec::percent(100));
//         assert_eq!(PrecDec::percent(1000) / one, PrecDec::percent(1000));

//         // double
//         assert_eq!(two / PrecDec::percent(1), PrecDec::percent(20_000));
//         assert_eq!(two / PrecDec::percent(10), PrecDec::percent(2_000));
//         assert_eq!(two / PrecDec::percent(100), PrecDec::percent(200));
//         assert_eq!(two / PrecDec::percent(1000), PrecDec::percent(20));
//         assert_eq!(PrecDec::percent(0) / two, PrecDec::percent(0));
//         assert_eq!(PrecDec::percent(1) / two, dec("0.005"));
//         assert_eq!(PrecDec::percent(10) / two, PrecDec::percent(5));
//         assert_eq!(PrecDec::percent(100) / two, PrecDec::percent(50));
//         assert_eq!(PrecDec::percent(1000) / two, PrecDec::percent(500));

//         // half
//         assert_eq!(half / PrecDec::percent(1), PrecDec::percent(5_000));
//         assert_eq!(half / PrecDec::percent(10), PrecDec::percent(500));
//         assert_eq!(half / PrecDec::percent(100), PrecDec::percent(50));
//         assert_eq!(half / PrecDec::percent(1000), PrecDec::percent(5));
//         assert_eq!(PrecDec::percent(0) / half, PrecDec::percent(0));
//         assert_eq!(PrecDec::percent(1) / half, PrecDec::percent(2));
//         assert_eq!(PrecDec::percent(10) / half, PrecDec::percent(20));
//         assert_eq!(PrecDec::percent(100) / half, PrecDec::percent(200));
//         assert_eq!(PrecDec::percent(1000) / half, PrecDec::percent(2000));

//         // Move right
//         let a = dec("123127726548762582");
//         assert_eq!(a / dec("1"), dec("123127726548762582"));
//         assert_eq!(a / dec("10"), dec("12312772654876258.2"));
//         assert_eq!(a / dec("100"), dec("1231277265487625.82"));
//         assert_eq!(a / dec("1000"), dec("123127726548762.582"));
//         assert_eq!(a / dec("1000000"), dec("123127726548.762582"));
//         assert_eq!(a / dec("1000000000"), dec("123127726.548762582"));
//         assert_eq!(a / dec("1000000000000"), dec("123127.726548762582"));
//         assert_eq!(a / dec("1000000000000000"), dec("123.127726548762582"));
//         assert_eq!(a / dec("1000000000000000000"), dec("0.123127726548762582"));
//         assert_eq!(dec("1") / a, dec("0.000000000000000008"));
//         assert_eq!(dec("10") / a, dec("0.000000000000000081"));
//         assert_eq!(dec("100") / a, dec("0.000000000000000812"));
//         assert_eq!(dec("1000") / a, dec("0.000000000000008121"));
//         assert_eq!(dec("1000000") / a, dec("0.000000000008121647"));
//         assert_eq!(dec("1000000000") / a, dec("0.000000008121647560"));
//         assert_eq!(dec("1000000000000") / a, dec("0.000008121647560868"));
//         assert_eq!(dec("1000000000000000") / a, dec("0.008121647560868164"));
//         assert_eq!(dec("1000000000000000000") / a, dec("8.121647560868164773"));

//         // Move left
//         let a = dec("0.123127726548762582");
//         assert_eq!(a / dec("1.0"), dec("0.123127726548762582"));
//         assert_eq!(a / dec("0.1"), dec("1.23127726548762582"));
//         assert_eq!(a / dec("0.01"), dec("12.3127726548762582"));
//         assert_eq!(a / dec("0.001"), dec("123.127726548762582"));
//         assert_eq!(a / dec("0.000001"), dec("123127.726548762582"));
//         assert_eq!(a / dec("0.000000001"), dec("123127726.548762582"));
//         assert_eq!(a / dec("0.000000000001"), dec("123127726548.762582"));
//         assert_eq!(a / dec("0.000000000000001"), dec("123127726548762.582"));
//         assert_eq!(a / dec("0.000000000000000001"), dec("123127726548762582"));

//         assert_eq!(
//             PrecDec::percent(15) / PrecDec::percent(60),
//             PrecDec::percent(25)
//         );

//         // works for refs
//         let a = PrecDec::percent(100);
//         let b = PrecDec::percent(20);
//         let expected = PrecDec::percent(500);
//         assert_eq!(a / b, expected);
//         assert_eq!(&a / b, expected);
//         assert_eq!(a / &b, expected);
//         assert_eq!(&a / &b, expected);
//     }

//     #[test]
//     fn decimal_div_assign_works() {
//         let mut a = PrecDec::percent(15);
//         a /= PrecDec::percent(20);
//         assert_eq!(a, PrecDec::percent(75));

//         // works for refs
//         let mut a = PrecDec::percent(50);
//         let b = PrecDec::percent(20);
//         a /= &b;
//         assert_eq!(a, PrecDec::percent(250));
//     }

//     #[test]
//     #[should_panic(expected = "Division failed - multiplication overflow")]
//     fn decimal_div_overflow_panics() {
//         let _value = PrecDec::MAX / PrecDec::percent(10);
//     }

//     #[test]
//     #[should_panic(expected = "Division failed - denominator must not be zero")]
//     fn decimal_div_by_zero_panics() {
//         let _value = PrecDec::one() / PrecDec::zero();
//     }

//     #[test]
//     fn decimal_uint128_division() {
//         // a/b
//         let left = PrecDec::percent(150); // 1.5
//         let right = Uint256::new(3);
//         assert_eq!(left / right, PrecDec::percent(50));

//         // 0/a
//         let left = PrecDec::zero();
//         let right = Uint256::new(300);
//         assert_eq!(left / right, PrecDec::zero());
//     }

//     #[test]
//     #[should_panic(expected = "attempt to divide by zero")]
//     fn decimal_uint128_divide_by_zero() {
//         let left = PrecDec::percent(150); // 1.5
//         let right = Uint256::new(0);
//         let _result = left / right;
//     }

//     #[test]
//     fn decimal_uint128_div_assign() {
//         // a/b
//         let mut dec = PrecDec::percent(150); // 1.5
//         dec /= Uint256::new(3);
//         assert_eq!(dec, PrecDec::percent(50));

//         // 0/a
//         let mut dec = PrecDec::zero();
//         dec /= Uint256::new(300);
//         assert_eq!(dec, PrecDec::zero());
//     }

//     #[test]
//     #[should_panic(expected = "attempt to divide by zero")]
//     fn decimal_uint128_div_assign_by_zero() {
//         // a/0
//         let mut dec = PrecDec::percent(50);
//         dec /= Uint256::new(0);
//     }

//     #[test]
//     fn decimal_uint128_sqrt() {
//         assert_eq!(PrecDec::percent(900).sqrt(), PrecDec::percent(300));

//         assert!(PrecDec::percent(316) < PrecDec::percent(1000).sqrt());
//         assert!(PrecDec::percent(1000).sqrt() < PrecDec::percent(317));
//     }

//     /// sqrt(2) is an irrational number, i.e. all 18 decimal places should be used.
//     #[test]
//     fn decimal_uint128_sqrt_is_precise() {
//         assert_eq!(
//             PrecDec::from_str("2").unwrap().sqrt(),
//             PrecDec::from_str("1.414213562373095048").unwrap() // https://www.wolframalpha.com/input/?i=sqrt%282%29
//         );
//     }

//     #[test]
//     fn decimal_uint128_sqrt_does_not_overflow() {
//         assert_eq!(
//             PrecDec::from_str("400").unwrap().sqrt(),
//             PrecDec::from_str("20").unwrap()
//         );
//     }

//     #[test]
//     fn decimal_uint128_sqrt_intermediate_precision_used() {
//         assert_eq!(
//             PrecDec::from_str("400001").unwrap().sqrt(),
//             // The last two digits (27) are truncated below due to the algorithm
//             // we use. Larger numbers will cause less precision.
//             // https://www.wolframalpha.com/input/?i=sqrt%28400001%29
//             PrecDec::from_str("632.456322602596803200").unwrap()
//         );
//     }

//     #[test]
//     fn decimal_checked_pow() {
//         for exp in 0..10 {
//             assert_eq!(PrecDec::one().checked_pow(exp).unwrap(), PrecDec::one());
//         }

//         // This case is mathematically undefined but we ensure consistency with Rust standard types
//         // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=20df6716048e77087acd40194b233494
//         assert_eq!(PrecDec::zero().checked_pow(0).unwrap(), PrecDec::one());

//         for exp in 1..10 {
//             assert_eq!(PrecDec::zero().checked_pow(exp).unwrap(), PrecDec::zero());
//         }

//         for num in &[
//             PrecDec::percent(50),
//             PrecDec::percent(99),
//             PrecDec::percent(200),
//         ] {
//             assert_eq!(num.checked_pow(0).unwrap(), PrecDec::one())
//         }

//         assert_eq!(
//             PrecDec::percent(20).checked_pow(2).unwrap(),
//             PrecDec::percent(4)
//         );

//         assert_eq!(
//             PrecDec::percent(20).checked_pow(3).unwrap(),
//             PrecDec::permille(8)
//         );

//         assert_eq!(
//             PrecDec::percent(200).checked_pow(4).unwrap(),
//             PrecDec::percent(1600)
//         );

//         assert_eq!(
//             PrecDec::percent(200).checked_pow(4).unwrap(),
//             PrecDec::percent(1600)
//         );

//         assert_eq!(
//             PrecDec::percent(700).checked_pow(5).unwrap(),
//             PrecDec::percent(1680700)
//         );

//         assert_eq!(
//             PrecDec::percent(700).checked_pow(8).unwrap(),
//             PrecDec::percent(576480100)
//         );

//         assert_eq!(
//             PrecDec::percent(700).checked_pow(10).unwrap(),
//             PrecDec::percent(28247524900)
//         );

//         assert_eq!(
//             PrecDec::percent(120).checked_pow(123).unwrap(),
//             PrecDec(5486473221892422150877397607u128.into())
//         );

//         assert_eq!(
//             PrecDec::percent(10).checked_pow(2).unwrap(),
//             PrecDec(10000000000000000u128.into())
//         );

//         assert_eq!(
//             PrecDec::percent(10).checked_pow(18).unwrap(),
//             PrecDec(1u128.into())
//         );
//     }

//     #[test]
//     fn decimal_checked_pow_overflow() {
//         assert_eq!(
//             PrecDec::MAX.checked_pow(2),
//             Err(OverflowError::new(OverflowOperation::Pow))
//         );
//     }

//     #[test]
//     fn decimal_to_string() {
//         // Integers
//         assert_eq!(PrecDec::zero().to_string(), "0");
//         assert_eq!(PrecDec::one().to_string(), "1");
//         assert_eq!(PrecDec::percent(500).to_string(), "5");

//         // Decimals
//         assert_eq!(PrecDec::percent(125).to_string(), "1.25");
//         assert_eq!(PrecDec::percent(42638).to_string(), "426.38");
//         assert_eq!(PrecDec::percent(3).to_string(), "0.03");
//         assert_eq!(PrecDec::permille(987).to_string(), "0.987");

//         assert_eq!(
//             PrecDec(Uint256::from(1u128)).to_string(),
//             "0.000000000000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(10u128)).to_string(),
//             "0.00000000000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(100u128)).to_string(),
//             "0.0000000000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(1000u128)).to_string(),
//             "0.000000000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(10000u128)).to_string(),
//             "0.00000000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(100000u128)).to_string(),
//             "0.0000000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(1000000u128)).to_string(),
//             "0.000000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(10000000u128)).to_string(),
//             "0.00000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(100000000u128)).to_string(),
//             "0.0000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(1000000000u128)).to_string(),
//             "0.000000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(10000000000u128)).to_string(),
//             "0.00000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(100000000000u128)).to_string(),
//             "0.0000001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(10000000000000u128)).to_string(),
//             "0.00001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(100000000000000u128)).to_string(),
//             "0.0001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(1000000000000000u128)).to_string(),
//             "0.001"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(10000000000000000u128)).to_string(),
//             "0.01"
//         );
//         assert_eq!(
//             PrecDec(Uint256::from(100000000000000000u128)).to_string(),
//             "0.1"
//         );
//     }

//     #[test]
//     fn decimal_iter_sum() {
//         let items = vec![
//             PrecDec::zero(),
//             PrecDec(Uint256::from(2u128)),
//             PrecDec(Uint256::from(2u128)),
//         ];
//         assert_eq!(items.iter().sum::<PrecDec>(), PrecDec(Uint256::from(4u128)));
//         assert_eq!(
//             items.into_iter().sum::<PrecDec>(),
//             PrecDec(Uint256::from(4u128))
//         );

//         let empty: Vec<PrecDec> = vec![];
//         assert_eq!(PrecDec::zero(), empty.iter().sum::<PrecDec>());
//     }

//     #[test]
//     fn decimal_serialize() {
//         assert_eq!(serde_json::to_vec(&PrecDec::zero()).unwrap(), br#""0""#);
//         assert_eq!(serde_json::to_vec(&PrecDec::one()).unwrap(), br#""1""#);
//         assert_eq!(
//             serde_json::to_vec(&PrecDec::percent(8)).unwrap(),
//             br#""0.08""#
//         );
//         assert_eq!(
//             serde_json::to_vec(&PrecDec::percent(87)).unwrap(),
//             br#""0.87""#
//         );
//         assert_eq!(
//             serde_json::to_vec(&PrecDec::percent(876)).unwrap(),
//             br#""8.76""#
//         );
//         assert_eq!(
//             serde_json::to_vec(&PrecDec::percent(8765)).unwrap(),
//             br#""87.65""#
//         );
//     }

//     #[test]
//     fn decimal_deserialize() {
//         assert_eq!(
//             serde_json::from_slice::<PrecDec>(br#""0""#).unwrap(),
//             PrecDec::zero()
//         );
//         assert_eq!(
//             serde_json::from_slice::<PrecDec>(br#""1""#).unwrap(),
//             PrecDec::one()
//         );
//         assert_eq!(
//             serde_json::from_slice::<PrecDec>(br#""000""#).unwrap(),
//             PrecDec::zero()
//         );
//         assert_eq!(
//             serde_json::from_slice::<PrecDec>(br#""001""#).unwrap(),
//             PrecDec::one()
//         );

//         assert_eq!(
//             serde_json::from_slice::<PrecDec>(br#""0.08""#).unwrap(),
//             PrecDec::percent(8)
//         );
//         assert_eq!(
//             serde_json::from_slice::<PrecDec>(br#""0.87""#).unwrap(),
//             PrecDec::percent(87)
//         );
//         assert_eq!(
//             serde_json::from_slice::<PrecDec>(br#""8.76""#).unwrap(),
//             PrecDec::percent(876)
//         );
//         assert_eq!(
//             serde_json::from_slice::<PrecDec>(br#""87.65""#).unwrap(),
//             PrecDec::percent(8765)
//         );
//     }

//     #[test]
//     fn decimal_abs_diff_works() {
//         let a = PrecDec::percent(285);
//         let b = PrecDec::percent(200);
//         let expected = PrecDec::percent(85);
//         assert_eq!(a.abs_diff(b), expected);
//         assert_eq!(b.abs_diff(a), expected);
//     }

//     #[test]
//     #[allow(clippy::op_ref)]
//     fn decimal_rem_works() {
//         // 4.02 % 1.11 = 0.69
//         assert_eq!(
//             PrecDec::percent(402) % PrecDec::percent(111),
//             PrecDec::percent(69)
//         );

//         // 15.25 % 4 = 3.25
//         assert_eq!(
//             PrecDec::percent(1525) % PrecDec::percent(400),
//             PrecDec::percent(325)
//         );

//         let a = PrecDec::percent(318);
//         let b = PrecDec::percent(317);
//         let expected = PrecDec::percent(1);
//         assert_eq!(a % b, expected);
//         assert_eq!(a % &b, expected);
//         assert_eq!(&a % b, expected);
//         assert_eq!(&a % &b, expected);
//     }

//     #[test]
//     fn decimal_rem_assign_works() {
//         let mut a = PrecDec::percent(17673);
//         a %= PrecDec::percent(2362);
//         assert_eq!(a, PrecDec::percent(1139)); // 176.73 % 23.62 = 11.39

//         let mut a = PrecDec::percent(4262);
//         let b = PrecDec::percent(1270);
//         a %= &b;
//         assert_eq!(a, PrecDec::percent(452)); // 42.62 % 12.7 = 4.52
//     }

//     #[test]
//     #[should_panic(expected = "divisor of zero")]
//     fn decimal_rem_panics_for_zero() {
//         let _ = PrecDec::percent(777) % PrecDec::zero();
//     }

//     #[test]
//     fn decimal_checked_methods() {
//         // checked add
//         assert_eq!(
//             PrecDec::percent(402)
//                 .checked_add(PrecDec::percent(111))
//                 .unwrap(),
//             PrecDec::percent(513)
//         );
//         assert!(matches!(
//             PrecDec::MAX.checked_add(PrecDec::percent(1)),
//             Err(OverflowError { .. })
//         ));

//         // checked sub
//         assert_eq!(
//             PrecDec::percent(1111)
//                 .checked_sub(PrecDec::percent(111))
//                 .unwrap(),
//             PrecDec::percent(1000)
//         );
//         assert!(matches!(
//             PrecDec::zero().checked_sub(PrecDec::percent(1)),
//             Err(OverflowError { .. })
//         ));

//         // checked div
//         assert_eq!(
//             PrecDec::percent(30)
//                 .checked_div(PrecDec::percent(200))
//                 .unwrap(),
//             PrecDec::percent(15)
//         );
//         assert_eq!(
//             PrecDec::percent(88)
//                 .checked_div(PrecDec::percent(20))
//                 .unwrap(),
//             PrecDec::percent(440)
//         );
//         assert!(matches!(
//             PrecDec::MAX.checked_div(PrecDec::zero()),
//             Err(CheckedFromRatioError::DivideByZero {})
//         ));
//         assert!(matches!(
//             PrecDec::MAX.checked_div(PrecDec::percent(1)),
//             Err(CheckedFromRatioError::Overflow {})
//         ));

//         // checked rem
//         assert_eq!(
//             PrecDec::percent(402)
//                 .checked_rem(PrecDec::percent(111))
//                 .unwrap(),
//             PrecDec::percent(69)
//         );
//         assert_eq!(
//             PrecDec::percent(1525)
//                 .checked_rem(PrecDec::percent(400))
//                 .unwrap(),
//             PrecDec::percent(325)
//         );
//         assert!(matches!(
//             PrecDec::MAX.checked_rem(PrecDec::zero()),
//             Err(DivideByZeroError { .. })
//         ));
//     }

//     #[test]
//     fn decimal_pow_works() {
//         assert_eq!(PrecDec::percent(200).pow(2), PrecDec::percent(400));
//         assert_eq!(PrecDec::percent(200).pow(10), PrecDec::percent(102400));
//     }

//     #[test]
//     #[should_panic]
//     fn decimal_pow_overflow_panics() {
//         _ = PrecDec::MAX.pow(2u32);
//     }

//     #[test]
//     fn decimal_saturating_works() {
//         assert_eq!(
//             PrecDec::percent(200).saturating_add(PrecDec::percent(200)),
//             PrecDec::percent(400)
//         );
//         assert_eq!(
//             PrecDec::MAX.saturating_add(PrecDec::percent(200)),
//             PrecDec::MAX
//         );
//         assert_eq!(
//             PrecDec::percent(200).saturating_sub(PrecDec::percent(100)),
//             PrecDec::percent(100)
//         );
//         assert_eq!(
//             PrecDec::zero().saturating_sub(PrecDec::percent(200)),
//             PrecDec::zero()
//         );
//         assert_eq!(
//             PrecDec::percent(200).saturating_mul(PrecDec::percent(50)),
//             PrecDec::percent(100)
//         );
//         assert_eq!(
//             PrecDec::MAX.saturating_mul(PrecDec::percent(200)),
//             PrecDec::MAX
//         );
//         assert_eq!(
//             PrecDec::percent(400).saturating_pow(2u32),
//             PrecDec::percent(1600)
//         );
//         assert_eq!(PrecDec::MAX.saturating_pow(2u32), PrecDec::MAX);
//     }

//     #[test]
//     fn decimal_rounding() {
//         assert_eq!(PrecDec::one().floor(), PrecDec::one());
//         assert_eq!(PrecDec::percent(150).floor(), PrecDec::one());
//         assert_eq!(PrecDec::percent(199).floor(), PrecDec::one());
//         assert_eq!(PrecDec::percent(200).floor(), PrecDec::percent(200));
//         assert_eq!(PrecDec::percent(99).floor(), PrecDec::zero());

//         assert_eq!(PrecDec::one().ceil(), PrecDec::one());
//         assert_eq!(PrecDec::percent(150).ceil(), PrecDec::percent(200));
//         assert_eq!(PrecDec::percent(199).ceil(), PrecDec::percent(200));
//         assert_eq!(PrecDec::percent(99).ceil(), PrecDec::one());
//         assert_eq!(PrecDec(Uint256::from(1u128)).ceil(), PrecDec::one());
//     }

//     #[test]
//     #[should_panic(expected = "attempt to ceil with overflow")]
//     fn decimal_ceil_panics() {
//         let _ = PrecDec::MAX.ceil();
//     }

//     #[test]
//     fn decimal_checked_ceil() {
//         assert_eq!(
//             PrecDec::percent(199).checked_ceil(),
//             Ok(PrecDec::percent(200))
//         );
//         assert!(matches!(
//             PrecDec::MAX.checked_ceil(),
//             Err(RoundUpOverflowError { .. })
//         ));
//     }

//     #[test]
//     fn decimal_to_uint_floor_works() {
//         let d = PrecDec::from_str("12.000000000000000001").unwrap();
//         assert_eq!(d.to_uint_floor(), Uint256::new(12));
//         let d = PrecDec::from_str("12.345").unwrap();
//         assert_eq!(d.to_uint_floor(), Uint256::new(12));
//         let d = PrecDec::from_str("12.999").unwrap();
//         assert_eq!(d.to_uint_floor(), Uint256::new(12));
//         let d = PrecDec::from_str("0.98451384").unwrap();
//         assert_eq!(d.to_uint_floor(), Uint256::new(0));

//         let d = PrecDec::from_str("75.0").unwrap();
//         assert_eq!(d.to_uint_floor(), Uint256::new(75));
//         let d = PrecDec::from_str("0.0").unwrap();
//         assert_eq!(d.to_uint_floor(), Uint256::new(0));

//         let d = PrecDec::MAX;
//         assert_eq!(d.to_uint_floor(), Uint256::new(340282366920938463463));

//         // Does the same as the old workaround `Uint256::one() * my_decimal`.
//         // This block can be deleted as part of https://github.com/CosmWasm/cosmwasm/issues/1485.
//         let tests = vec![
//             (PrecDec::from_str("12.345").unwrap(), 12u128),
//             (PrecDec::from_str("0.98451384").unwrap(), 0u128),
//             (PrecDec::from_str("178.0").unwrap(), 178u128),
//             (PrecDec::MIN, 0u128),
//             (PrecDec::MAX, u128::MAX / PrecDec::DECIMAL_FRACTIONAL.u128()),
//         ];
//         for (my_decimal, expected) in tests.into_iter() {
//             assert_eq!(my_decimal.to_uint_floor(), Uint256::new(expected));
//         }
//     }

//     #[test]
//     fn decimal_to_uint_ceil_works() {
//         let d = PrecDec::from_str("12.000000000000000001").unwrap();
//         assert_eq!(d.to_uint_ceil(), Uint256::new(13));
//         let d = PrecDec::from_str("12.345").unwrap();
//         assert_eq!(d.to_uint_ceil(), Uint256::new(13));
//         let d = PrecDec::from_str("12.999").unwrap();
//         assert_eq!(d.to_uint_ceil(), Uint256::new(13));

//         let d = PrecDec::from_str("75.0").unwrap();
//         assert_eq!(d.to_uint_ceil(), Uint256::new(75));
//         let d = PrecDec::from_str("0.0").unwrap();
//         assert_eq!(d.to_uint_ceil(), Uint256::new(0));

//         let d = PrecDec::MAX;
//         assert_eq!(d.to_uint_ceil(), Uint256::new(340282366920938463464));
//     }

//     #[test]
//     fn decimal_partial_eq() {
//         let test_cases = [
//             ("1", "1", true),
//             ("0.5", "0.5", true),
//             ("0.5", "0.51", false),
//             ("0", "0.00000", true),
//         ]
//         .into_iter()
//         .map(|(lhs, rhs, expected)| (dec(lhs), dec(rhs), expected));

//         #[allow(clippy::op_ref)]
//         for (lhs, rhs, expected) in test_cases {
//             assert_eq!(lhs == rhs, expected);
//             assert_eq!(&lhs == rhs, expected);
//             assert_eq!(lhs == &rhs, expected);
//             assert_eq!(&lhs == &rhs, expected);
//         }
//     }

//     #[test]
//     fn decimal_implements_debug() {
//         let decimal = PrecDec::from_str("123.45").unwrap();
//         assert_eq!(format!("{decimal:?}"), "Decimal(123.45)");

//         let test_cases = ["5", "5.01", "42", "0", "2"];
//         for s in test_cases {
//             let decimal = PrecDec::from_str(s).unwrap();
//             let expected = format!("Decimal({s})");
//             assert_eq!(format!("{decimal:?}"), expected);
//         }
//     }
// }
