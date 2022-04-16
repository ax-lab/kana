use super::text_input::*;

pub struct Input<T>
where
	T: TextInput,
{
	source: T,
}

impl<T> Input<T>
where
	T: TextInput,
{
	pub fn new(source: T) -> Self {
		Input { source }
	}

	pub fn peek(&mut self, count: usize) -> &str {
		self.source.peek_chars(count)
	}

	pub fn advance(&mut self, count: usize) {
		self.source.advance(count);
	}
}

impl<'a> From<&'a str> for Input<StrInput<'a>> {
	fn from(input: &'a str) -> Self {
		Input::new(StrInput::new(input))
	}
}

impl From<String> for Input<StringInput> {
	fn from(input: String) -> Self {
		Input::new(StringInput::new(input))
	}
}

impl<T> From<T> for Input<CharsInput<T::IntoIter>>
where
	T: IntoIterator<Item = char>,
{
	fn from(input: T) -> Self {
		Input::new(CharsInput::new(input.into_iter()))
	}
}

//============================================================================//
// Tests
//============================================================================//

#[cfg(test)]
mod test_input {
	use super::Input;

	#[test]
	fn can_create_from_string() {
		let mut input = Input::from("abc");
		assert_eq!(input.peek(3), "abc");

		let mut input = Input::from("123".to_string());
		assert_eq!(input.peek(3), "123");
	}

	#[test]
	fn can_create_from_chars() {
		let mut input = Input::from("456".chars());
		assert_eq!(input.peek(3), "456");
	}

	#[test]
	fn advance_moves_input() {
		let mut input = Input::from("123");
		assert_eq!(input.peek(1), "1");
		input.advance(1);
		assert_eq!(input.peek(1), "2");
		input.advance(1);
		assert_eq!(input.peek(1), "3");
		input.advance(1);
		assert_eq!(input.peek(1), "");
	}

	#[test]
	fn peek_does_not_advance_input() {
		let mut input = Input::from("123");
		assert_eq!(input.peek(1), "1");
		assert_eq!(input.peek(2), "12");
		assert_eq!(input.peek(3), "123");
	}

	#[test]
	fn read_returns_empty_at_the_end() {
		let mut input = Input::from("");
		assert!(input.peek(1).len() == 0);

		let mut input = Input::from("abc");
		input.advance(3);
		assert!(input.peek(1).len() == 0);
	}
}
