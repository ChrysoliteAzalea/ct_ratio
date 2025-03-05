use ct_ratio::*;

#[test]
fn negative_less_than_zero()
{
   type Negative = Ratio<1,-1>;
   type Negative2 = Ratio<-1,1>;
   type Zero = Ratio<0, 1>;
   type Cmp = RationalCmp<Negative,Zero>;
   type Cmp2 = RationalCmp<Negative2,Zero>;
   assert!(Cmp::LESSER);
   assert!(Cmp2::LESSER);
}

#[test]
fn negative_less_than_positive()
{
   type N1 = Ratio<1,-1>;
   type N2 = Ratio<-1,1>;
   type P1 = Ratio<1,1>;
   type P2 = Ratio<-1,-1>;
   type C1 = RationalCmp<N1,P1>;
   type C2 = RationalCmp<N1,P2>;
   type C3 = RationalCmp<N2,P1>;
   type C4 = RationalCmp<N2,P2>;
   assert!(C1::LESSER && C2::LESSER && C3::LESSER && C4::LESSER);
}

#[test]
fn positive_greater_than_zero()
{
   type P1 = Ratio<1,1>;
   type P2 = Ratio<-1,-1>;
   type Zero = Ratio<0,-1>;
   type C1 = RationalCmp<P1,Zero>;
   type C2 = RationalCmp<P2,Zero>;
   type C3 = RationalCmp<P1,P2>;
   assert!(C1::GREATER && C2::GREATER && C3::EQUAL);
}

#[test]
fn funny_cmp()
{
   type Half = Ratio<-1,-2>;
   type Big = Ratio<2,4>;
   type Cmp = RationalCmp<Half,Big>;
   assert!(Cmp::EQUAL);
}