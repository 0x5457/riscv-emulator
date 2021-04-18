use proc_macro2::Span;
use syn::{DeriveInput, Error, Ident, NestedMeta, Result};

pub fn expand(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let match_code = parse_code_attr(ast, "match_code")?;
    let mask = parse_code_attr(ast, "mask")?;
    let format = parse_format_attr(ast)?;
    let ident_fn = format_ident!(
        "{}_FN",
        Ident::new(&name.to_string().to_uppercase(), name.span())
    );

    let name_str = name.to_string().to_lowercase();
    Ok(quote!(
        impl_format!(#name, #format);
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", #name_str)
            }
        }

        #[distributed_slice(INSN_SLICE)]
        static #ident_fn: fn() -> (u32, u32, fn(u32) -> Insn) = || -> (u32, u32, fn(u32) -> Insn) {
            (#match_code, #mask, |code: u32| { Insn::new(#name{code: code}) })
        };
    ))
}

fn parse_code_attr(ast: &DeriveInput, name: &str) -> Result<u32> {
    let attr = parse_attr(ast, name)?;

    match attr.attr {
        NestedMeta::Lit(syn::Lit::Int(raw)) => Ok(raw.base10_parse()?),
        _ => Err(Error::new(
            attr.ident.span(),
            format!("\"{}\" is expected a int value", name),
        )),
    }
}

fn parse_format_attr(ast: &DeriveInput) -> Result<Ident> {
    let attr = parse_attr(ast, "format")?;
    match attr.attr {
        NestedMeta::Meta(syn::Meta::Path(path)) => match path.get_ident() {
            Some(ident) => Ok(ident.clone()),
            None => Err(Error::new(
                attr.ident.span(),
                format!("\"{}\" is expected as Ident", "format"),
            )),
        },
        _ => Err(Error::new(
            attr.ident.span(),
            format!("\"{}\" is expected as Ident", "format"),
        )),
    }
}

struct Attr {
    ident: Ident,
    attr: NestedMeta,
}

impl Attr {
    fn new(ident: Ident, attr: NestedMeta) -> Self {
        Attr { ident, attr }
    }
}

fn parse_attr(ast: &DeriveInput, name: &str) -> Result<Attr> {
    if let Some(attr) = ast
        .attrs
        .iter()
        .find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == name)
    {
        let meta = attr.parse_meta()?;
        if let syn::Meta::List(ref nested_meta) = meta {
            if nested_meta.nested.len() == 1 {
                Ok(Attr::new(
                    attr.path.segments[0].ident.clone(),
                    nested_meta.nested[0].clone(),
                ))
            } else {
                Err(Error::new(
                    attr.path.segments[0].ident.span(),
                    format!("\"{}\" is expected to be a single value", name),
                ))
            }
        } else {
            Err(Error::new(
                attr.path.segments[0].ident.span(),
                format!("\"{}\" is expected to be a single value", name),
            ))
        }
    } else {
        Err(Error::new(
            Span::call_site(),
            format!("attr \"{}\" missed", name),
        ))
    }
}
