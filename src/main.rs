use rust_side::*;

fn main() {
	unsafe {
		let a1: u8 = u8::MAX;
		let a2: u16 = 2;
		let a3: u32 = u32::MAX;
		let a4: u64 = 4;
		let a5: isize = isize::MAX;
		println!("Rust  SEND {} {} {} {} {}", a1, a2, a3, a4, a5);
		helloFromSwift(a1, a2, a3, a4, a5);
		let params = HelloParams {
			a0: Inner {
				a0: 0,
				a1: 1,
				a2: 2,
			},
			a1: 1,
			a2: 2,
			a3: 3,
			a4: 4,
			a5: 6.,
		};
		println!("Rust  SEND {:?}", params);
		helloFromSwift2(params);
		// helloFromSwift2(HelloParams {
		// 	a1: u8::MAX,
		// 	a2: 2,
		// 	a3: u32::MAX,
		// 	a4: 4,
		// 	a5: std::f32::consts::PI,
		// 	a6: StaticString::from_str("This StaticString came from Rust!"),
		// });
	}
}
