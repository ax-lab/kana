use proc_macro::TokenStream;
use proc_macro2::*;
use quote::quote;
use syn::{parse::Parse, *};

#[proc_macro]
pub fn make_convert(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as ConversionTable);

	let name = input.name;

	let pairs = input.pairs.into_iter().filter_map(|it| {
		let source = it.source;
		let lookahead = format!("{}{}", source, it.lookahead);
		let target = it.target;
		let target = match target.len() {
			0 => {
				return None;
			}
			1 => {
				let target = &target[0];
				quote! { Match::Text(#target) }
			}
			_ => {
				let head = &target[0];
				let tail = target.iter().skip(1);
				quote! { Match::List(#head, vec![ #( #tail,)* ])}
			}
		};

		let output = quote! {
			if input.starts_with(#lookahead) {
				return (#source.len(), #target);
			}
		};
		Some(output)
	});

	let pairs = pairs.collect::<Vec<_>>();

	let tokens = quote! {
		struct #name;

		impl Conversion for #name {
			fn convert_next(mut input: &str) -> (usize, Match) {
				#( #pairs )*

				let skip_length = if let Some((index, _)) = input.char_indices().nth(1) {
					index
				} else {
					input.len()
				};
				(skip_length, Match::None)
			}
		}
	};

	tokens.into()
}

struct ConversionTable {
	pub name: Ident,
	pub pairs: Vec<ConversionRow>,
}

impl Parse for ConversionTable {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut table = ConversionTable {
			name: input.call(Ident::parse)?,
			pairs: Vec::new(),
		};

		input.parse::<Token![=>]>()?;

		let pairs = input.parse_terminated::<_, Token![,]>(ConversionRow::parse)?;
		table.pairs.extend(pairs);

		Ok(table)
	}
}

struct ConversionRow {
	pub source: String,
	pub lookahead: String,
	pub target: Vec<String>,
}

impl Parse for ConversionRow {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let source: LitStr = input.parse()?;
		let source = source.value();

		let lookahead = if input.peek(Token![+]) {
			input.parse::<Token![+]>().unwrap();
			let lookahead: LitStr = input.parse()?;
			lookahead.value()
		} else {
			String::new()
		};

		input.parse::<Token![=]>()?;

		let target = if input.peek(token::Bracket) {
			let content;
			bracketed!(content in input);

			let target = content.parse_terminated::<_, Token![,]>(<LitStr as Parse>::parse)?;
			let target: Vec<String> = target.into_iter().map(|x| x.value()).collect();
			target
		} else {
			let target: LitStr = input.parse()?;
			let target = vec![target.value()];
			target
		};

		Ok(ConversionRow {
			source,
			lookahead,
			target,
		})
	}
}
