use proc_macro::TokenStream;
use proc_macro2::*;
use quote::quote;
use syn::{parse::Parse, *};

#[proc_macro]
pub fn make_convert(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as ConversionTable);

	let name = input.name;

	let pairs = input.pairs.into_iter().map(|it| {
		let source = it.source;
		let target = it.target;
		quote! {
			if let Some(stripped) = input.strip_prefix(#source) {
				output.push_str(#target);
				input = stripped;
				continue;
			}
		}
	});
	let pairs = pairs.collect::<Vec<_>>();

	let tokens = quote! {
		fn #name(mut input: &'static str) -> String {
			let mut output = String::new();
			while input.len() > 0 {
				#( #pairs )*

				if let Some((index, _)) = input.char_indices().nth(1) {
					output.push_str(&input[..index]);
					input = &input[index..];
				} else {
					output.push_str(input);
					break;
				}
			}

			output
		}
	};

	tokens.into()
}

struct ConversionTable {
	pub name: Ident,
	pub pairs: Vec<ConversionPair>,
}

impl Parse for ConversionTable {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut table = ConversionTable {
			name: input.call(Ident::parse)?,
			pairs: Vec::new(),
		};

		input.parse::<Token![=>]>()?;

		let pairs = input.parse_terminated::<_, Token![,]>(ConversionPair::parse)?;
		table.pairs.extend(pairs);

		Ok(table)
	}
}

struct ConversionPair {
	pub source: String,
	pub target: String,
}

impl Parse for ConversionPair {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let source: LitStr = input.parse()?;
		let source = source.value();

		input.parse::<Token![=]>()?;

		let target: LitStr = input.parse()?;
		let target = target.value();

		Ok(ConversionPair { source, target })
	}
}
