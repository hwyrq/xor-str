
//! # XOR 编译期加密字符串并且运行时自动解密
//!
//! # XOR encrypts strings at compile time and decrypts them automatically at run time
//!
//! 为什么要用这个？
//!
//! 原因：项目编译成机器码后，数据库密码或者sql语句等敏感信息，是暴露在机器码中的，
//!
//! 如果通过gbk编码强行打开该exe文件，通过搜索"mysql"关键字，即可看到数据库链接信息，包括您的密码
//!
//! 通过使用该依赖，就可以隐藏重要文本数据
//!
//! Why use this?
//!
//! Reason: After the project is compiled into machine code, sensitive information such as database passwords or sql statements are exposed to the machine code
//!
//! If you force open the exe file with gbk encoding, you can see the database link information, including your password, by searching for the keyword "mysql"
//!
//! By using this dependency, you can hide important text data
//! # 使用方式 how to use
//! ```
//! [dependencies]
//!  xor-str = "*"
//!
//! use xor_str::xor;
//! use xor_str::encode;
//! use xor_str::decode;
//! fn main() {
//!     println!("{}",xor!("Hello, world!"));
//! }
//! ```


///具体的xor加密代码都在这里实现
use std::ops::Index;
use syn::{Expr, Lit, parse_macro_input, Token};
use syn::__private::{ Span, TokenStream, ToTokens};
use syn::__private::quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;

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
