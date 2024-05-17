///具体的xor加密代码都在这里实现
use std::ops::Index;
use syn::{Expr, Lit, parse_macro_input, Token};
use syn::__private::{IntoSpans, Span, TokenStream, ToTokens};
use syn::__private::quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

#[proc_macro]
pub fn encode(input: TokenStream) -> TokenStream {
    let punctuated = Punctuated::<Expr, Token![,]>::parse_separated_nonempty.parse(input).unwrap();
    let mut str = "".to_string();
    let token_stream = TokenStream::from(punctuated.index(0).into_token_stream());
    let string = parse_macro_input!(token_stream as Lit);
    match string {
        Lit::Str(lit) => {
            str = lit.value();
        },
        Lit::ByteStr(lit) => {
            str = String::from_utf8(Vec::from(lit.value())).unwrap();
        }
        _ => {}
    };
    let key = str.len();

    let bytes = str.bytes().collect::<Vec<_>>();
    let mut encrypted = Vec::with_capacity(bytes.len());
    for (i, v) in bytes.iter().enumerate() {
        encrypted.push(v ^ (key + i ) as u8);
    }
    let lit = syn::LitByteStr::new(encrypted.as_slice(), Span::call_site());
    return quote! { #lit  }.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
