#![no_std]
#![feature(const_option)]
#![feature(generic_const_exprs)]
#![expect(incomplete_features)] // should be removed when the "generic_const_exprs" becomes a complete feature
//! This crate provides compile-time rational arithmetic support in Rust with const functions and const generics. It can be used to specify measurement units for numbers.
//!
//! Note that this crate relies on the incomplete [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560) feature. Please, use with caution.

use core::num::NonZero;
use core::marker::PhantomData;

/// Finds the greatest common divisor using the Euclidean algorithm
pub const fn gcd(x: i128, y: i128) -> i128
{
   if x == 0 { return y; }
   if y == 0 { return x; }
   if x < 0 { return gcd(-x, y); }
   if y < 0 { return gcd(x, -y); }
   let mut a = x;
   let mut b = y;
   let mut r = a % b;
   while r != 0
   {
      a = b;
      b = r;
      r = a % b;
   }
   b
}

/// A type that represents a rational compile-time constant. This is an uninhabited type that cannot be instantiated, instead, it's intended to be used for generics.
/// Please note that two distict ```Ratio``` types may represent the same number. You can use the ```Reduced``` type alias from the ```Reducible``` trait to reduce the number to the lowest terms
pub enum Ratio<const N: i128, const D: i128> {}

/// A trait for rational compile-time constants that can be used as a trait bound for generics.
pub trait StaticRatio
{
   /// The numerator
   const NUMERATOR: i128;
   /// The denominator
   const DENOMINATOR: NonZero<i128>;
}

/// A trait to obtain results of compile-time rational arithmetic operations
pub trait StaticArithmetic : StaticRatio
{
   type OpResult where [(); {Self::DENOMINATOR.get()} as usize]:, [(); {Self::NUMERATOR} as usize]:;
}

/// A trait to reduce compile-time rational constants
pub trait Reducible
{
   /// The greatest common divisor of the numerator and denominator
   const GCD_VALUE: i128;
   /// Reduced numerator value
   const R_NUMERATOR: i128;
   /// Reduced denominator value
   const R_DENOMINATOR: i128;
   /// Equals to true if the fraction cannot be reduced further
   const IS_IRREDUCIBLE: bool;
   /// Fraction reduced to its lowest terms
   type Reduced where [(); {Self::R_DENOMINATOR} as usize]:, [(); {Self::R_NUMERATOR} as usize]:;
}

impl<const N: i128, const D: i128> Reducible for Ratio<N,D>
{
   const GCD_VALUE: i128 = gcd(N, D);
   const R_NUMERATOR: i128 = N / Self::GCD_VALUE;
   const R_DENOMINATOR: i128 = D / Self::GCD_VALUE;
   const IS_IRREDUCIBLE: bool = Self::GCD_VALUE == 1;
   type Reduced = Ratio<{Self::R_NUMERATOR},{Self::R_DENOMINATOR}> where [(); {Self::R_NUMERATOR} as usize]:, [(); {Self::R_DENOMINATOR} as usize]:;
}

impl<T: StaticRatio> StaticArithmetic for T
{
   type OpResult = Ratio<{T::NUMERATOR},{T::DENOMINATOR.get()}> where [(); {T::NUMERATOR} as usize]:, [(); {T::DENOMINATOR.get()} as usize]:;
}

impl<const N: i128, const D: i128> StaticRatio for Ratio<N,D>
{
   const NUMERATOR: i128 = if D >= 0 { N } else { -N };
   const DENOMINATOR: NonZero<i128> = NonZero::new(if D >= 0 { D } else { -D }).unwrap();
}

/// Helper type for adding up two compile time rational constants
pub struct RationalSum<X, Y>
{
   _a: PhantomData<X>,
   _b: PhantomData<Y>,
}

