use ct_ratio::*;
use std::io::Write;

#[test]
fn reduction()
{
   type ORIGINAL = Ratio<5,10>;
   type Reduced = <ORIGINAL as Reducible>::Reduced;
   let _ = writeln!(std::io::stdout().lock(), "Reduced {}/{} to {}/{}", ORIGINAL::NUMERATOR, ORIGINAL::DENOMINATOR, Reduced::NUMERATOR, Reduced::DENOMINATOR);
   assert!(!ORIGINAL::IS_IRREDUCIBLE);
   assert!(Reduced::IS_IRREDUCIBLE);
   assert_eq!(Reduced::NUMERATOR, 1);
   assert_eq!(Reduced::DENOMINATOR.get(), 2);
}

#[test]
fn part_negative()
{
   type ORIGINAL = Ratio<-12,18>;
   type Reduced = <ORIGINAL as Reducible>::Reduced;
   let _ = writeln!(std::io::stdout().lock(), "Reduced {}/{} to {}/{}", ORIGINAL::NUMERATOR, ORIGINAL::DENOMINATOR, Reduced::NUMERATOR, Reduced::DENOMINATOR);
   assert!(!ORIGINAL::IS_IRREDUCIBLE);
   assert!(Reduced::IS_IRREDUCIBLE);
   assert_eq!(Reduced::NUMERATOR, -2);
   assert_eq!(Reduced::DENOMINATOR.get(), 3);
}

#[test]
fn part_negative2()
{
   type ORIGINAL = Ratio<70,-154>;
   type Reduced = <ORIGINAL as Reducible>::Reduced;
   let _ = writeln!(std::io::stdout().lock(), "Reduced {}/{} to {}/{}", ORIGINAL::NUMERATOR, ORIGINAL::DENOMINATOR, Reduced::NUMERATOR, Reduced::DENOMINATOR);
   assert!(!ORIGINAL::IS_IRREDUCIBLE);
   assert!(Reduced::IS_IRREDUCIBLE);
   assert_eq!(Reduced::NUMERATOR, -5);
   assert_eq!(Reduced::DENOMINATOR.get(), 11);
}

#[test]
fn all_negative()
{
   type ORIGINAL = Ratio<-99,-9999>;
   type Reduced = <ORIGINAL as Reducible>::Reduced;
   let _ = writeln!(std::io::stdout().lock(), "Reduced {}/{} to {}/{}", ORIGINAL::NUMERATOR, ORIGINAL::DENOMINATOR, Reduced::NUMERATOR, Reduced::DENOMINATOR);
   assert!(!ORIGINAL::IS_IRREDUCIBLE);
   assert!(Reduced::IS_IRREDUCIBLE);
   assert_eq!(Reduced::NUMERATOR, 1);
   assert_eq!(Reduced::DENOMINATOR.get(), 101);
}