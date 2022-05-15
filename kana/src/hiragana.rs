use crate::convert::*;
use kana_macros::make_convert;

make_convert!(Hiragana =>
	"a" = "あ",
	"-" = "－",
	"hiragana" = "ひらがな",
	"nya" = ["にゃ", "んや"],
	"tta" = "った",
	"ttta" = "っった",
	"tttta" = "っっった",
);

pub fn to_hiragana<S: AsRef<str>>(input: S) -> Vec<String> {
	convert_all::<_, Hiragana>(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn accepts_empty_input() {
		let output = to_hiragana("");
		assert!(output.len() == 0);
	}

	#[test]
	fn converts_simple_romaji() {
		let output = to_hiragana("a");
		assert_eq!(output, &["あ"]);

		let output = to_hiragana("hiragana");
		assert_eq!(output, &["ひらがな"]);

		let output = to_hiragana("aaaaa");
		assert_eq!(output, &["あああああ"]);
	}

	#[test]
	fn passes_through_non_kana_input() {
		let output = to_hiragana("😀");
		assert_eq!(output, &["😀"]);

		let output = to_hiragana("çã");
		assert_eq!(output, &["çã"]);
	}

	#[test]
	fn supports_ambiguous_conversions() {
		let output = to_hiragana("nya");
		assert_eq!(output, &["にゃ", "んや"]);

		let output = to_hiragana("nya-nya");
		assert_eq!(
			output,
			&["にゃ－にゃ", "にゃ－んや", "んや－にゃ", "んや－んや"]
		);
	}

	#[test]
	fn supports_repeated_patterns() {
		let output = to_hiragana("tta");
		assert_eq!(output, &["った"]);

		let output = to_hiragana("ttta");
		assert_eq!(output, &["っった"]);

		let output = to_hiragana("tttta");
		assert_eq!(output, &["っっった"]);
	}
}
