#![feature(abi_swift)]

#[derive(Debug)]
pub struct StaticString {
	a1: isize,
	a2: isize,
	a3: i8
}

#[derive(Debug)]
pub struct Large {
	a: isize,
	b: isize,
	c: isize,
	d: isize,
	e: isize,
	f: isize,
}

#[unsafe(no_mangle)]
pub unsafe extern "Swift" fn test1(msg: StaticString) {
	println!("{:?}", msg);
}

#[unsafe(no_mangle)]
pub unsafe extern "Swift" fn test2(msg: Large) {
	println!("{:?}", msg);
}
