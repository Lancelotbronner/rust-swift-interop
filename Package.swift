// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
	name: "rust-interop",
	products: [
		.library(name: "SwiftSide", type: .static, targets: ["SwiftSide"]),
	],
	targets: [
		.target(name: "SwiftSide"),
		.executableTarget(name: "swift-side", dependencies: [
			"SwiftSide",
		]),
	]
)
