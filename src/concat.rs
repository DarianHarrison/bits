fn main() {
	// bitwise concatenation
	let mut concat:u32 = 3u32;
	for _ in 0..4{
		concat <<= 7;
		concat |= 3u32;
	}
	println!("{:?}", &concat);
	println!("{:b}", &concat);
}