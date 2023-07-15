use form::ToTransform;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

mod form;

#[proc_macro]
pub fn sgr(input: TokenStream) -> TokenStream {
    match parse_tokens(input) {
        Ok(s) => tokenize(&s),
        Err(error_tokens) => error_tokens,
    }
}
fn parse_tokens(input: TokenStream) -> Result<String, TokenStream> {
    match input.into_iter().next() {
        // return the source TokenTree in the case there is any error
        // any error should be picked up by the rust compiler,
        // as it would be string literal error
        Some(source) => match source {
            TokenTree::Literal(s) => match correct_string(&s.to_string()) {
                Some(s) => Ok(parse_string(&mut s.chars())),
                None => Err(str_literal_err(s.span())),
            },
            _ => Err(str_literal_err(source.span())),
        },
        None => Err(str_literal_err(Span::mixed_site())),
    }
}
fn correct_string<'a>(string: &'a String) -> Option<&'a str> {
    string.strip_prefix('"')?.strip_suffix('"')
}
fn parse_string(chars: &mut impl Iterator<Item = char>) -> String {
    chars.transform(parse_chars).collect()
}
fn parse_chars(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    fn inner(next: char, chars: &mut impl Iterator<Item = char>) -> Option<char> {
        match next {
            '\\' => match chars.next()? {
                //quote escapes
                '\'' => Some('\''),
                '"' => Some('"'),
                //ascii escapes
                'x' => parse_7bit(chars),
                'n' => Some('\n'),
                'r' => Some('\r'),
                't' => Some('\t'),
                '\\' => Some('\\'),
                '\0' => Some('\0'),
                //unicode escape
                'u' => parse_24bit(chars),
                //whitespace ignore
                '\n' => {
                    for c in chars.by_ref() {
                        let (' ' | '\n' | '\r' | '\t') = c else {
                            return inner(c, chars)
                        };
                    }
                    None // end of string reached
                }
                _ => None, // invalid char
            },
            '{' => match chars.next()? {
                '{' => Some('{'),
                c => Some(c),
            },
            '}' => match chars.next()? {
                '}' => Some('}'),
                c => Some(c),
            },
            c => Some(c),
        }
    }
    inner(chars.next()?, chars)
}
fn parse_7bit(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    let mut src = String::with_capacity(2);
    src.push(chars.next()?);
    src.push(chars.next()?);

    char::from_u32(u32::from_str_radix(&src, 16).ok()?)
}
fn parse_24bit(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    chars.next()?;
    let src: String = chars.take_while(|&c| c != '}').collect();

    char::from_u32(u32::from_str_radix(&src, 16).ok()?)
}
fn tokenize(s: &str) -> TokenStream {
    [TokenTree::Literal(Literal::string(s))]
        .into_iter()
        .collect()
}
fn str_literal_err(span: Span) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::string(
                "first item must be a string literal\ncannot be raw and/or byte string",
            ))]
            .into_iter()
            .collect(),
        )),
    ]
    .into_iter()
    .collect()
}
