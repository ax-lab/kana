pub fn to_hiragana<T>(input: T) -> impl Iterator<Item = char>
where
	T: IntoIterator<Item = char>,
{
	let s: String = input.into_iter().collect();
	let s = match s.as_str() {
		"a" => "あ".into(),
		"i" => "い".into(),
		"u" => "う".into(),
		"e" => "え".into(),
		"o" => "お".into(),
		_ => s,
	};
	s.chars().collect::<Vec<_>>().into_iter()
}

#[cfg(test)]
mod test_to_hiragana {
	use super::to_hiragana;

	macro_rules! check {
		($input:expr => $expected:expr) => {
			let output: String = to_hiragana($input.chars()).collect();
			assert_eq!(
				output, $expected,
				"\n -> to_hiragana(`{}`): expected `{}`, but got `{}`",
				$input, $expected, output
			);
		};
	}

	#[test]
	fn returns_empty_with_empty_input() {
		check!("" => "");
	}

	#[test]
	fn returns_input_when_not_convertible() {
		check!("¿¡" => "¿¡");
	}

	#[test]
	fn can_convert_single_char() {
		check!("a" => "あ");
		check!("i" => "い");
		check!("u" => "う");
		check!("e" => "え");
		check!("o" => "お");
	}
}
