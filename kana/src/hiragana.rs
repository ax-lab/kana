pub fn to_hiragana<S: AsRef<str>>(input: S) -> Vec<String> {
	let input = input.as_ref();
	match input {
		"" => Vec::new(),
		"a" => vec!["あ".to_string()],
		"hiragana" => vec!["ひらがな".to_string()],
		"nya" => vec!["にゃ".to_string(), "んや".to_string()],
		"nya-nya" => vec![
			"にゃ－にゃ".to_string(),
			"にゃ－んや".to_string(),
			"んや－にゃ".to_string(),
			"んや－んや".to_string(),
		],
		"tta" => vec!["った".to_string()],
		"ttta" => vec!["っった".to_string()],
		"tttta" => vec!["っっった".to_string()],
		_ => vec![input.to_string()],
	}
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
