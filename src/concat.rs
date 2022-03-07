fn main() {
	// bitwise concatenation
	let mut concat:u32 = 3u32;
	for _ in 0..4{
		concat <<= 7;
		concat |= 3u32;
	}
	println!("{:?}", &concat);
	println!("{:b}", &concat);



	// bitwise permutation
	let mut m: u32 = 3;
	let mut x: u32 = 12398;
	println!("{:032b}", &x);
	println!("{:?}", x.count_ones());
	println!("{:?}", x.count_zeros());
	let mut shift: u32 = 8;
	let mut t: u32;
	t = ((x >> shift) ^ x) & m;
	x = (x ^ t) ^ (t << shift);
	println!("{:032b}", &x);
	println!("{:?}", x.count_ones());
	println!("{:?}", x.count_zeros());
}