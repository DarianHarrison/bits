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

    // XOR, find bitwise magnets
    let a: u8 = 12;
    let b: u8 = 7;
    let op_1: u8 = a ^ b;

    println!("{:?}", "// XOR");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // NOT, flip bits
    let b: u8 = 7;
    let op_1: u8 = !b;

    println!("{:?}", "// NOT");
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // LEFT SHIFT, larger number
    let a: u8 = 12;
    let op_1: u8 = a << 3;

    println!("{:?}", "// LEFT SHIFT 3");
    println!("{:08b}", &a);
    println!("{:08b}", &op_1);

    // RIGHT SHIFT, smaller number 
    let a: u8 = 12;
    let op_1: u8 = a >> 3;

    println!("{:?}", "// RIGHT SHIFT 3");
    println!("{:08b}", &a);
    println!("{:08b}", &op_1);

    // ROTATE RIGT, clockwise movement
    println!("{:?}", "// ROTATE RIGHT 3");
    let a: u8 = 7;
    let op_1: u8 = a.rotate_right(3);
    println!("{:08b}", &a);
    println!("{:08b}", &op_1);

    // ROTATE LEFT, counter-clockwise movement
    println!("{:?}", "// ROTATE LEFT 3");
    let a: u8 = 7;
    let op_1: u8 = a.rotate_left(3);
    println!("{:08b}", &a);
    println!("{:08b}", &op_1);

    // COUNT ONES, one cardinallity
    println!("{:?}", "// COUNT ONES");
    let a: u8 = 7;
    let op_1: u8 = a.count_ones() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    // COUNT ONES, zero cardinallity
    println!("{:?}", "// COUNT ZEROS");
    let a: u8 = 7;
    let op_1: u8 = a.count_zeros() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    // LEADING ZEROS, index of first ocurrence of 1, if any
    println!("{:?}", "// LEADING ZEROS");
    let a: u8 = 7;
    let op_1: u8 = a.leading_zeros() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    // LEADING ONES, index of first ocurrence of 0, if any
    println!("{:?}", "// LEADING ONES");
    let a: u8 = 7;
    let op_1: u8 = a.leading_ones() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    // TRAILING ZEROS, position of last ocurrence of 1, if any
    println!("{:?}", "// TRAILING ZEROS");
    let a: u8 = 7;
    let op_1: u8 = a.trailing_zeros() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    // TRAILING ONES, position of last ocurrence of 0, if any
    println!("{:?}", "// TRAILING ONES");
    let a: u8 = 7;
    let op_1: u8 = a.trailing_ones() as u8;
    println!("{:08b}", &a);
    println!("{:?}", &op_1);

    // WRAPPING ADDITION, modular addition
    println!("{:?}", "// WRAPPING ADDITION: 200u8 + 256u8");
    println!("{:08b}", 200u8);
    println!("{:08b}", 200u8.wrapping_add(u8::MAX));

    // WRAPPING SUBSTRACTION, modular difference
    println!("{:?}", "// WRAPPING SUBSTRACTION: 100u8 - 256u8");
    println!("{:08b}", 100u8);
    println!("{:08b}", 100u8.wrapping_sub(u8::MAX));

    // WRAPPING MULTIPLICATION, modular multiply
    println!("{:?}", "// WRAPPING MULTIPLICATION: 25u8 * 12u8 ");
    println!("{:08b}", 25u8);
    println!("{:08b}", 25u8.wrapping_mul(12));

    // WRAPPING REMAINDER, modular modulo
    println!("{:?}", "// WRAPPING REMAINDER 100u8 % 10u8");
    println!("{:08b}", 100u8);
    println!("{:08b}", 100u8.wrapping_rem(10));

    // WRAPPING EXPONENTIATION, modular power
    println!("{:?}", "// WRAPPING EXPONENTIATION 3u8.pow(6u8)");
    println!("{:08b}", 3u8);
    println!("{:08b}", 3u8.wrapping_pow(6));

    println!("{:?}", "// MODULAR/WRAPPING SHIFT LEFT: 150u8 << 25");
    // mask removes any high-order bits that would cause the shift to exceed the bitwidth of the type
    println!("{:08b}",150u8);
    println!("{:08b}",150u8.wrapping_shl(25));

    println!("{:?}", "MODULAR/WRAPPING SHIFT RIGHT: 150u8 >> 25");
    //where mask removes any high-order bits of rhs that would cause the shift to exceed the bitwidth of the type.
    println!("{:08b}",150u8);
    println!("{:08b}",150u8.wrapping_shr(25));

/*bitand_assign
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
-=*/


abs_diff
100u8.abs_diff(80)
100u8.abs_diff(110)


POWER OF 2
// Raises self to the power of exp, using exponentiation by squaring.
2u8.pow(5)


is_power_of_two
// Returns true if and only if self == 2^k for some k.
assert!(16u8.is_power_of_two());

next_power_of_two
// Returns the smallest power of two greater than or equal to self.
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