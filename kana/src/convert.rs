#[cfg(test)]
mod tests {
	use kana_macros::make_convert;

	make_convert!(to_ab =>
		"a" = "A",
		"b" = "B",
	);

	#[test]
	fn converts_simple_table() {
		assert_eq!(to_ab(""), "");
		assert_eq!(to_ab("a"), "A");
		assert_eq!(to_ab("b"), "B");
		assert_eq!(to_ab("abc"), "ABc");
	}
}
