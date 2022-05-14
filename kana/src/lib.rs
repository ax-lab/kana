mod hiragana;
pub use hiragana::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn to_hiragana_works() {
		let output = to_hiragana("a");
		assert_eq!(output, &["あ"]);

		let output = to_hiragana("hiragana");
		assert_eq!(output, &["ひらがな"]);
	}
}
