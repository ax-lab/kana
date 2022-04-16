/// Provides an abstraction for text input that supports processing the input
/// segmented by characters.
///
/// This trait supports peeking at an arbitrary number of characters from the
/// input as a [`str`] slice and advancing the input by characters.
pub trait TextInput {
	/// Returns the next `count` characters from the input as a string slice
	/// without consuming the input.
	///
	/// At the end of the input, this may return less than `count` characters
	/// (including the empty string).
	fn peek_chars(&mut self, count: usize) -> &str;

	/// Consumes the next `count` characters from the input, stopping at the
	/// end of the input if there is not enough characters left.
	fn advance(&mut self, count: usize);
}

/// Implementation of [`TextInput`] for a borrowed [`str`] slice.
pub struct StrInput<'a> {
	source: &'a str,
	offset: usize,
}

impl<'a> StrInput<'a> {
	pub fn new(input: &'a str) -> StrInput<'a> {
		StrInput {
			source: input,
			offset: 0,
		}
	}
}

impl<'a> TextInput for StrInput<'a> {
	fn peek_chars(&mut self, count: usize) -> &str {
		let output = take_chars_from(self.source, self.offset, count);
		output
	}

	fn advance(&mut self, count: usize) {
		let text = self.peek_chars(count);
		self.offset += text.len();
	}
}

/// Implementation of [`TextInput`] for a owned [`String`].
pub struct StringInput {
	source: String,
	offset: usize,
}

impl StringInput {
	pub fn new(input: String) -> StringInput {
		StringInput {
			source: input,
			offset: 0,
		}
	}
}

impl TextInput for StringInput {
	fn peek_chars(&mut self, count: usize) -> &str {
		let output = take_chars_from(self.source.as_str(), self.offset, count);
		output
	}

	fn advance(&mut self, count: usize) {
		let text = self.peek_chars(count);
		self.offset += text.len();
	}
}

/// Implementation of [`TextInput`] for a [`Iterator`] of [`char`].
pub struct CharsInput<T>
where
	T: Iterator<Item = char>,
{
	source: T,
	buffer: String,
	buffer_count: usize,
}

impl<T> CharsInput<T>
where
	T: Iterator<Item = char>,
{
	pub fn new(input: T) -> CharsInput<T> {
		CharsInput {
			source: input,
			buffer: Default::default(),
			buffer_count: 0,
		}
	}
}

impl<T> TextInput for CharsInput<T>
where
	T: Iterator<Item = char>,
{
	fn peek_chars(&mut self, count: usize) -> &str {
		while self.buffer_count < count {
			if let Some(char) = self.source.next() {
				self.buffer.push(char);
				self.buffer_count += 1;
			} else {
				break;
			}
		}

		let buffer = self.buffer.as_str();
		if count < self.buffer_count {
			take_chars_from(buffer, 0, count)
		} else {
			buffer
		}
	}

	fn advance(&mut self, count: usize) {
		let advance_len = self.peek_chars(count).len();
		if count < self.buffer_count {
			self.buffer_count -= count;
			self.buffer.replace_range(0..advance_len, "");
		} else {
			self.buffer.clear();
			self.buffer_count = 0;
		}
	}
}

//============================================================================//
// Utilities
//============================================================================//

fn take_chars_from(input: &str, offset: usize, count: usize) -> &str {
	let input = unsafe { input.get_unchecked(offset..) };
	let length = if let Some((next_index, _)) = input.char_indices().nth(count) {
		next_index
	} else {
		input.len()
	};
	unsafe { input.get_unchecked(..length) }
}

//============================================================================//
// Tests
//============================================================================//

#[cfg(test)]
mod test_text_input {
	use super::CharsInput;
	use super::StrInput;
	use super::StringInput;
	use super::TextInput;

	#[test]
	fn from_str() {
		let mut input = StrInput::new("abc");
		assert_eq!(input.peek_chars(3), "abc");
	}

	#[test]
	fn from_string() {
		let mut input = StringInput::new(String::from("123"));
		assert_eq!(input.peek_chars(3), "123");
	}

	#[test]
	fn from_iterator() {
		let mut input = CharsInput::new("xyz".chars());
		assert_eq!(input.peek_chars(3), "xyz");
	}

