import SwiftSide

struct Foo: Equatable {
	var a: UInt64
	var b: UInt8
}

struct Bar: Equatable {
	var x: Foo
	var y: (UInt8, UInt8, UInt8, UInt8, UInt8, UInt8, UInt8)

	static func == (lhs: Bar, rhs: Bar) -> Bool {
		lhs.x == rhs.x
			&& lhs.y.0 == rhs.y.0
			&& lhs.y.1 == rhs.y.1
			&& lhs.y.2 == rhs.y.2
			&& lhs.y.3 == rhs.y.3
			&& lhs.y.4 == rhs.y.4
			&& lhs.y.5 == rhs.y.5
			&& lhs.y.6 == rhs.y.6
	}
}

print("Bar \(MemoryLayout<Bar>.size) Foo \(MemoryLayout<Foo>.size)")
var bar = Bar(
	x: Foo(a: 0, b: 0),
	y: (1, 2, 3, 4, 5, 6, 7),
)
var new_x = Foo(
	a: .max,
	b: .max
)
swap(&bar.x, &new_x)
print(bar)
assert(bar == Bar(x: Foo(a: .max, b: .max), y: (1, 2, 3, 4, 5, 6, 7)))

print("\n[ Size / Stride Tests ]")

typealias FooTuple = (Foo, Foo)
typealias FooArray = [2 of Foo]

print("Foo", MemoryLayout<Foo>.size, MemoryLayout<Foo>.stride)
print("(Foo, Foo)", MemoryLayout<FooTuple>.size, MemoryLayout<FooTuple>.stride)
print("[2 of Foo]", MemoryLayout<FooArray>.size, MemoryLayout<FooArray>.stride)
