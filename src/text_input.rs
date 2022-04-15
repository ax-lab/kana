/// Provides an abstraction for text input that supports reading a string
/// with the specified number of characters at a time.
pub trait TextInput {
	/// Read the next `count` characters from the input and return them as a
	/// string slice.
	///
	/// At the end of the input, this may return less than `count` characters
	/// (including the empty string).
	fn read_chars(&mut self, count: usize) -> &str;
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
	fn read_chars(&mut self, count: usize) -> &str {
		let output = take_chars_from(self.source, self.offset, count);
		self.offset += output.len();
		output
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
	fn read_chars(&mut self, count: usize) -> &str {
		let output = take_chars_from(self.source.as_str(), self.offset, count);
		self.offset += output.len();
		output
	}
}

/// Implementation of [`TextInput`] for a [`Iterator`] of [`char`].
pub struct CharsInput<T>
where
	T: Iterator<Item = char>,
{
	source: T,
	buffer: String,
}

impl<T> CharsInput<T>
where
	T: Iterator<Item = char>,
{
	pub fn new(input: T) -> CharsInput<T> {
		CharsInput {
			source: input,
			buffer: Default::default(),
		}
	}
}

impl<T> TextInput for CharsInput<T>
where
	T: Iterator<Item = char>,
{
	fn read_chars(&mut self, count: usize) -> &str {
		self.buffer.clear();

		let mut read = 0;
		while read < count {
			if let Some(char) = self.source.next() {
				self.buffer.push(char);
				read += 1;
			} else {
				break;
			}
		}

		self.buffer.as_str()
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

	macro_rules! check {
		($input:expr, $( $count:expr => $expected:expr ),*) => {
			let mut input1 = StrInput::new($input);
			let mut input2 = StringInput::new($input.to_string());
			let mut input3 = CharsInput::new($input.chars());
			$({
				let count = $count;
				let expected = $expected;

				let output = input1.read_chars(count);
				assert_eq!(output, expected,
					"StrInput.read({}): expected `{}`, got `{}`",
					count, expected, output);

				let output = input2.read_chars(count);
				assert_eq!(output, expected,
					"StringInput.read({}): expected `{}`, got `{}`",
					count, expected, output);

				let output = input3.read_chars(count);
				assert_eq!(output, expected,
					"CharsInput.read({}): expected `{}`, got `{}`",
					count, expected, output);
			})+
		};
	}

	#[test]
	fn from_str() {
		let mut input = StrInput::new("abc");
		assert_eq!(input.read_chars(3), "abc");
	}

	#[test]
	fn from_string() {
		let mut input = StringInput::new(String::from("123"));
		assert_eq!(input.read_chars(3), "123");
	}

	#[test]
	fn from_iterator() {
		let mut input = CharsInput::new("xyz".chars());
		assert_eq!(input.read_chars(3), "xyz");
	}

	#[test]
	fn with_empty_input() {
		check!("", 1 => "");
	}

	#[test]
	fn can_read_whole_input() {
		check!("input", 5 => "input");
	}

	#[test]
	fn returns_empty_at_end_of_input() {
		check!("123", 3 => "123", 3 => "", 1 => "");
	}

	#[test]
	fn returns_count_characters() {
		check!("12345", 1 => "1", 1 => "2", 2 => "34", 1 => "5");
		check!("12345", 3 => "123", 3 => "45");
	}

	#[test]
	fn supports_multibyte_characters() {
		check!("日本語が上手ですね", 2 => "日本", 1 => "語", 1 => "が", 2 => "上手", 3 => "ですね");
	}
}
