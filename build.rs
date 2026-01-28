use std::env;

fn main() {
	let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

	println!("cargo::rustc-link-search=.build/arm64-apple-macosx/debug");
	println!("cargo::rustc-link-lib=SwiftSide");

	println!("cargo::rustc-link-search=/Library/Developer/Toolchains/swift-latest.xctoolchain/usr/lib/swift/macosx");
}
