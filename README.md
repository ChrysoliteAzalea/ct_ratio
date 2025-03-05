# Compile-time rational arithmetic in Rust

The ```ct_ratio``` provides compile-time arithmetic in Rust using const functions and const generics. An example of such code:

```
type A = Ratio<1, 2>;
type B = Ratio<1, 3>;
type C = <RationalSum<A,B> as StaticArithmetic>::OpResult; // This type will be equal to Ratio<5, 6>
```

This code relies on a feature [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560) that is currently (at the README writing time) is considered incomplete. Please, use with caution.