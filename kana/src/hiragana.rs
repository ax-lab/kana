use crate::convert::*;
use kana_macros::make_convert;

make_convert!(Hiragana =>
	"-" = "－",

	"a" = "あ",
	"i" = "い",
	"u" = "う",
	"e" = "え",
	"o" = "お",

	"ca" = "か",
	"ci" = "し",
	"cu" = "く",
	"ce" = "せ",
	"co" = "こ",

	"ka" = "か",
	"ki" = "き",
	"ku" = "く",
	"ke" = "け",
	"ko" = "こ",

	"kya" = "きゃ",
	"kyi" = "きぃ",
	"kyu" = "きゅ",
	"kye" = "きぇ",
	"kyo" = "きょ",

	"ga" = "が",
	"gi" = "ぎ",
	"gu" = "ぐ",
	"ge" = "げ",
	"go" = "ご",

	"gya" = "ぎゃ",
	"gyi" = "ぎぃ",
	"gyu" = "ぎゅ",
	"gye" = "ぎぇ",
	"gyo" = "ぎょ",

	"sa" = "さ",
	"si" = "し",
	"su" = "す",
	"se" = "せ",
	"so" = "そ",

	"sha" = "しゃ",
	"shi" = "し",
	"shu" = "しゅ",
	"she" = "しぇ",
	"sho" = "しょ",

	"sya" = "しゃ",
	"syi" = "しぃ",
	"syu" = "しゅ",
	"sye" = "しぇ",
	"syo" = "しょ",

	"za" = "ざ",
	"zi" = "じ",
	"zu" = "ず",
	"ze" = "ぜ",
	"zo" = "ぞ",

	"zya" = "じゃ",
	"zyi" = "じぃ",
	"zyu" = "じゅ",
	"zye" = "じぇ",
	"zyo" = "じょ",

	"ja" = "じゃ",
	"ji" = "じ",
	"ju" = "じゅ",
	"je" = "じぇ",
	"jo" = "じょ",

	"jya" = "じゃ",
	"jyi" = "じぃ",
	"jyu" = "じゅ",
	"jye" = "じぇ",
	"jyo" = "じょ",

	"ta" = "た",
	"ti" = "ち",
	"tu" = "つ",
	"tsu" = "つ",
	"te" = "て",
	"to" = "と",

	"tya" = "ちゃ",
	"tyi" = "ちぃ",
	"tyu" = "ちゅ",
	"tye" = "ちぇ",
	"tyo" = "ちょ",

	"cha" = "ちゃ",
	"chi" = "ち",
	"chu" = "ちゅ",
	"che" = "ちぇ",
	"cho" = "ちょ",

	"da" = "だ",
	"di" = "ぢ",
	"du" = "づ",
	"de" = "で",
	"do" = "ど",

	"dya" = "ぢゃ",
	"dyi" = "ぢぃ",
	"dyu" = "ぢゅ",
	"dye" = "ぢぇ",
	"dyo" = "ぢょ",

	"dja" = "ぢゃ",
	"dji" = "ぢぃ",
	"dju" = "ぢゅ",
	"dje" = "ぢぇ",
	"djo" = "ぢょ",

	"dza" = "ぢゃ",
	"dzi" = "ぢぃ",
	"dzu" = "づ",
	"dze" = "ぢぇ",
	"dzo" = "ぢょ",

	"na" = ["な", "んあ"],
	"ni" = ["に", "んい"],
	"nu" = ["ぬ", "んう"],
	"ne" = ["ね", "んえ"],
	"no" = ["の", "んお"],

	"nya" = ["にゃ", "んや"],
	"nyi" = ["にぃ"],
	"nyu" = ["にゅ", "んゆ"],
	"nye" = ["にぇ"],
	"nyo" = ["にょ", "んよ"],

	"ha" = "は",
	"hi" = "ひ",
	"hu" = "ふ",
	"he" = "へ",
	"ho" = "ほ",

	"fa" = "ふぁ",
	"fi" = "ふぃ",
	"fu" = "ふ",
	"fe" = "ふぇ",
	"fo" = "ふぉ",

	"hya" = "ひゃ",
	"hyi" = "ひぃ",
	"hyu" = "ひゅ",
	"hye" = "ひぇ",
	"hyo" = "ひょ",

	"ba" = "ば",
	"bi" = "び",
	"bu" = "ぶ",
	"be" = "べ",
	"bo" = "ぼ",

	"bya" = "びゃ",
	"byi" = "びぃ",
	"byu" = "びゅ",
	"bye" = "びぇ",
	"byo" = "びょ",

	"pa" = "ぱ",
	"pi" = "ぴ",
	"pu" = "ぷ",
	"pe" = "ぺ",
	"po" = "ぽ",

	"pya" = "ぴゃ",
	"pyi" = "ぴぃ",
	"pyu" = "ぴゅ",
	"pye" = "ぴぇ",
	"pyo" = "ぴょ",

	"ma" = "ま",
	"mi" = "み",
	"mu" = "む",
	"me" = "め",
	"mo" = "も",

	"mya" = "みゃ",
	"myi" = "みぃ",
	"myu" = "みゅ",
	"mye" = "みぇ",
	"myo" = "みょ",

	"ya" = "や",
	"yi" = "いぃ",
	"yu" = "ゆ",
	"ye" = "いぇ",
	"yo" = "よ",

	"ra" = "ら",
	"ri" = "り",
	"ru" = "る",
	"re" = "れ",
	"ro" = "ろ",

	"rya" = "りゃ",
	"ryi" = "りぃ",
	"ryu" = "りゅ",
	"rye" = "りぇ",
	"ryo" = "りょ",

	"wa" = "わ",
	"wi" = "うぃ",
	"wu" = "う",
	"we" = "うぇ",
	"wo" = "を",

	"va" = "ゔぁ",
	"vi" = "ゔぃ",
	"vu" = "ゔ",
	"ve" = "ゔぇ",
	"vo" = "ゔぉ",

	"qua" = "くぁ",
	"qui" = "くぃ",
	"que" = "くぇ",
	"quo" = "くぉ",
	"qu" = "く",

	"qwa" = "くぁ",
	"qwi" = "くぃ",
	"qwu" = "くぅ",
	"qwe" = "くぇ",
	"qwo" = "くぉ",

	"n" = "ん",

	"c"+"c" = "っ",
	"k"+"k" = "っ",
	"g"+"g" = "っ",
	"s"+"s" = "っ",
	"z"+"z" = "っ",
	"j"+"j" = "っ",
	"t"+"t" = "っ",
	"d"+"d" = "っ",
	"h"+"h" = "っ",
	"f"+"f" = "っ",
	"b"+"b" = "っ",
	"p"+"p" = "っ",
	"m"+"m" = "っ",
	"y"+"y" = "っ",
	"r"+"r" = "っ",
	"w"+"w" = "っ",
	"q"+"q" = "っ",
	"v"+"v" = "っ",
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
		assert_eq!(output, &["ひらがな", "ひらがんあ"]);

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
