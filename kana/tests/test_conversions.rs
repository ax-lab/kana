#[test]
fn hiragana() {
	tux::testdata("tests/hiragana", |input| {
		input
			.into_iter()
			.map(|source| {
				let output = kana::to_hiragana(&source);
				let output = output.join(" | ");
				format!("{}: {}", source, output)
			})
			.collect()
	})
}
