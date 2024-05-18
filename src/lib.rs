
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
pub use xor_str_encode::encode;

///xor宏
/// 在这里加密并解密，加密是过程宏，编译期间执行

#[macro_export]
macro_rules!  xor {
    ($str:literal) => {
      decode(encode!($str))
    };
}

///具体的xor解密逻辑
pub fn decode(str: &[u8]) -> String {
    let len = str.len();
    let mut decrypted = Vec::with_capacity(len);
    for (i, v) in str.iter().enumerate() {
        decrypted.push(v ^ (len + i ) as u8);
    }
    String::from_utf8(decrypted).unwrap()
}

///创建并通过xor方式加密字符串
#[cfg(test)]
mod tests {
    use crate::decode;
    use crate::encode;
    #[test]
    fn it_works() {
        println!("{}", xor!(br#"hello word!"#));
        println!("{}", xor!("你好 世界！"));
    }
}