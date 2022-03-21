#![feature(int_log)]
#[allow(unused_variables)]

fn bit_permute_step(x: u32, m: u32, shift: u32) -> u32 {
    let t: u32 = ((x >> shift) ^ x) & m;
    let x: u32 = (x ^ t) ^ (t << shift);
    x
}

fn main() {

    // AND, minimizes
    let a: u8 = 230;
    let b: u8 = 7;
    let op_1: u8 = a & b;
    println!("{:?}", "// AND");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // OR, maximizes
    let a: u8 = 230;
    let b: u8 = 7;
    let op_1: u8 = a | b;
    println!("{:?}", "// OR");
    println!("{:08b}", &a);
    println!("{:08b}", &b);
    println!("{:08b}", &op_1);

    // XOR, find bitwise magnets
    let a: u8 = 230;
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

    // += // add assign
    // -= // sub assign
    // *= // mul assign
    // /= // div assign
    // ^= // bitxor assign
    // &= // bitand assign
    // |= // bitor assign
    // %= // rem assign
    // <<= // shl assign
    // >>= // shr assign

    // bitwise concatenation
    let bit_vector: Vec<u8> = vec![255,1,255,1];
    let mut concat:u32 = bit_vector[0] as u32;
    for hask_k in bit_vector {
    	concat <<= 8;
        concat |= hask_k as u32;
    }
    println!("last {:32b}", &concat);
    println!("{:?}", &concat);

    // bitwise permutation
    let mut x: u32 = 123789;
    println!("{:032b}", &x);
    println!("{:?}", x.count_ones());
    x = bit_permute_step(x, 0x22222222, 1);  // Bit index swap 0,1
    x = bit_permute_step(x, 0x0c0c0c0c, 2);  // Bit index swap 1,2
    x = bit_permute_step(x, 0x00f000f0, 4);  // Bit index swap 2,3
    x = bit_permute_step(x, 0x0000ff00, 8);  // Bit index swap 3,4
    println!("{:032b}", &x);
    println!("{:?}", x.count_ones());

    // base 2 powering with shift
    let d_pow: u32 = 32; // this example is 2^32
    let d: u32;
    if d_pow == u32::BITS {
        d = u32::MAX;
    } else {
        d = 2 << (d_pow - 1);
    }
    assert_eq!(d, u32::MAX);

    ///////////////////////////////////
    ///////////////////////////////////
    ///////////////////////////////////


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

    ///////////////////////////////////
    ///////////////////////////////////
    ///////////////////////////////////

    // log2
    //Returns the base 2 logarithm of the number, rounded down.
    //#![feature(int_log)]
    //assert_eq!(2u8.log2(), 1);

    // log10
    // Returns the base 10 logarithm of the number, rounded down.
    //#![feature(int_log)]
    //assert_eq!(10u8.log10(), 1);


    ///////////////////////////////////
    ///////////////////////////////////
    ///////////////////////////////////


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

    // WRAPPING SHIFT LEFT, modular exponentiation
    println!("{:?}", "// WRAPPING SHIFT LEFT: 150u8 << 25");
    // mask removes any high-order bits that would cause the shift to exceed the bitwidth of the type
    println!("{:08b}",150u8);
    println!("{:08b}",150u8.wrapping_shl(25));

    // WRAPPING EXPONENTIATION, modular right shift
    println!("{:?}", "// WRAPPING SHIFT RIGHT: 150u8 >> 25");
    //where mask removes any high-order bits of rhs that would cause the shift to exceed the bitwidth of the type.
    println!("{:08b}",150u8);
    println!("{:08b}",150u8.wrapping_shr(25));


    ///////////////////////////////////
    ///////////////////////////////////
    ///////////////////////////////////


    println!("{:?}", "// ABSOLUTE DIFFERENCE: 100 110");
    println!("{:?}", 100u8.abs_diff(110)); 

    // IS POWER OF 2
    println!("{:?}", "// IS POWER OF 2: 16");
    println!("{:?}", 16u8.is_power_of_two());

    // NEXT POWER OF 2
    println!("{:?}", "// NEXT POWER OF 2: 12");
    println!("{:?}", 12u8.next_power_of_two());

    // BASE 2 LOGARITHM
    
    // exact powers of 2
    let a: u32 = 134217728u32.trailing_zeros(); // exact power of 2
    println!("base_2_log: {:?}", a); 

    // inexact powers of 2: floor
    let a: u32 = 134217727u32.log2(); // not exact power of 2
    println!("base_2_log: {:?}", a); 

    // inexact powers of 2: ceil // not exact power of 2
    let a: u32 = 134217727u32 - 1u32; // not exact power of 2
    let a: u32 = a.log2() + 1;
    println!("base_2_log: {:?}", a); 


    // MIN VALUE FOR TYPE, min
    println!("{:?}", "// MIN VALUE FOR TYPE u8");
    println!("{:08b}", u8::MIN);

    // MAX VALUE FOR TYPE, max
    println!("{:?}", "// MAX VALUE FOR TYPE u8");
    println!("{:08b}", u8::MAX);

    // SATURATING ADDITION, max if overflows
    println!("{:?}", "// SATURATING ADDITION: 255 + 127 for u8");
    println!("{:?}", u8::MAX.saturating_add(127));

    // SATURATING SUBSTRACTION, min if overflows
    println!("{:?}", "// SATURATING SUBSTRACTION: 13 - 127 for u8");
    println!("{:?}",13u8.saturating_sub(127));

}