	#[test]
	fn peek_does_not_consume_input() {
		fn check<T: TextInput>(mut input: T, name: &str) {
			assert_eq!(input.peek_chars(1), "1", "in {}", name);
			assert_eq!(input.peek_chars(3), "123", "in {}", name);
			assert_eq!(input.peek_chars(2), "12", "in {}", name);
			assert_eq!(input.peek_chars(1), "1", "in {}", name);
		}
		check(StrInput::new("123"), "StrInput");
		check(StringInput::new("123".into()), "StringInput");
		check(CharsInput::new("123".chars()), "CharsInput");
	}

	#[test]
	fn advance_consumes_input() {
		fn check<T: TextInput>(mut input: T, name: &str) {
			// scenarios:
			// 1) read then advance
			// 2) advance then read
			// 3) advance within read buffer
			// 4) advance across read buffer
			// 5) advance partially then read across buffer
			// 6) advance to end
			// 7) advance past end

			// 1
			assert_eq!(input.peek_chars(1), "1", "in {}", name);
			input.advance(1);
			assert_eq!(input.peek_chars(2), "23", "in {}", name);
			input.advance(2);
			assert_eq!(input.peek_chars(1), "4", "in {}", name);
			input.advance(1);

			// 2
			input.advance(1);
			assert_eq!(input.peek_chars(1), "6", "in {}", name);
			input.advance(1);

			// 3
			assert_eq!(input.peek_chars(4), "7890", "in {}", name);
			input.advance(1);
			assert_eq!(input.peek_chars(2), "89", "in {}", name);
			input.advance(2);
			assert_eq!(input.peek_chars(1), "0", "in {}", name);
			input.advance(1);

			// 4
			assert_eq!(input.peek_chars(1), "A", "in {}", name);
			input.advance(2);
			assert_eq!(input.peek_chars(1), "C", "in {}", name);
			input.advance(1);

			// 5
			assert_eq!(input.peek_chars(2), "DE", "in {}", name);
			input.advance(1);
			assert_eq!(input.peek_chars(2), "EF", "in {}", name);
			input.advance(2);

			// 6 and 7
			assert_eq!(input.peek_chars(1), "!", "in {}", name);
			input.advance(1);
			assert_eq!(input.peek_chars(1), "", "in {}", name);
			input.advance(1);
			assert_eq!(input.peek_chars(1), "", "in {}", name);
		}

		const INPUT: &'static str = "1234567890ABCDEF!";
		check(StrInput::new(INPUT), "StrInput");
		check(StringInput::new(INPUT.into()), "StringInput");
		check(CharsInput::new(INPUT.chars()), "CharsInput");
	}

	macro_rules! check_reads {
		($input:expr, $( $count:expr => $expected:expr ),*) => {
			let mut input1 = StrInput::new($input);
			let mut input2 = StringInput::new($input.to_string());
			let mut input3 = CharsInput::new($input.chars());
			$({
				let count = $count;
				let expected = $expected;

				let output = input1.peek_chars(count);
				assert_eq!(output, expected,
					"StrInput.read({}): expected `{}`, got `{}`",
					count, expected, output);

				let output = input2.peek_chars(count);
				assert_eq!(output, expected,
					"StringInput.read({}): expected `{}`, got `{}`",
					count, expected, output);

				let output = input3.peek_chars(count);
				assert_eq!(output, expected,
					"CharsInput.read({}): expected `{}`, got `{}`",
					count, expected, output);

				input1.advance(count);
				input2.advance(count);
				input3.advance(count);
			})+
		};
	}

	#[test]
	fn with_empty_input() {
		check_reads!("", 1 => "");
	}

	#[test]
	fn can_read_whole_input() {
		check_reads!("input", 5 => "input");
	}

	#[test]
	fn returns_empty_at_end_of_input() {
		check_reads!("123", 3 => "123", 3 => "", 1 => "");
	}

	#[test]
	fn returns_count_characters() {
		check_reads!("12345", 1 => "1", 1 => "2", 2 => "34", 1 => "5");
		check_reads!("12345", 3 => "123", 3 => "45");
	}

	#[test]
	fn supports_multibyte_characters() {
		check_reads!("日本語が上手ですね", 2 => "日本", 1 => "語", 1 => "が", 2 => "上手", 3 => "ですね");
	}
}
