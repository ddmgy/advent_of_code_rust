use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    FnArg, Ident, ItemFn,
    LitInt, LitStr,
    PatType, ReturnType, Token,
};

mod kw {
    syn::custom_keyword!(year);
    syn::custom_keyword!(day);
    syn::custom_keyword!(part);
    syn::custom_keyword!(version);
    syn::custom_keyword!(separator);
}

struct AocEntry {
    year: usize,
    day: usize,
    part: aoc_common::Part,
    version: Option<String>,
    separator: Option<String>,
}

impl Parse for AocEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut year: Option<usize> = None;
        let mut day: Option<usize> = None;
        let mut part: Option<aoc_common::Part> = None;
        let mut version: Option<String> = None;
        let mut separator: Option<String> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::year) {
                input.parse::<kw::year>()?;
                input.parse::<Token![=]>()?;
                year = Some(input.parse::<LitInt>()?.base10_parse()?);
            } else if lookahead.peek(kw::day) {
                input.parse::<kw::day>()?;
                input.parse::<Token![=]>()?;
                day = Some(input.parse::<LitInt>()?.base10_parse()?);
            } else if lookahead.peek(kw::part) {
                input.parse::<kw::part>()?;
                input.parse::<Token![=]>()?;
                let p = input.parse::<LitStr>()?;
                match aoc_common::Part::from_str(&p.value()) {
                    Ok(p) => part = Some(p),
                    Err(_) => {
                        return Err(input.error(format!("expected A|B, found: {}", p.value())));
                    },
                }
            } else if lookahead.peek(kw::version) {
                input.parse::<kw::version>()?;
                input.parse::<Token![=]>()?;
                version = Some(input.parse::<LitStr>()?.value());
            } else if lookahead.peek(kw::separator) {
                input.parse::<kw::separator>()?;
                input.parse::<Token![=]>()?;
                separator = Some(input.parse::<LitStr>()?.value());
            } else {
                return Err(lookahead.error());
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AocEntry {
            year: year.expect("year must be supplied"),
            day: day.expect("day must be supplied"),
            part: part.expect("part must be supplied"),
            version,
            separator,
        })
    }
}

#[proc_macro_attribute]
#[proc_macro_error]
/// Create a runnable function for an Advent of Code solution,
/// with a defined name structure.
///
/// Attributes are key -> value pairs:
///
/// * `year` -> integer
/// * `day` -> integer
/// * `part` -> string (one of "A" or "B")
/// * `version` -> string (optional)
/// * `separator` -> string (optional)
///
/// with `integer` being equivalent to regex `\d+` and `string`
/// being equivalent to regex `"[a-zA-Z0-9_\s\\]+"`.
///
/// TODO: Enumerate allowed argument and return types.
pub fn aoc(attr: TokenStream, input: TokenStream) -> TokenStream {
    let AocEntry {
        year,
        day,
        part,
        version,
        separator,
    } = parse_macro_input!(attr as AocEntry);
    let part = part.to_string();

    let func = parse_macro_input!(input as ItemFn);
    let func_name = func.sig.ident.clone();
    let runner_func_name = Ident::new(
        &format!(
            "runner_y{year}_day{day:02}_part{part}_{}",
            version.clone().map_or_else(|| String::from("none"), |v| v.replace(" ", "_")),
        ),
        func.sig.ident.span(),
    );
    let sep = match separator {
        Some(sep) => quote!(Some(#sep)),
        None => quote!(None),
    };

    let inputs = match func.sig.inputs.first() {
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("Vec < & [u8] >") => {
            quote!((crate::input::parse_input_bytes(&crate::input::input_bytes(#year, #day)?, #sep.map(|c: char| c as u8))?))
        },
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& [& [u8]]") => {
            quote!((&crate::input::parse_input_bytes(&crate::input::input_bytes(#year, #day)?, #sep.map(|c: char| c as u8))?))
        },
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("Vec < & str >") => {
            quote!((crate::input::input_string(#year, #day)?.lines().collect()))
        },
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& [& str]") => {
            quote!((&crate::input::input_string(#year, #day)?.lines().collect::<Vec<_>>()))
        },
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& mut [& str]") => {
            quote!((&mut crate::input::input_string(#year, #day)?.lines().collect::<Vec<_>>()))
        },
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& str") => {
            quote!((&crate::input::input_string(#year, #day)?))
        },
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& [u8]") => {
            quote!((&crate::input::input_bytes(#year, #day)?))
        },
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("Vec <") => {
            quote!((crate::input::parse_input(&crate::input::input_string(#year, #day)?, #sep)?))
        },
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& mut [") => {
            quote!((&mut crate::input::parse_input(&crate::input::input_string(#year, #day)?, #sep)?))
        },
        Some(_) => {
            quote!((&crate::input::parse_input(&crate::input::input_string(#year, #day)?, #sep)?))
        },
        None => quote!(()),
    };

    let (call, ty) = match func.sig.output {
        ReturnType::Type(_, ref t) if t.to_token_stream().to_string().contains("Result < ") => {
            (quote!(#func_name #inputs), quote!(#t))
        },
        ReturnType::Type(_, ref t) if t.to_token_stream().to_string().contains("Option < ") => {
            let no_option = t
                .to_token_stream()
                .into_iter()
                .skip(1)
                .collect::<proc_macro2::TokenStream>();
            let version = version.map_or(quote!(None), |v| quote!(Some(#v)));

            (
                quote!(Ok(#func_name #inputs .ok_or(crate::error::Error::NoOutput(#year, #day, #part.to_string(), #version))?)),
                quote!(::eyre::Result #no_option),
            )
        },
        ReturnType::Type(_, ref t) => (quote!(Ok(#func_name #inputs)), quote!(::eyre::Result<#t>)),
        _ => abort!(func.sig, "AOC solution cannot return ()"),
    };

    quote! {
        #func

        #[allow(non_snake_case)]
        pub fn #runner_func_name() -> #ty {
            #call
        }
    }.into()
}
