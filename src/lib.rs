#![feature(abi_swift)]
#![feature(stride)]

mod tests;

use core::mem::{align_of, size_without_padding_of, stride_of, offset_of, transmute};

unsafe extern "Swift" {
	#[unsafe(no_mangle)]
	pub fn helloFromSwift(a1: u8, a2: u16, a3: u32, a4: u64, a5: isize);

	#[unsafe(no_mangle)]
	pub fn helloFromSwift2(params: HelloParams);
}

#[unsafe(no_mangle)]
pub extern "Swift" fn helloFromRust(msg: u32) {
	println!("Rust  RECV {}", msg);
	println!("Rust  LAYOUT:");
	println!("	StaticString {} {} {} HelloParams {} {} {}", size_without_padding_of::<Inner>(), stride_of::<Inner>(), align_of::<StaticString>(), size_without_padding_of::<HelloParams>(), stride_of::<HelloParams>(), align_of::<HelloParams>());
	println!("	HelloParams {} {} {} {} {} {}", offset_of!(HelloParams, a0), offset_of!(HelloParams, a1), offset_of!(HelloParams, a2), offset_of!(HelloParams, a3), offset_of!(HelloParams, a4), offset_of!(HelloParams, a5));
	println!("	StaticString {} {} {}", offset_of!(StaticString, start_ptr_or_data), offset_of!(StaticString, utf8_code_unit_count), offset_of!(StaticString, flags));
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "Swift" fn helloFromRust2(msg: StaticString) {
	println!("Rust  RECV {:?}", msg.as_str());
	println!("	{:?}", msg);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "Swift" fn helloFromRust3(msg: HelloParams) {
	println!("Rust  RECV {:?}", msg);
	println!("	a0.a0 {:?}", &msg.a0.a0);
	println!("	a0.a1 {:?}", &msg.a0.a1);
	println!("	a0.a2 {:?}", &msg.a0.a2);
	println!("	a1 {:?}", &msg.a1);
	println!("	a2 {:?}", &msg.a2);
	println!("	a3 {:?}", &msg.a3);
	println!("	a4 {:?}", &msg.a4);
	println!("	a5 {:?}", &msg.a5);
}

#[derive(Copy, Clone, Debug)]
#[repr(Swift)]
pub struct Inner {
	pub a0: isize,
	pub a1: isize,
	pub a2: i8,
}

#[derive(Copy, Clone, Debug)]
#[repr(Swift)]
pub struct HelloParams {
	pub a0: Inner,
	pub a1: u8,
	pub a2: u16,
	pub a3: u32,
	pub a4: u64,
	pub a5: f32,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(Swift)]
pub struct StaticString {
	pub start_ptr_or_data: isize,
	pub utf8_code_unit_count: isize,
	pub flags: i8,
}

impl StaticString {
	pub fn from_str(data: &'static str) -> Self {
		let pair: (isize, isize) = unsafe { transmute(data) };
		StaticString {
			start_ptr_or_data: pair.0,
			utf8_code_unit_count: pair.1,
			flags: 0b11,
		}
	}

	pub fn as_str(&self) -> &'static str {
		unsafe { transmute((self.start_ptr_or_data, self.utf8_code_unit_count)) }
	}
}
