//
//  Hello.swift
//  rust-interop
//
//  Created by Christophe Bronner on 2026-01-26.
//

@_silgen_name("helloFromSwift")
public func helloFromSwift(_ a1: UInt8, _ a2: UInt16, _ a3: UInt32, _ a4: UInt64, _ a5: Int) {
	print("Swift RECV", a1, a2, a3, a4, a5)
	print("Swift LAYOUT:")
	print("\tStaticString \(MemoryLayout<StaticString>.size) \(MemoryLayout<StaticString>.stride) \(MemoryLayout<StaticString>.alignment) HelloParams \(MemoryLayout<HelloParams>.size) \(MemoryLayout<HelloParams>.stride) \(MemoryLayout<HelloParams>.alignment)")
	print(
		"\tHelloParams",
		MemoryLayout<HelloParams>.offset(of: \.a0) ?? -1,
		MemoryLayout<HelloParams>.offset(of: \.a1) ?? -1,
		MemoryLayout<HelloParams>.offset(of: \.a2) ?? -1,
		MemoryLayout<HelloParams>.offset(of: \.a3) ?? -1,
		MemoryLayout<HelloParams>.offset(of: \.a4) ?? -1,
		MemoryLayout<HelloParams>.offset(of: \.a5) ?? -1,
	)
	print(
		"\tStaticString",
		MemoryLayout<Inner>.offset(of: \.a0) ?? -1,
		MemoryLayout<Inner>.offset(of: \.a1) ?? -1,
		MemoryLayout<Inner>.offset(of: \.a2) ?? -1,
	)
	let msg: UInt32 = 42
	print("Swift SEND \(msg)")
	helloFromRust(msg)
}

@_silgen_name("helloFromSwift2")
public func helloFromSwift2(_ params: HelloParams) {
	print("Swift RECV", params)
	let msg: StaticString = "Hello Rust, this is Swift!"
	print("Swift SEND", msg)
	print("\t\(unsafeBitCast(msg, to: Inner.self))")
	helloFromRust2(unsafeBitCast(msg, to: Inner.self))
	var params = params
	params.a1 *= 10
	params.a2 += 10000
	params.a3 &-= 10000
	params.a4 %= 3
	params.a5 *= .pi * 1000
	helloFromRust3(params)
}

@_silgen_name("helloFromRust")
public func helloFromRust(_ msg: UInt32)

@_silgen_name("helloFromRust2")
public func helloFromRust2(_ msg: Inner)

@_silgen_name("helloFromRust3")
public func helloFromRust3(_ msg: HelloParams)

public struct Inner: CustomStringConvertible {
	var a0: Int
	var a1: Int
	var a2: Int8

	public var description: String {
		"Inner { a0: \(a0), a1: \(a1), a2: \(a2) }"
	}
}

public struct HelloParams: CustomStringConvertible {
	var a0: Inner
	var a1: UInt8
	var a2: UInt16
	var a3: UInt32
	var a4: UInt64
	var a5: Float

	public var description: String {
		"HelloParams { a0: \(a0), a1: \(a1), a2: \(a2), a3: \(a3), a4: \(a4), a5: \(a5) }"
	}
}
