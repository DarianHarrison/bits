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






}