impl<X: StaticRatio, Y: StaticRatio> StaticRatio for RationalSum<X,Y>
{
   const NUMERATOR: i128 = X::NUMERATOR*Y::DENOMINATOR.get() + X::DENOMINATOR.get()*Y::NUMERATOR;
   const DENOMINATOR: NonZero<i128> = NonZero::new(X::DENOMINATOR.get()*Y::DENOMINATOR.get()).unwrap();
}

/// Helper type for substracting one rational constant from another
pub struct RationalDiff<X, Y>
{
   _a: PhantomData<X>,
   _b: PhantomData<Y>,
}

impl<X: StaticRatio, Y: StaticRatio> StaticRatio for RationalDiff<X,Y>
{
   const NUMERATOR: i128 = X::NUMERATOR*Y::DENOMINATOR.get() - X::DENOMINATOR.get()*Y::NUMERATOR;
   const DENOMINATOR: NonZero<i128> = NonZero::new(X::DENOMINATOR.get()*Y::DENOMINATOR.get()).unwrap();
}

/// Helper type for multiplying two compile time rational constants
pub struct RationalProduct<X, Y>
{
   _a: PhantomData<X>,
   _b: PhantomData<Y>,
}

impl<X: StaticRatio, Y: StaticRatio> StaticRatio for RationalProduct<X,Y>
{
   const NUMERATOR: i128 = X::NUMERATOR*Y::NUMERATOR;
   const DENOMINATOR: NonZero<i128> = NonZero::new(X::DENOMINATOR.get()*Y::DENOMINATOR.get()).unwrap();
}

/// Helper type for dividing one rational constant by another
pub struct RationalDiv<X, Y>
{
   _a: PhantomData<X>,
   _b: PhantomData<Y>,
}

impl<X: StaticRatio, Y: StaticRatio> StaticRatio for RationalDiv<X,Y>
{
   const NUMERATOR: i128 = X::NUMERATOR*Y::DENOMINATOR.get();
   const DENOMINATOR: NonZero<i128> = NonZero::new(X::DENOMINATOR.get()*Y::NUMERATOR).unwrap();
}

/// Helper type for comparing two compile time rational constants
pub struct RationalCmp<X, Y>
{
   _a: PhantomData<X>,
   _b: PhantomData<Y>,
}

impl<X: StaticRatio, Y: StaticRatio> RationalCmp<X,Y>
{
   /// Equals to ```true``` if generic parameters represent equal values
   pub const EQUAL: bool = (X::NUMERATOR*Y::DENOMINATOR.get())==(Y::NUMERATOR*X::DENOMINATOR.get());
   /// Equals to ```true``` if generic parameters represent unequal values
   pub const NOT_EQUAL: bool = !(Self::EQUAL);
   const ABS_D1: i128 = if X::DENOMINATOR.get() > 0 { X::DENOMINATOR.get() } else { -(X::DENOMINATOR.get()) };
   const ABS_D2: i128 = if Y::DENOMINATOR.get() > 0 { Y::DENOMINATOR.get() } else { -(Y::DENOMINATOR.get()) };
   const SIGN1: i128 = if X::DENOMINATOR.get() > 0 { 1 } else { -1 };
   const SIGN2: i128 = if Y::DENOMINATOR.get() > 0 { 1 } else { -1 };
   /// Equals to ```true``` if ```X``` represents a number that is smaller than one represented by ```Y```
   pub const LESSER: bool = Self::SIGN1*(X::NUMERATOR*(Self::ABS_D2)) < Self::SIGN2*(Y::NUMERATOR*(Self::ABS_D1));
   /// Equals to ```true``` if ```X``` represents a number that is bigger than one represented by ```Y```
   pub const GREATER: bool = !(Self::EQUAL) && !(Self::LESSER);
   /// Equals to ```true``` if ```X``` represents a number that is no smaller than one represented by ```Y```
   pub const GREATER_OR_EQUAL: bool = !(Self::LESSER);
   /// Equals to ```true``` if ```X``` represents a number that is no bigger than one represented by ```Y```
   pub const LESSER_OR_EQUAL: bool = Self::LESSER || Self::EQUAL;
}