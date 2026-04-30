use std::mem;

#[derive(Clone, PartialEq, Debug)]
#[repr(Swift)]
struct Foo {
	a: u64,
	b: u8,
}

#[derive(Clone, PartialEq, Debug)]
#[repr(Swift)]
struct Bar {
	x: Foo,
	y: [u8; 7],
}

#[test]
fn unaligned_fields() {
	println!("Bar {} Foo {}", mem::size_of::<Bar>(), mem::size_of::<Foo>());
	let mut bar = Bar {
		x: Foo { a: 0, b: 0 },
		y: [1, 2, 3, 4, 5, 6, 7],
	};
	let x = &mut bar.x;
	let mut new_x = Foo {
		a: u64::MAX,
		b: u8::MAX
	};
	mem::swap(x, &mut new_x);
	println!("{:#?}", bar);
	assert_eq!(bar, Bar {
		x: Foo { a: u64::MAX, b: u8::MAX },
		y: [1, 2, 3, 4, 5, 6, 7]
	});
}
