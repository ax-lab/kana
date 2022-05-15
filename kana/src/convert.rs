#[allow(dead_code)]
pub enum Match {
	None,
	Text(&'static str),
	List(&'static str, Vec<&'static str>),
}

impl Match {
	pub fn first_target(&self) -> Option<&'static str> {
		match self {
			Match::None => None,
			Match::Text(text) => Some(text),
			Match::List(text, _) => Some(text),
		}
	}
}

pub trait Conversion {
	fn convert_next(input: &str) -> (usize, Match);
}

#[allow(dead_code)]
pub fn convert<S: AsRef<str>, T: Conversion>(input: S) -> String {
	let mut input = input.as_ref();
	let mut output = String::new();
	while input.len() > 0 {
		let (skip_len, matched) = T::convert_next(input);
		let target_text = if let Some(text) = matched.first_target() {
			text
		} else {
			&input[..skip_len]
		};
		output.push_str(target_text);
		input = &input[skip_len..];
	}

	output
}

#[allow(dead_code)]
pub fn convert_all<S: AsRef<str>, T: Conversion>(input: S) -> Vec<String> {
	let mut input = input.as_ref();
	let mut output = String::new();
	while input.len() > 0 {
		let (skip_len, matched) = T::convert_next(input);
		match matched {
			Match::None => {
				let literal = &input[..skip_len];
				output.push_str(literal);
			}
			Match::Text(target) => {
				output.push_str(target);
			}
			Match::List(head, tail) => {
				let next_output = convert_all::<_, T>(&input[skip_len..]);
				let mut result = Vec::new();

				let push = |result: &mut Vec<String>, text: &str| {
					for it in next_output.iter() {
						let mut item = String::with_capacity(output.len() + text.len() + it.len());
						item.push_str(&output);
						item.push_str(text);
						item.push_str(it);
						result.push(item);
					}
				};

				push(&mut result, head);
				for it in tail {
					push(&mut result, it);
				}

				return result;
			}
		}
		input = &input[skip_len..];
	}
	vec![output]
}

#[cfg(test)]
mod tests {
	use super::*;
	use kana_macros::make_convert;

	make_convert!(Empty =>
		"a" = [],
		"b" = [],
		"c" = [],
	);

	#[test]
	fn converts_ignores_empty_targets() {
		assert_eq!(convert::<_, Empty>("abc"), "abc");
	}

	make_convert!(AB =>
		"a" = "A",
		"b" = "B",
	);

	#[test]
	fn converts_simple_table() {
		assert_eq!(convert::<_, AB>(""), "");
		assert_eq!(convert::<_, AB>("a"), "A");
		assert_eq!(convert::<_, AB>("b"), "B");
		assert_eq!(convert::<_, AB>("abc"), "ABc");
	}

	make_convert!(Multi =>
		"a" = ["A", "Ae", "@"],
		"b" = ["B", "Be"],
		"c" = ["C", "Ce"],
	);

	#[test]
	fn converts_ambiguous_input_to_multiple_matches() {
		assert_eq!(convert_all::<_, Multi>("a"), &["A", "Ae", "@"]);
		assert_eq!(convert_all::<_, Multi>("b"), &["B", "Be"]);
		assert_eq!(convert_all::<_, Multi>("c"), &["C", "Ce"]);
		assert_eq!(
			convert_all::<_, Multi>("abc"),
			&[
				"ABC", "ABCe", "ABeC", "ABeCe", "AeBC", "AeBCe", "AeBeC", "AeBeCe", "@BC", "@BCe",
				"@BeC", "@BeCe"
			]
		);
	}

	#[test]
	fn convert_returns_first_conversion_for_ambiguous_matches() {
		assert_eq!(convert::<_, Multi>("abc-abc"), "ABC-ABC");
	}
}
