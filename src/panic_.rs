use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
pub fn main() {
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {panic!("error open file, e:{:?}", error)}
    // };
    //
    // 如果文件不存在错误，就创建
    let f = match f {
        Ok(file) => {file},
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(new_file) => {
                    println!("file not exist, crate new");
                    new_file
                },
                Err(create_e) => panic!("crate error:{:?}", create_e),
            },
            other => panic!("error opening file. {:?}", other),
        }
    };

    // unwrap : match表达式的一个快捷方法. 一般表示开发者认为不会出错，否则panic
    // 打开文件或报错:
    let f= File::open("hello.txt").unwrap();

    //expect: 在unwrap上增加错误信息
    let f = File::open("hello.txt").expect("无法打开文件");


    // 错误的传递
    fn read_username_from_file() -> Result<String,io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e)  // 返回错误
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),  // 返回读取到的字符串
            Err(e) => Err(e), // 返回错误
        }
    }

    // 使用？进行错误的传递
    // 在Result后面加?  来表示如果有错误就返回异常
    // 使用? 不会引起panic，会把错误传递给函数的调用者
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    //改为更简洁的链式调用
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    // ?运算符与main函数
    // main函数的返回值是(), 也可以是Result<T, E>
    fn main() -> Result<(), Box<dyn Error>> { // Box是trait对象，可以理解为：任何可能的错误类型
        let f = File::open("h.txt")?;
        Ok(())
    }

}