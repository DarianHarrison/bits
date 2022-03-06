#[allow(unused_variables)]

How to create your own hash table in rust
```
https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html#:~:text=By%20default%2C%20Rust%20hash%20tables,high%20quality%20but%20fairly%20slow.
```

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
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // LEFT SHIFT
    let a: u8 = 12;
    let b: u8 = 7;
    let op_1: u8 = a << b;

    println!("{:?}", "// LEFT SHIFT");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // RIGHT SHIFT
    let a: u8 = 12;
    let b: u8 = 7;
    let op_1: u8 = a >> b;

    println!("{:?}", "// RIGHT SHIFT");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // RIGHT SHIFT WITH ZERO
/*    let a: u8 = 12;
    let b: u8 = 7;
    let op_1: u8 = a >>> 1;*/

    println!("{:?}", "// RIGHT SHIFT WITH ZERO");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);



bitand_assign
&=

bitor_assign
|=

bitxor_assign
^=

div
/

div_assign
/=

rem_assign
%=

mul
Performs the * operation

mul_assign
*=

shl_assign
<<=

shl_assign
<<=

shr_assign
>>=

sub_assign
Performs the -= operation.

mask
The mask element type corresponding to this element type
type Mask = i8



Performs the conversion.
impl<T> From<T> for T

pub fn from(t: T) -> T
Performs the conversion.

impl<T, U> Into<U> for T
where
    U: From<T>, 
[src]
pub fn into(self) -> U


into
Performs the conversion.
impl<T, U> Into<U> for T
where
    U: From<T>, 
[src]
pub fn into(self) -> U

```
impl<'_, '_> Shl<&'_ u32> for &'_ u8
type Output = <u8 as Shl<u32>>::Output
The resulting type after applying the << operator.
```
```
impl Shl<u32> for u8
type Output = u8
The resulting type after applying the << operator.
```

Hasher
A trait for hashing an arbitrary stream of bytes.
Instances of Hasher usually represent state that is changed while hashing data.
Hasher provides a fairly basic interface for retrieving the generated hash (with finish), and writing integers as well as slices of bytes into an instance (with write and write_u8 etc.). Most of the time, Hasher instances are used in conjunction with the Hash trait.
This trait makes no assumptions about how the various write_* methods are defined and implementations of Hash should not assume that they work one way or another. You cannot assume, for example, that a write_u32 call is equivalent to four calls of write_u8.
```
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

let mut hasher = DefaultHasher::new();

hasher.write_u32(1989);
hasher.write_u8(11);
hasher.write_u8(9);
hasher.write(b"Huh?");

println!("Hash is {:x}!", hasher.finish());
```

hash
hash<H>(&self, state: &mut H)
Feeds this value into the given Hasher.

hash_slice
hash_slice<H>(data: &[u8], state: &mut H)
Feeds a slice of this type into the given Hasher.

fn write_u8(&mut self, i: u8)
Writes a single u8 into this hasher.

```
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

let mut hasher = DefaultHasher::new();
let data = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
hasher.write(&data);
println!("Hash is {:x}!", hasher.finish());
```

finish(&self) -> u64
Returns the hash value for the values written so far.
Despite its name, the method does not reset the hasher’s internal state. Additional writes will continue from the current value. If you need to start a fresh hash value, you will have to create a new hasher.
```
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
let mut hasher = DefaultHasher::new();
hasher.write(b"Cool!");
println!("Hash is {:x}!", hasher.finish());
```


    /////////////////////
    // // log2
    // //Returns the base 2 logarithm of the number, rounded down.
    // #![feature(int_log)]
    // assert_eq!(2u8.log2(), 1);

    // // log10
    // // Returns the base 10 logarithm of the number, rounded down.
    // #![feature(int_log)]
    // assert_eq!(10u8.log10(), 1);


    // // Saturating integer addition.
    // // Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    // assert_eq!(100u8.saturating_add(1), 101);
    // assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

    // // Saturating integer subtraction. 
    // // Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    // assert_eq!(100u8.saturating_sub(27), 73);
    // assert_eq!(13u8.saturating_sub(127), 0);

/*Saturating integer multiplication. Computes self * rhs, saturating at the numeric bounds instead of overflowing.
assert_eq!(2u8.saturating_mul(10), 20);
assert_eq!((u8::MAX).saturating_mul(10), u8::MAX);


Saturating integer division. Computes self / rhs, saturating at the numeric bounds instead of overflowing.
assert_eq!(5u8.saturating_div(2), 2);*/

Saturating integer exponentiation. Computes self.pow(exp), saturating at the numeric bounds instead of overflowing.
assert_eq!(4u8.saturating_pow(3), 64);
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



overflowing_add
Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.

assert_eq!(5u8.overflowing_add(2), (7, false));
assert_eq!(u8::MAX.overflowing_add(1), (0, true));

