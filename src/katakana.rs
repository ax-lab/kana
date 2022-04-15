pub fn to_katakana<T>(input: T) -> impl Iterator<Item = char>
where
	T: IntoIterator<Item = char>,
{
	let s = input.into_iter().collect::<String>();
	let s = match s.as_str() {
		"a" => "ア".into(),
		"i" => "イ".into(),
		"u" => "ウ".into(),
		"e" => "エ".into(),
		"o" => "オ".into(),
		_ => s,
	};
	s.chars().collect::<Vec<_>>().into_iter()
}

#[cfg(test)]
mod test_to_katakana {
	use super::to_katakana;

	macro_rules! check {
		($input:expr => $expected:expr) => {
			let output: String = to_katakana($input.chars()).collect();
			assert_eq!(
				output, $expected,
				"\n -> to_katakana(`{}`): expected `{}`, but got `{}`",
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
	fn can_convert_single_letter() {
		check!("a" => "ア");
		check!("i" => "イ");
		check!("u" => "ウ");
		check!("e" => "エ");
		check!("o" => "オ");
	}
}
