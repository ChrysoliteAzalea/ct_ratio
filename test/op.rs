use ct_ratio::*;
use std::io::Write;

#[test]
fn check_addition()
{
   type A = Ratio<1, 2>;
   type B = Ratio<2, 3>;
   type C = <RationalSum<A,B> as StaticArithmetic>::OpResult;
   writeln!(std::io::stderr().lock(), "{} / {}", C::NUMERATOR, C::DENOMINATOR.get()).unwrap();
   assert_eq!(C::NUMERATOR, 7);
   assert_eq!(C::DENOMINATOR.get(), 6);
}

#[test]
fn check_substraction()
{
   type A = Ratio<1, 2>;
   type B = Ratio<1, 3>;
   type C = <RationalDiff<A,B> as StaticArithmetic>::OpResult;
   writeln!(std::io::stderr().lock(), "{} / {}", C::NUMERATOR, C::DENOMINATOR.get()).unwrap();
   assert_eq!(C::NUMERATOR, 1);
   assert_eq!(C::DENOMINATOR.get(), 6);
}

#[test]
fn check_div()
{
   type A = Ratio<1, 1>;
   //type B = Ratio<0, 1>; // OK, division by zero is not allowed
   type B = Ratio<1, 2>;
   type C = RationalDiv<A,B>;
   writeln!(std::io::stderr().lock(), "{} / {}", C::NUMERATOR, C::DENOMINATOR.get()).unwrap();
   assert_eq!(C::NUMERATOR, 2);
   assert_eq!(C::DENOMINATOR.get(), 1);
}