carrying_add
Calculates self + rhs + carry without the ability to overflow.
Performs “ternary addition” which takes in an extra bit to add, and may return an additional bit of overflow. This allows for chaining together multiple additions to create “big integers” which represent larger values.
This can be thought of as a 8-bit “full adder”, in the electronics sense.
#![feature(bigint_helper_methods)]
assert_eq!(5u8.carrying_add(2, false), (7, false));
assert_eq!(5u8.carrying_add(2, true), (8, false));
assert_eq!(u8::MAX.carrying_add(1, false), (0, true));
assert_eq!(u8::MAX.carrying_add(0, true), (0, true));
assert_eq!(u8::MAX.carrying_add(1, true), (1, true));
assert_eq!(u8::MAX.carrying_add(u8::MAX, true), (u8::MAX, true));

overflowing_sub
Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned
assert_eq!(5u8.overflowing_sub(2), (3, false));
assert_eq!(0u8.overflowing_sub(1), (u8::MAX, true));

borrowing_sub
Calculates self - rhs - borrow without the ability to overflow.
Performs “ternary subtraction” which takes in an extra bit to subtract, and may return an additional bit of overflow. This allows for chaining together multiple subtractions to create “big integers” which represent larger values.
#![feature(bigint_helper_methods)]
assert_eq!(5u8.borrowing_sub(2, false), (3, false));
assert_eq!(5u8.borrowing_sub(2, true), (2, false));
assert_eq!(0u8.borrowing_sub(1, false), (u8::MAX, true));
assert_eq!(0u8.borrowing_sub(1, true), (u8::MAX - 1, true));

abs_diff
Computes the absolute difference between self and other.
#![feature(int_abs_diff)]
assert_eq!(100u8.abs_diff(80), 20u8);
assert_eq!(100u8.abs_diff(110), 10u8);


overflowing_mul
Calculates the multiplication of self and rhs.
Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
assert_eq!(5u32.overflowing_mul(2), (10, false));
assert_eq!(1_000_000_000u32.overflowing_mul(10), (1410065408, true));


overflowing_div
Calculates the divisor when self is divided by rhs.
Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would occur. Note that for unsigned integers overflow never occurs, so the second value is always false.
assert_eq!(5u8.overflowing_div(2), (2, false));


overflowing_rem
Calculates the remainder when self is divided by rhs.
Returns a tuple of the remainder after dividing along with a boolean indicating whether an arithmetic overflow would occur. Note that for unsigned integers overflow never occurs, so the second value is always false.
assert_eq!(5u8.overflowing_rem(2), (1, false));


overflowing_neg
Negates self in an overflowing fashion.
Returns !self + 1 using wrapping operations to return the value that represents the negation of this unsigned value. Note that for positive unsigned values overflow always occurs, but negating 0 does not overflow.
assert_eq!(0u8.overflowing_neg(), (0, false));
assert_eq!(2u8.overflowing_neg(), (-2i32 as u8, true));

overflowing_pow
Raises self to the power of exp, using exponentiation by squaring.
Returns a tuple of the exponentiation along with a bool indicating whether an overflow happened.
assert_eq!(3u8.overflowing_pow(5), (243, false));
assert_eq!(3u8.overflowing_pow(6), (217, true));

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

u8::MIN
Returns the smallest value that can be represented by this integer type.
u8::MIN

u8::MAX 
Returns the largest value that can be represented by this integer type.
u8::MAX

widening_mul
Calculates the complete product self * rhs without the possibility to overflow.
This returns the low-order (wrapping) bits and the high-order (overflow) bits of the result as two separate values, in that order.
#![feature(bigint_helper_methods)]
assert_eq!(5u32.widening_mul(2), (10, 0));
assert_eq!(1_000_000_000u32.widening_mul(10), (1410065408, 2));

carrying_mul
Calculates the “full multiplication” self * rhs + carry without the possibility to overflow.
This returns the low-order (wrapping) bits and the high-order (overflow) bits of the result as two separate values, in that order.
Performs “long multiplication” which takes in an extra amount to add, and may return an additional amount of overflow. This allows for chaining together multiple multiplications to create “big integers” which represent larger values.
#![feature(bigint_helper_methods)]
assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
assert_eq!(u8::MAX.carrying_mul(u8::MAX, u8::MAX), (0, u8::MAX));

forward
Returns the value that would be obtained by taking the successor of self count times.
forward(start: u8, n: usize) -> u8

backward
Returns the value that would be obtained by taking the predecessor of self count times.
backward(start: u8, n: usize) -> u8

steps_between(start: &u8, end: &u8) -> Option<usize>
Returns the number of successor steps required to get from start to end.

sum<I>(iter: I) -> u8
Method which takes an iterator and generates Self from the elements by “summing up” the items.

}