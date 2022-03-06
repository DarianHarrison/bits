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


    // 100u8.saturating_add(1) // Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.


    // Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    //assert_eq!(100u8.saturating_sub(27), 73);

}