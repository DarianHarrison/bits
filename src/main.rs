#[allow(unused_variables)]

fn main() {


    // AND, minimizes
    let a: u8 = 12;
    let b: u8 = 7;
    let op_1: u8 = a & b;

    println!("{:?}", "// AND");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // OR, maximizes
    let a: u8 = 12;
    let b: u8 = 7;
    let op_1: u8 = a | b;

    println!("{:?}", "// OR");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // XOR
    let a: u8 = 12;
    let b: u8 = 7;
    let op_1: u8 = a ^ b;

    println!("{:?}", "// XOR");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // NOT, inverts
    let b: u8 = 7;
    let op_1: u8 = !b;

    println!("{:?}", "// NOT");
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // LEFT SHIFT
    let a: u8 = 12;
    let op_1: u8 = a << 3;

    println!("{:?}", "// LEFT SHIFT 3");
    println!("{:08b}", &a);
    println!("{:08b}", &op_1);

    // RIGHT SHIFT
    let a: u8 = 12;
    let op_1: u8 = a >> 3;

    println!("{:?}", "// RIGHT SHIFT 3");
    println!("{:08b}", &a);
    println!("{:08b}", &op_1);


    println!("{:?}", "// ROTATE LEFT 3");
    let a: u8 = 7;
    let op_1: u8 = a.rotate_left(3);
    println!("{:08b}", &a);
    println!("{:08b}", &op_1);

    println!("{:?}", "// ROTATE RIGHT 3");
    let a: u8 = 7;
    let op_1: u8 = a.rotate_right(3);
    println!("{:08b}", &a);
    println!("{:08b}", &op_1);

    println!("{:?}", "// COUNT ONES");
    let a: u8 = 7;
    let op_1: u8 = a.count_ones() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    println!("{:?}", "// COUNT ZEROS");
    let a: u8 = 7;
    let op_1: u8 = a.count_zeros() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    println!("{:?}", "// LEADING ZEROS");
    let a: u8 = 7;
    let op_1: u8 = a.leading_zeros() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    println!("{:?}", "// LEADING ONES");
    let a: u8 = 7;
    let op_1: u8 = a.leading_ones() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    println!("{:?}", "// TRAILING ZEROS");
    let a: u8 = 7;
    let op_1: u8 = a.trailing_zeros() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    println!("{:?}", "// TRAILING ONES");
    let a: u8 = 7;
    let op_1: u8 = a.trailing_ones() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);


assert_eq!(u8::MAX.saturating_pow(2), u8::MAX);





// useful

Wrapping (modular) addition. Computes self + rhs, wrapping around at the boundary of the type.
assert_eq!(200u8.wrapping_add(55), 255);
assert_eq!(200u8.wrapping_add(u8::MAX), 199);

Wrapping (modular) subtraction. Computes self - rhs, wrapping around at the boundary of the type.
assert_eq!(100u8.wrapping_sub(100), 0);
assert_eq!(100u8.wrapping_sub(u8::MAX), 101);

Wrapping (modular) multiplication. Computes self * rhs, wrapping around at the boundary of the type.
assert_eq!(10u8.wrapping_mul(12), 120); 
assert_eq!(25u8.wrapping_mul(12), 44);


Wrapping (modular) division. 
Computes self / rhs. Wrapped division on unsigned types is just normal division. There’s no way wrapping could ever happen. This function exists, so that all operations are accounted for in the wrapping operations.
assert_eq!(100u8.wrapping_div(10), 10);


Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder calculation on unsigned types is just the regular remainder calculation. There’s no way wrapping could ever happen. This function exists, so that all operations are accounted for in the wrapping operations.
assert_eq!(100u8.wrapping_rem(10), 0);

Wrapping (modular) negation. Computes -self, wrapping around at the boundary of the type.
Since unsigned types do not have negative equivalents all applications of this function will wrap (except for -0). For values smaller than the corresponding signed type’s maximum the result is the same as casting the corresponding signed value. Any larger values are equivalent to MAX + 1 - (val - MAX - 1) where MAX is the corresponding signed type’s maximum.
assert_eq!(100i8.wrapping_neg(), -100);
assert_eq!((-128i8).wrapping_neg(), -128);

Wrapping (modular) exponentiation. Computes self.pow(exp), wrapping around at the boundary of the type.
assert_eq!(3u8.wrapping_pow(5), 243);
assert_eq!(3u8.wrapping_pow(6), 217);

wrapping shift
wrapping


abs_diff
Computes the absolute difference between self and other.
#![feature(int_abs_diff)]
assert_eq!(100u8.abs_diff(80), 20u8);
assert_eq!(100u8.abs_diff(110), 10u8);


pow
Raises self to the power of exp, using exponentiation by squaring.
assert_eq!(2u8.pow(5), 32);

 div_floor
div_ceil

 next_multiple_of
 Calculates the smallest value greater than or equal to self that is a multiple of rhs.
 #![feature(int_roundings)]
assert_eq!(16_u8.next_multiple_of(8), 16);
assert_eq!(23_u8.next_multiple_of(8), 24);


is_power_of_two
Returns true if and only if self == 2^k for some k.
assert!(16u8.is_power_of_two());
assert!(!10u8.is_power_of_two());

next_power_of_two
Returns the smallest power of two greater than or equal to self.
When return value overflows (i.e., self > (1 << (N-1)) for type uN), it panics in debug mode and the return value is wrapped to 0 in release mode (the only situation in which method can return 0).
assert_eq!(2u8.next_power_of_two(), 2);
assert_eq!(3u8.next_power_of_two(), 4);


 wrapping_next_power_of_two
 Returns the smallest power of two greater than or equal to n. If the next power of two is greater than the type’s maximum value, the return value is wrapped to 0.
#![feature(wrapping_next_power_of_two)]
assert_eq!(2u8.wrapping_next_power_of_two(), 2);
assert_eq!(3u8.wrapping_next_power_of_two(), 4);
assert_eq!(u8::MAX.wrapping_next_power_of_two(), 0);

}