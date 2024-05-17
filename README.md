 # XOR 编译期加密字符串并且运行时自动解密
 # XOR encrypts strings at compile time and decrypts them automatically at run time

 为什么要用这个？

 原因：项目编译成机器码后，数据库密码或者sql语句等敏感信息，是暴露在机器码中的，

 如果通过gbk编码强行打开该exe文件，通过搜索"mysql"关键字，即可看到数据库链接信息，包括您的密码

 通过使用该依赖，就可以隐藏重要文本数据

 Why use this?

 Reason: After the project is compiled into machine code, sensitive information such as database passwords or sql statements are exposed to the machine code

 If you force open the exe file with gbk encoding, you can see the database link information, including your password, by searching for the keyword "mysql"

 By using this dependency, you can hide important text data

 # 使用方式 how to use
 ```
 use xor_str::xor;
 let value = xor!("你好 世界！");
 println!("{}",value);
