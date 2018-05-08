use {quote, syn, utils};

pub struct Interface {
	name: String,
	constructor: Option<Signature>,
	items: Vec<Item>,
}

pub struct Event {
	pub name: syn::Ident,
	pub canonical: String,
	pub method_sig: syn::MethodSig,
	pub indexed: Vec<(syn::Pat, syn::Ty)>,
	pub data: Vec<(syn::Pat, syn::Ty)>,
}

#[derive(Clone)]
pub struct Signature {
	pub name: syn::Ident,
	pub canonical: String,
	pub method_sig: syn::MethodSig,
	pub hash: u32,
	pub arguments: Vec<(syn::Pat, syn::Ty)>,
	pub return_types: Vec<syn::Ty>,
	pub is_constant: bool,
	pub is_payable: bool,
}

pub enum Item {
	Signature(Signature),
	Event(Event),
	Other(syn::TraitItem),
}

impl Item {
	fn name(&self) -> Option<&syn::Ident> {
		use Item::*;
		match *self {
			Signature(ref sig) => Some(&sig.name),
			Event(ref event) => Some(&event.name),
			Other(_) => None,
		}
	}
}

impl Interface {
	pub fn from_item(source: syn::Item) -> Self {
		let trait_items = match source.node {
			syn::ItemKind::Trait(_, _, _, items) => items,
			_ => { panic!("Dispatch trait can work with trait declarations only!"); }
		};

		let (constructor_items, other_items) = trait_items
			.into_iter()
			.map(Item::from_trait_item)
			.partition::<Vec<Item>, _>(|item| item.name().map_or(false, |ident| ident.as_ref() == "constructor"));

		Interface {
			constructor: constructor_items
				.into_iter()
				.next()
				.map(|item| match item { Item::Signature(sig) => sig, _ => panic!("constructor must be function!") }),
			name: source.ident.as_ref().to_string(),
			items: other_items,
		}
	}

	pub fn items(&self) -> &[Item] {
		&self.items
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn constructor(&self) -> Option<&Signature> {
		self.constructor.as_ref()
	}
}

fn into_signature(ident: syn::Ident, method_sig: syn::MethodSig, is_constant: bool, is_payable: bool) -> Signature {
	let arguments: Vec<(syn::Pat, syn::Ty)> = utils::iter_signature(&method_sig).collect();
	let canonical = utils::canonical(&ident, &method_sig);
	let return_types: Vec<syn::Ty> = match method_sig.decl.output {
		syn::FunctionRetTy::Default => Vec::new(),
		syn::FunctionRetTy::Ty(syn::Ty::Tup(ref tys)) => tys.iter().map(|ty|ty.clone()).collect(),
		syn::FunctionRetTy::Ty(ref ty) => vec![ty.clone()],
	};
	let hash = utils::hash(&canonical);

	Signature {
		name: ident,
		arguments: arguments,
		method_sig: method_sig,
		canonical: canonical,
		hash: hash,
		return_types: return_types,
		is_constant: is_constant,
		is_payable: is_payable,
	}
}

fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
	attrs.iter().any(|a| match a.value {
		syn::MetaItem::Word(ref ident) => ident.as_ref() == name,
		_ => false
	})
}

impl Item {
	pub fn from_trait_item(source: syn::TraitItem) -> Self {
		let ident = source.ident;
		let node = source.node;
		let attrs = source.attrs;
		match node {
			syn::TraitItemKind::Method(method_sig, None) => {
				if has_attribute(&attrs, "event") {
					assert!(ident.as_ref() != "constructor", "Constructor can't be event");
					let (indexed, non_indexed) = utils::iter_signature(&method_sig)
						.partition(|&(ref pat, _)| quote! { #pat }.to_string().starts_with("indexed_"));
					let canonical = utils::canonical(&ident, &method_sig);

					let event = Event {
						name: ident,
						canonical: canonical,
						indexed: indexed,
						data: non_indexed,
						method_sig: method_sig,
					};

					Item::Event(event)
				} else {
					let constant = has_attribute(&attrs, "constant");
					let payable = has_attribute(&attrs, "payable");
					assert!(!(constant && payable),
						format!("Method {} cannot be constant and payable at the same time", ident.to_string()
					));
					assert!(!(ident.as_ref() == "constructor" && constant), "Constructor can't be constant");
					Item::Signature(
						into_signature(ident, method_sig, constant, payable)
					)
				}
			},
			_ => {
				Item::Other(syn::TraitItem { attrs: attrs, node: node, ident: ident })
			}
		}
	}
}

impl quote::ToTokens for Item {
	fn to_tokens(&self, tokens: &mut quote::Tokens) {
		match *self {
			Item::Event(ref event) => {
				let method_sig = &event.method_sig;
				let name = &event.name;
				tokens.append_all(&[
					utils::produce_signature(
						name,
						method_sig,
						{
							let keccak = utils::keccak(&event.canonical);
							let hash_bytes = keccak.as_ref().iter().map(|b| {
								syn::Lit::Int(*b as u64, syn::IntTy::U8)
							});

							let indexed_pats = event.indexed.iter()
								.map(|&(ref pat, _)| pat);

							let data_pats = event.data.iter()
								.map(|&(ref pat, _)| pat);

							let data_pats_count_lit = syn::Lit::Int(event.data.len() as u64, syn::IntTy::Usize);

							quote! {
								let topics = &[
									[#(#hash_bytes),*].into(),
									#(::pwasm_abi::eth::AsLog::as_log(&#indexed_pats)),*
								];

								let mut sink = ::pwasm_abi::eth::Sink::new(#data_pats_count_lit);
								#(sink.push(#data_pats));*;
								let payload = sink.finalize_panicking();

								::pwasm_ethereum::log(topics, &payload);
							}
						}
					)
				]);
			},
			Item::Signature(ref signature) => {
				tokens.append_all(&[syn::TraitItem {
					ident: signature.name.clone(),
					attrs: Vec::new(),
					node: syn::TraitItemKind::Method(
						signature.method_sig.clone(),
						None,
					),
				}]);
			},
			Item::Other(ref item) => {
				tokens.append_all(&[item]);
			}
		}
	}
}

impl quote::ToTokens for Interface {
	fn to_tokens(&self, tokens: &mut quote::Tokens) {
		let trait_ident: syn::Ident = self.name.clone().into();

		let items = &self.items;
		let constructor_item = self.constructor().map(|c| Item::Signature(c.clone()));
		tokens.append(
			quote! (
				pub trait #trait_ident {
					#constructor_item
					#(#items)*
				}
			)
		);
	}
}
